# `libbig`

A library to read BIG archives (from EA's SAGE engine [BFME, Generals, etc])

### Warning

This is _not_ idiomatic Rust (there is _no_ error handling) and should be

refactored before being consumed.

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
$ cargo run -- test.big
```

This should result in something similar to:

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
