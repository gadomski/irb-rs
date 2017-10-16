# irb-rs

Reads .irb files from [InfraTec](http://www.infratec.eu/).
Without InfraTec's SDK, the most you can do is read text files created by the InfraTec software.
With their SDK, you can read the binary .irb files themselves, via the [irbacs-sys](https://github.com/gadomski/irbacs-sys) crate.

**This project is not created by InfraTec.
Please do not contact InfraTec with any questions or issues.**

## Using irbacs

To build with SDK support, use the "irbacs-sys" feature:

```bash
cargo build --features "irbacs-sys"
```

Because **libirbacs** is not thread-safe, tests with **irbacs-sys** need to be run single-threaded:

```bash
cargo test --features "irbacs-sys" -- --test-threads=1
```

## Command-line executable

If you enable both the "irbacs-sys" and "clap" features, you get a simple command-line executable, useful for querying irb files:

```
cargo run --all-features -- info data/image.irb
```
