use clap::{App, Arg};

fn main() {
    let matches = App::new("The Sourdough Calculator")
        .version("0.1.0")
        .author("Philip Karlsson Gisslow <ripxorip@gmail.com>")
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
    println!("{:?}", matches);
}
