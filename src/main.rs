use std::path::Path;

use ddragon::parse_item_json;
use items::ddragon_to_items;
use microlp::{ComparisonOp, OptimizationDirection, Problem, Solution, Variable};

use crate::items::Item;

mod ddragon;
mod items;

fn main() {
    let items = parse_item_json(Path::new("./item.json")).unwrap();
    let mut items = ddragon_to_items(&items);

    println!(
        "Found {} items that contribute to desired stats",
        items.len()
    );

    // TODO find the items by name and not by id

    // nashor's tooth makes this slow as fuck
    items.remove(51);

    // Kraken Slayer
    items.remove(102);

    println!("Ignorning Nashor's Tooth and Kraken Slayer");

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
