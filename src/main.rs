use microlp::{ComparisonOp, OptimizationDirection, Problem};

fn main() {
    let berserkers_greaves = Item {
        name: "berserkers",
        cost: 1100,
        ad: 0,
        ap: 0,
        asp: 25,
        limit: 10,
    };
    let ginsoos = Item {
        name: "ginsoos",
        cost: 3000,
        ad: 30,
        ap: 30,
        asp: 25,
        limit: 2,
    };
    let statikk = Item {
        name: "statikk",
        cost: 2700,
        ad: 45,
        ap: 10,
        asp: 30,
        limit: 10,
    };

    let items = [berserkers_greaves, ginsoos, statikk];
    microlp(items);
}

fn microlp<const N: usize>(items: [Item; N]) {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let vars = items.map(|item| {
        (
            problem.add_integer_var(item.cost as _, (0, item.limit as _)),
            item,
        )
    });

    // TODO take into account that requirements scale by level
    let ad_constraint = vars.map(|(var, item)| (var, item.ad as f64));
    problem.add_constraint(ad_constraint, ComparisonOp::Ge, 100.0);
    let ap_constraint = vars.map(|(var, item)| (var, item.ap as f64));
    problem.add_constraint(ap_constraint, ComparisonOp::Ge, 100.0);
    let asp_constraint = vars.map(|(var, item)| (var, item.asp as f64));
    problem.add_constraint(asp_constraint, ComparisonOp::Ge, 100.0);

    let solution = problem.solve().unwrap();

    println!("best build");
    println!("total cost: {}", solution.objective());
    for (var, item) in vars {
        println!("- {}: {}", item.name, solution.var_value_rounded(var));
    }
}

#[derive(Clone, Copy)]
struct Item {
    name: &'static str,
    cost: u16,
    ad: u16,
    ap: u16,
    asp: u16,
    limit: u16,
}
