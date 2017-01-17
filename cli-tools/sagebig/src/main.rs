extern crate libbig;
use libbig::{BigArchive, ReadError};

extern crate clap;
use clap::{Arg, App, AppSettings, SubCommand};

use std::fs;
use std::path::Path;

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
        .subcommand(SubCommand::with_name("extract")
            .about("Extract all files from an archive")
            .version("0.1.0")
            .author("Alexandre Oliveira <alex@layerbnc.org>")
            .arg(Arg::with_name("archive_path")
                .value_name("archive_path")
                .required(true)
                .index(1))
            .arg(Arg::with_name("output_dir")
                .value_name("output_dir")
                .required(true)
                .index(2))
            .arg(Arg::with_name("verbose")
                .short("v")))
        .get_matches();

    let res = match matches.subcommand() {
        ("list", Some(args)) => cmd_list(args),
        ("search", Some(args)) => cmd_search(args),
        ("contains", Some(args)) => cmd_contains(args),
        ("extract", Some(args)) => cmd_extract(args),
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

fn cmd_extract(args: &clap::ArgMatches) -> Result<(), ReadError> {
    let big_path = args.value_of("archive_path").unwrap();
    let archive = try!(BigArchive::new_from_path(&big_path));
    let output_dir = Path::new(args.value_of("output_dir").unwrap());
    let verbose = args.is_present("verbose");

    // Create output dir if it doesn't exist
    if !output_dir.exists() {
        try!(fs::create_dir(&output_dir));

        if verbose {
            println!("Created output_dir `{:?}`", &output_dir.display());
        }
    }

    let output_dir = output_dir.canonicalize().unwrap();

    for name in archive.get_all_entry_names() {
        let entry = archive.get_entry(name)
            .expect(&format!("Failed to read known entry {} from {}", name, big_path));

        let file_path = output_dir.join(Path::new(&entry.name));
        let file_dirpath = match file_path.parent() {
            Some(x) => x,
            None => panic!(format!("Invalid file path detected: {}", &file_path.display()))
        };

        if !file_dirpath.exists() {
            try!(fs::DirBuilder::new().recursive(true).create(&file_dirpath));
            if verbose {
                println!("Created path `{}`", &file_dirpath.display());
            }
        }
    }

    Ok(())
}
