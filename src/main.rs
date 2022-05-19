mod weather;

use clap::{arg, Command};

fn cli() -> Command<'static> {
    Command::new("mm")
        .about("mm tools")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("weather")
                .about("天气预报")
                .arg(arg!([CITY] "The city to query"))
                .arg_required_else_help(false)
        )
}


fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("weather", sub_matches)) => {
            if let Some(city) = sub_matches.value_of("CITY") {
                println!("Query weather of city: {}", city);
            } else {
                if let Err(err) = weather::get_weather("101190101") {
                    println!("Query weather failed: {}", err.to_string());
                }
            }
        }
        _ => unreachable!(),
    }
}
