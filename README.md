# segappend

Append custom data by creating a new segment in compiled binary.

Only supports Mach-O 64-bit binaries for now (macOS, iOS, etc).

## Usage

While it is primarily meant to be used as a C library, a simple utility tool is also compiled
along with the library. See `include/segappend.h` for the C API.

There are also Rust bindings published on crates.io: [segappend](https://crates.io/crates/segappend).

## Build

```sh
# build library and utility tool
deno task build

# utility tool
deno task run <input-file> <segment-name> <segment-data-file> <output-file>
```

## Resources

- [Adding a Segment to an Existing macOS Mach-O Binary (Alexander O'Mara)](https://alexomara.com/blog/adding-a-segment-to-an-existing-macos-mach-o-binary/)

## License

Apache-2.0 License

Copyright (c) 2023, DjDeveloperr
