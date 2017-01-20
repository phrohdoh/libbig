use std::fs::{self, DirBuilder, File};
use std::io::Write;
use std::path::Path;

extern crate clap;
use clap::{Arg, App, AppSettings, SubCommand};

extern crate libbig;
use libbig::BigArchive;
use libbig::errors::{ExtractError, ReadError};

#[derive(Debug)]
enum CliError {
    Read(ReadError),
    Extract(ExtractError),
}

impl From<ReadError> for CliError {
    fn from(e: ReadError) -> Self {
        CliError::Read(e)
    }
}

impl From<ExtractError> for CliError {
    fn from(e: ExtractError) -> Self {
        CliError::Extract(e)
    }
}

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
            .about("Locate entries with names containing a given string")
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
            .about("Create a directory structure and extract files from an archive's hierarchy")
            .version("0.1.0")
            .author("Taryn Hill <taryn@phrohdoh.com>")
            .arg(Arg::with_name("archive_path")
                .value_name("archive_path")
                .required(true)
                .index(1))
            .arg(Arg::with_name("output_dir")
                .value_name("output_dir")
                .required(true)
                .index(2))
            .arg(Arg::with_name("verbose")
                .short("v")
                .long("verbose")))
        .subcommand(SubCommand::with_name("info")
            .about("Prints out information about an archive")
            .version("0.1.0")
            .author("Taryn Hill <taryn@phrohdoh.com>")
            .arg(Arg::with_name("archive_path")
                .value_name("archive_path")
                .required(true)
                .index(1)))
        .get_matches();

    let res: Result<(), CliError> = match matches.subcommand() {
        ("list", Some(args)) => cmd_list(args).map_err(CliError::Read),
        ("search", Some(args)) => cmd_search(args).map_err(CliError::Read),
        ("contains", Some(args)) => cmd_contains(args).map_err(CliError::Read),
        ("extract", Some(args)) => cmd_extract(args),
        ("info", Some(args)) => cmd_info(args).map_err(CliError::Read),
        _ => unreachable!(),
    };

    let code = if let Some(e) = res.err() {
        println!("CliError: {:?}", e);
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

fn cmd_extract(args: &clap::ArgMatches) -> Result<(), CliError> {
    let path = args.value_of("archive_path").unwrap();
    let output_dir = Path::new(args.value_of("output_dir").unwrap());
    let is_verbose = args.is_present("verbose");
    let archive = try!(BigArchive::new_from_path(&path));

    if !output_dir.exists() {
        try!(fs::create_dir(&output_dir)
            .map_err(|e| CliError::Extract(ExtractError::StdIoError(e))));
    }

    let output_dir = try!(output_dir.canonicalize().map_err(ExtractError::StdIoError));

    let names = archive.get_all_entry_names().map(|k| k.to_owned()).collect::<Vec<_>>();
    for name in names {
        let entry = archive.get_entry(&name)
            .expect(&format!("Failed to read known entry {} from {}", name, path));

        let file_path = output_dir.join(Path::new(&entry.name));
        let file_path_dir = try!(file_path.parent()
            .ok_or(CliError::Extract(ExtractError::InvalidPath(file_path.clone()))));

        if !file_path_dir.exists() {
            try!(DirBuilder::new()
                .recursive(true)
                .create(&file_path_dir)
                .map_err(ExtractError::StdIoError));
        }

        if let Some(entry_data) = archive.read_entry(&name) {
            let mut file = try!(File::create(file_path.clone()).map_err(ExtractError::StdIoError));
            try!(file.write_all(entry_data.as_slice())
                .map_err(|e| CliError::Extract(ExtractError::StdIoError(e))));

            if is_verbose {
                println!("Wrote to {}", file_path.display());
            }
        }
    }

    Ok(())
}

fn cmd_info(args: &clap::ArgMatches) -> Result<(), ReadError> {
    let path = args.value_of("archive_path").unwrap();
    let archive = try!(BigArchive::new_from_path(&path));

    let mut header_len = 4 + 4 + 4 + 4;
    for name in archive.get_all_entry_names().map(|k| k.to_owned()).collect::<Vec<_>>() {
        let entry = archive.get_entry(&name)
            .expect(&format!("Failed to read known entry {} from {}", name, path));

        header_len += 4 + 4 + entry.name_len();
    }

    println!("Format: {:?}", archive.format);
    println!("Total length: {}", archive.size);
    println!("Header length: {}", header_len);
    println!("Entries: {}", archive.entry_count());

    Ok(())
}
