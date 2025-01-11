use std::path::Path;

use ddragon::parse_item_json;
use items::ddragon_to_items;
use microlp::{ComparisonOp, OptimizationDirection, Problem, Solution, Variable};

use crate::items::Item;

mod ddragon;
mod items {
    use crate::ddragon::Items;

    #[derive(Clone, Debug)]
    pub struct Item {
        pub name: String,
        pub id: String,

        pub cost: u16,

        pub ad: u16,
        pub ap: u16,
        pub asp: u16,
        pub ms: u16,

        pub limit: u16,
    }

    pub fn ddragon_to_items(items: &Items) -> Vec<Item> {
        // TODO get all items that are inStore

        // TODO limit comes from groups
        // so we need to find a group that contains that item and has a max ownable

        // ad -> FlatPhysicalDamageMod
        // ap -> FlatMagicDamageMod
        // asp -> FlatAttackSpeedMod
        // ms -> FlatMovementSpeedMod

        let mut result = vec![];
        for (id, item) in items.data.iter() {
            // filter out items
            let in_store = item.in_store.or(items.basic.in_store).unwrap_or(false);
            let purchasable = item.gold.purchasable;
            let in_rift = item.maps.get("11").cloned().unwrap_or(true);
            if !in_store || !purchasable || !in_rift {
                continue;
            }
            // TODO filter out items not buyable in the rift

            let stat = |name: &str| item.stats.get(name).cloned().unwrap_or(0.0);
            let percent_stat = |name: &str| (stat(name) * 100.0).floor() as u16;

            let ad = stat("FlatPhysicalDamageMod") as u16;
            let ap = stat("FlatMagicDamageMod") as u16;
            let asp = percent_stat("PercentAttackSpeedMod");
            let ms = percent_stat("PercentMovementSpeedMod");

            if ad == 0 && ap == 0 && asp == 0 {
                continue;
            }

            let cost = item.gold.total as u16;

            // max bc of item slots
            let max = 5 + item.stacks.unwrap_or(1);
            // check the list of groups to see if it contains this item
            let group = items.find_group_for_item(id, item);
            let limit = group
                .and_then(|group| group.max_group_ownable.parse().ok())
                // -1 is nothing
                .and_then(|limit| if limit == -1 { None } else { Some(limit) })
                .unwrap_or(max) as u16;

            result.push(Item {
                name: item.name.clone(),
                id: id.clone(),
                cost,
                ad,
                ap,
                asp,
                ms,
                limit,
            });
        }

        result.sort_unstable_by_key(|item| item.id.clone());

        result
    }
}

fn main() {
    let mut i = 0;

    let items = parse_item_json(Path::new("./item.json")).unwrap();
    let items = ddragon_to_items(&items)
        .into_iter()
        .filter(|item| {
            i += 1;

            let items = [
                "Blasting Wand",
                "B. F. Sword",
                "Needlessly Large Rod",
                "Phantom Dancer",
                "Hearthbound Axe",
                "Experimental Hexplate",
                "Nashor's Tooth",
                "Kraken Slayer",
            ];

            items.contains(&item.name.as_str()) || (0..40).contains(&i)
        })
        .collect::<Vec<_>>();

    println!(
        "Found {} items that contribute to desired stats",
        items.len()
    );

    microlp(&items);
}

fn microlp(items: &[Item]) {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let vars = items
        .iter()
        .map(|item| {
            (
                problem.add_integer_var(item.cost as _, (0, item.limit as _)),
                item,
            )
        })
        .collect::<Vec<_>>();

    // TODO limit number of items bc of slots
    let slots_constraints = vars.iter().map(|(var, _item)| (*var, 1.0));
    problem.add_constraint(slots_constraints, ComparisonOp::Le, 6.0);

    // TODO take into account that requirements scale by level
    let ad_constraint = vars.iter().map(|(var, item)| (*var, item.ad as f64));
    problem.add_constraint(ad_constraint, ComparisonOp::Ge, 100.0);
    let ap_constraint = vars.iter().map(|(var, item)| (*var, item.ap as f64));
    problem.add_constraint(ap_constraint, ComparisonOp::Ge, 100.0);
    let asp_constraint = vars.iter().map(|(var, item)| (*var, item.asp as f64));
    problem.add_constraint(asp_constraint, ComparisonOp::Ge, 100.0);

    use std::time::Instant;
    let now = Instant::now();
    let solution = problem.solve().unwrap();
    let elapsed = now.elapsed();
    println!("Found solution in {:.2?}", elapsed);

    print_solution(&vars, &solution);

    // TODO we can add a new constraint that one of the items doesn't match
    // to get further solutions
}

fn print_solution(vars: &[(Variable, &Item)], solution: &Solution) {
    let cost = solution.objective();
    println!("-----");
    println!("Best build");
    println!("Total cost: {}", cost);
    println!();
    println!(
        "{:<6} {:<24} {:<6} {:<3} {:<3} {:<12} {:<3}",
        "count", "name", "cost", "ad", "ap", "attack speed", "movement speed"
    );

    let mut ad = 0;
    let mut ap = 0;
    let mut asp = 0;
    let mut ms = 0;

    for (var, item) in vars {
        let count = solution.var_value_rounded(*var);
        if count > 0.0 {
            println!(
                "{:<6} {:<24} {:<6} {:<3} {:<3} {:<12} {:<3}",
                count, item.name, item.cost, item.ad, item.ap, item.asp, item.ms
            );

            ad += item.ad;
            ap += item.ap;
            asp += item.asp;
            ms += item.ms;
        }
    }

    println!("-----");
    println!(
        "{:<6} {:<24} {:<6} {:<3} {:<3} {:<12} {:<3}",
        "total", "", cost, ad, ap, asp, ms
    );
}
