# hamberder

A JSON push parser I wrote over the weekend after finishing the book The Rust Programming Language and wanting some more practice. 

It works similar to SAX parsers in that it avoids creating a DOM for the JSON source and only emits a flat sequence of tags (e.g. `BeginObject`, `EndArray`, `ObjectKey`, `Number`, etc.).

In the `examples/` subdirectory you'll find a simple use-case of converting 100,000 rows of JSON data into INSERT statements (perhaps for migrating to a database). You can run it with

```sh
cargo run --example tosql --release
```
The input and output of the parser work with `std::sync::mpsc::channel()` and thus don't care where the original JSON data comes from and don't require waiting for it to be fully loaded before the parsing can begin (unless you use the convenience function `parse_file()`, which blocks in order to potentially return a file I/O error).
