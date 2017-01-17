# `libbig`

A library to read BIG archives (from EA's SAGE engine [BFME, Generals, etc])

### License

MIT

### Inspiration

https://github.com/feliwir/libbig

### Testing

```
$ cargo test
```

### Running

```
$ cd cli-tools/sagebig
$ cargo run -- list ../../test.big

# or

$ cargo run -- search ../../test.big art
```

`list` should result in something similar to:

```
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

`search` will yield:

```
BigEntry {
    offset: 69,
    size: 7,
    name: "art/image.txt"
}
```