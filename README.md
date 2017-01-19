# `libbig`

A library to read BIG archives (from EA's SAGE engine [BFME, Generals, etc])

### License

MIT

### Inspiration

https://github.com/feliwir/libbig

### Testing

```
$ make test # from the root
```

### Running

The executable tools live in the `cli-tools` directory.

For now only `sagebig` exists (todo: is there any reason any others would exist?).

```
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
    list        List all entries in an archive
    search      Locate entries with names containing a given string
    contains    Query an archive to determine if it contains an entry with a name
    extract     Create a directory structure and extract files from an archive's hierarchy
```

`cd cli-tools/sagebig` then:

```
$ cargo run -- list ../../test.big

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
$ cargo run -- search ../../test.big art

BigEntry {
    offset: 69,
    size: 7,
    name: "art/image.txt"
}
```