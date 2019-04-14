extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn main() {
    let matches = parse_args();

    if let Some(ref matches) = matches.subcommand_matches("week") {
        let day = match matches.value_of("day") {
            Some(day) => day,
            None => {
                println!("day command requires day of week");
                return;
            }
        };

        // todo: write stupid library, stupid...
        println!("{} of week", day);
    }

    if let Some(ref matches) = matches.subcommand_matches("month") {
        let day = match matches.value_of("day") {
            Some(day) => day,
            None => {
                println!("month command requires day of week and ord");
                return;
            }
        };

        let ord = match matches.value_of("ord") {
            Some(ord) => ord,
            None => {
                println!("month command requires day of week and ord");
                return;
            }
        };

        // todo: do actual repeated date code here!
        println!("{} {}", day, ord);
    }

    if let Some(ref matches) = matches.subcommand_matches("year") {
        let day = match matches.value_of("day") {
            Some(day) => day,
            None => {
                println!("year command requires day of year");
                return;
            }
        };

        // todo: actual repeated date code here! ... No, probably not...
        println!("{} day of year", day);
    }
}

fn parse_args<'a>() -> ArgMatches<'a> {
    let arg_day = Arg::with_name("day")
        .short("d")
        .long("day")
        .takes_value(true);
    let arg_ord = Arg::with_name("ord")
        .short("o")
        .long("ord")
        .help("ord value of repeated date")
        .takes_value(true);
    let sub_month = SubCommand::with_name("month")
        .about("Allows creation of month-relative date streams.")
        .arg(arg_day.clone().help("day of week"))
        .arg(arg_ord);
    let sub_week = SubCommand::with_name("week")
        .about("Allows creation of week-relative date streams.")
        .arg(arg_day.clone())
        .help("day of week");
    let sub_year = SubCommand::with_name("year")
        .about("Allows immortals to sort out their calendars.")
        .arg(arg_day.clone().help("day of year"));
    // todo: add range control; we have to know how many dates to print
    App::new("reldate")
        .version("0.0.1")
        .author("J/A <archer884@gmail.com>")
        .about("Prints relative dates.")
        .subcommand(sub_month)
        .subcommand(sub_week)
        .subcommand(sub_year)
        .get_matches()
}
