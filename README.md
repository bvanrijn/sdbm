# sdbm

<p align="center">
<a href="https://crates.io/crates/sdbm"><img src="https://img.shields.io/crates/v/sdbm.svg?style=flat" alt="View on Crates.io" /></a> <a href="https://docs.rs/sdbm"><img src="https://docs.rs/sdbm/badge.svg" alt="Documentation" /></a> <a href="https://travis-ci.com/bvanrijn/sdbm"><img src="https://travis-ci.com/bvanrijn/sdbm.svg?branch=master" alt="Build Status" /></a>
</p>

<hr />

SDBM is a non-cryptographic hashing algorithm.

## Usage

To use it, simply add this to your Cargo.toml:

```toml
[dependencies]
sdbm = "*"
```

Using it is as simple as:

```rust
fn main() {
    dbg!(sdbm::sdbm_hash("Hello, Rustaceans!"));
}
```

This will print:

```rust
[src/main.rs:2] sdbm::sdbm_hash("Hello, Rustaceans!") = 98379892
```
