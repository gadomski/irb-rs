# irb-rs

Reads .irb files from [InfraTec](http://www.infratec.eu/).
Without InfraTec's SDK, the most you can do is read text files created by the InfraTec software.
With their SDK, you can read the binary .irb files themselves, via the [irbasc-sys](https://github.com/gadomski/irbasc-sys) crate.

**This project is not created by InfraTec.
Please do not contact Infratec with any questions or issues.**

## Using irbasc

To build with SDK support, use the "irbasc-sys" feature:

```bash
cargo build --features "irbasc-sys"
```

Because **libirbasc** is not thread-safe, tests with **irbasc-sys** need to be run single-threaded:

```bash
cargo test --features "irbasc-sys" -- --test-threads=1
```
