use std::path::Path;

use ddragon::parse_item_json;
use good_lp::{variable, variables, IntoAffineExpression, Solution as _, SolverModel};
use items::ddragon_to_items;

use crate::items::Item;

mod ddragon;
mod items;

fn main() {
    let items = parse_item_json(Path::new("./item.json")).unwrap();
    let items = ddragon_to_items(&items);

    println!(
        "Found {} items that contribute to desired stats",
        items.len()
    );

    good_lp(&items);
}

fn good_lp(items: &[Item]) {
    let mut vars = variables! {};

    let mut cost = 0.into_expression();

    let mut ad = 0.into_expression();
    let mut ap = 0.into_expression();
    let mut asp = 0.into_expression();
    // There are a total of 6 slots
    let mut slots = 0.into_expression();

    let items = items
        .iter()
        .map(|item| {
            let var = vars.add(variable().integer().min(0).max(item.limit));

            ad += var * item.ad;
            ap += var * item.ap;
            asp += var * item.asp;
            slots += var * 1;
            cost += var * item.cost;

            (var, item)
        })
        .collect::<Vec<_>>();

    use std::time::Instant;
    let now = Instant::now();

    let solution = vars
        .minimise(cost.clone())
        .using(good_lp::default_solver)
        .with(ad.geq(100))
        .with(ap.geq(100))
        .with(asp.geq(100))
        .with(slots.leq(6.0))
        .solve()
        .unwrap();

    let elapsed = now.elapsed();
    println!("Found solution in {:.2?}", elapsed);

    let cost = solution.eval(cost);
    let items = items
        .into_iter()
        .map(|(var, item)| {
            let count = solution.value(var).round();
            (count, item)
        })
        .collect::<Vec<_>>();
    print_solution(&items, cost);

    // TODO we can add a new constraint that one of the items doesn't match
    // to get further solutions
}

fn print_solution(vars: &[(f64, &Item)], cost: f64) {
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

    for (count, item) in vars {
        if *count > 0.0 {
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
