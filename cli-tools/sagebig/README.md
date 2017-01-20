# `sagebig`

A command-line interface tool used to test the functionality provided by `libbig`.

### License

MIT

### Information

```
$ sagebig help

sagebig 0.1.0
Taryn Hill <taryn@phrohdoh.com>
CLI for libbig

USAGE:
    sagebig [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    info        Prints out information about an archive
    list        List all entries in an archive
    search      Locate entries with names containing a given string
    contains    Query an archive to determine if it contains an entry with a name
    extract     Create a directory structure and extract files from an archive's hierarchy
```

You can also get help for individual commands:

```
$ sagebig help extract

sagebig-extract 0.1.0
Taryn Hill <taryn@phrohdoh.com>
Create a directory structure and extract files from an archive's hierarchy

USAGE:
    sagebig extract [FLAGS] <archive_path> <output_dir>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose

ARGS:
    <archive_path>
    <output_dir>
```

### Usage

```
# List all entries in an archive
$ sagebig test.big

BigEntry {
    offset: 76,
    size: 26,
    name: "data/test.ini"
}
BigEntry {
    offset: 69,
    size: 7,
    name: "art/image.txt"
}
```

```
# Searching for entries within an archive with names containing "art"
$ sagebig search test.big art

BigEntry {
    offset: 69,
    size: 7,
    name: "art/image.txt"
}
```

Try these yourself:

```
$ sagebig extract test.big --verbose
```

```
$ sagebig contains test.big image.bmp
```