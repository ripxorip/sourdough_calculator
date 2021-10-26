#[macro_use] extern crate prettytable;
use prettytable::Table;
use clap::{App, Arg};

fn main() {
    let matches = App::new("The Sourdough Calculator")
        .version("0.1.0")
        .author("Philip K. Gisslow <ripxorip@gmail.com>")
        .about("A CLI program used to calculate sourdough bread recipes")
        .arg(
            Arg::with_name("hydration")
                .short("h")
                .long("hydration")
                .takes_value(true)
                .required(true)
                .help("The desired dough hydration (percentage)"),
        )
        .arg(
            Arg::with_name("flour")
                .short("f")
                .long("flour")
                .takes_value(true)
                .required(true)
                .help("The amount of flour used (grams)"),
        )
        .arg(
            Arg::with_name("salt")
                .short("s")
                .long("salt")
                .takes_value(true)
                .required(true)
                .help("The desired amount of salt (percentage)"),
        )
        .arg(
            Arg::with_name("inoculation")
                .short("i")
                .long("inoculation")
                .takes_value(true)
                .required(true)
                .help("The dough inoculation (percentage)"),
        )
        .arg(
            Arg::with_name("starter_hydration")
                .short("x")
                .long("starter_hydration")
                .takes_value(true)
                .required(false)
                .default_value("50")
                .help("The hydration of the starter (percentage)"),
        )
        .get_matches();

    let inoculation = 0.01 * matches.value_of("inoculation").unwrap().parse::<f32>().unwrap();
    let starter_hydration = 0.01 * matches.value_of("starter_hydration").unwrap().parse::<f32>().unwrap();
    let salt = 0.01 * matches.value_of("salt").unwrap().parse::<f32>().unwrap();
    let flour = matches.value_of("flour").unwrap().parse::<f32>().unwrap();
    let hydration = 0.01 * matches.value_of("hydration").unwrap().parse::<f32>().unwrap();

    let res = sourdough_calculator::calculate_dough(sourdough_calculator::DoughData{inoculation, starter_hydration, salt, flour, hydration});
    let mut table = Table::new();
    table.add_row(row!["Flour (g)", "Water (g)", "Salt (g)", "Starter (g)", "Total Dough Weight (g)"]);
    table.add_row(row![format!("{:.2}", res.flour), format!("{:.2}", res.water), format!("{:.2}", res.salt), format!("{:.2}", res.starter), res.dough_weight]);
    table.printstd();
}
