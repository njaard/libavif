# libavif

[![crates.io](https://img.shields.io/crates/v/libavif.svg)](https://crates.io/crates/libavif)
[![Documentation](https://docs.rs/libavif/badge.svg)](https://docs.rs/libavif)
[![BSD-2-Clause licensed](https://img.shields.io/crates/l/libavif.svg)](LICENSE)
[![Rustc Version 1.40+](https://img.shields.io/badge/rustc-1.40+-lightgray.svg)](https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html)
[![CI](https://github.com/njaard/libavif-rs/workflows/CI/badge.svg)](https://github.com/njaard/libavif-rs/actions?query=workflow%3ACI)

Initial release of a high-level avif decoder.

This crate is not really usable. Until I determine
a useful high-level API, you may want to use the unsafe
API in [`libavif-sys`](https://crates.io/crates/libavif-sys)
or the utility functions for [`image`](https://crates.io/crates/image),
[`libavif-image`](https://crates.io/crates/libavif-image).


