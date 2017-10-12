# irb-rs

Two-crate workspace to read .irb files from [InfraTec](http://www.infratec.eu/).
You'll need the libraries from InfraTec for some of these codes to work.
The **irb** crate does include a text reader that can be used to read text files exported from InfraTec software.

**This project is not created by InfraTec.
Please do not contact Infratec with any questions or issues.**

### irbasc-sys

Sys-crate interface for the InfraTec irbasc library.
At this point, this interface will only work for the Linux64-bit library.
Make sure the header file, `irbacs_v2.h`, is on your include path, and that the `libarbacs_l64.so` is on your library and load library paths.

### irb

A Rust-friendly library that wraps **irbasc-sys** in a nice struct-base interface.

## Running tests

Because **libirbasc** is not thread-safe, any tests with **irbasc-sys** need to be run single-threaded:

```bash
cargo test -p irb -- --test-threads=1
```
