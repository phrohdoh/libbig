extern crate libbig;
use libbig::{BigArchive, ReadError};

extern crate clap;
use clap::{Arg, App, AppSettings, SubCommand};

fn main() {
    let matches = App::new("sagebig")
        .version("0.1.0")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("CLI for libbig")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("list")
            .about("List all entries in an archive")
            .version("0.1.0")
            .author("Taryn Hill <taryn@phrohdoh.com>")
            .arg(Arg::with_name("archive_path")
                .value_name("archive_path")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("search")
            .about("Locate entries with names containing a string")
            .version("0.1.0")
            .author("Taryn Hill <taryn@phrohdoh.com>")
            .arg(Arg::with_name("archive_path")
                .value_name("archive_path")
                .required(true)
                .index(1))
            .arg(Arg::with_name("query")
                .value_name("query")
                .required(true)
                .index(2)))
        .subcommand(SubCommand::with_name("contains")
            .about("Query an archive to determine if it contains an entry with a name")
            .version("0.1.0")
            .author("Taryn Hill <taryn@phrohdoh.com>")
            .arg(Arg::with_name("archive_path")
                .value_name("archive_path")
                .required(true)
                .index(1))
            .arg(Arg::with_name("query")
                .value_name("query")
                .required(true)
                .index(2)))
        .get_matches();

    let res = match matches.subcommand() {
        ("list", Some(args)) => cmd_list(args),
        ("search", Some(args)) => cmd_search(args),
        ("contains", Some(args)) => cmd_contains(args),
        _ => unreachable!(),
    };

    let code = if let Some(e) = res.err() {
        println!("Error: {:?}", e);
        255
    } else {
        0
    };

    std::process::exit(code);
}

fn cmd_list(args: &clap::ArgMatches) -> Result<(), ReadError> {
    let path = args.value_of("archive_path").unwrap();
    let archive = try!(BigArchive::new_from_path(&path));

    for name in archive.get_all_entry_names() {
        let entry = archive.get_entry(name)
            .expect(&format!("Failed to read known entry {} from {}", name, path));
        println!("{:#?}", entry);
    }

    Ok(())
}

fn cmd_search(args: &clap::ArgMatches) -> Result<(), ReadError> {
    let path = args.value_of("archive_path").unwrap();
    let archive = try!(BigArchive::new_from_path(&path));
    let query = args.value_of("query").unwrap();

    for name in archive.get_all_entry_names().filter(|n| n.contains(query)) {
        let entry = archive.get_entry(name)
            .expect(&format!("Failed to read known entry {} from {}", name, path));
        println!("{:#?}", entry);
    }

    Ok(())
}

fn cmd_contains(args: &clap::ArgMatches) -> Result<(), ReadError> {
    let path = args.value_of("archive_path").unwrap();
    let archive = try!(BigArchive::new_from_path(&path));
    let query = args.value_of("query").unwrap();

    println!("{} contains {}: {}", path, query, archive.contains(query));

    Ok(())
}
