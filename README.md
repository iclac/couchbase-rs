# Couchbase Rust Client [![crates-io][crates-io-image]][crates-io-url] [![docs][docs-image]][docs-url]

[crates-io-image]: https://img.shields.io/crates/v/couchbase.svg
[crates-io-url]: https://crates.io/crates/couchbase
[docs-image]: https://docs.rs/couchbase/badge.svg
[docs-url]: https://docs.rs/couchbase/

A fully asynchronous [Couchbase Server](http://couchbase.com/) [Rust](https://www.rust-lang.org)
Client based on [libcouchbase](https://github.com/couchbase/libcouchbase).

> Important: This library is a work in progress and the API is subject to change (and it is not officially supported by Couchbase, Inc. at this point!).

# Usage

In your `Cargo.toml` use:

```toml
[dependencies]
couchbase = "0.4"
```

Minimum rust version is **1.19**, since it supports unions which are needed for the `couchbase-sys` binding.

## Building
Since this crate is built on the `libcouchbase` C library, we need to link (and potentially build) it. By default, the underlying `couchbase-sys` crate will use `pkg-config` to pick it up and if its not found it will try to build it via `cmake`. Once build/found, it will try to match it up to a stored binding for the used version. If none is found it will fail, but you can enable the `generate-binding` feature to build the binding on demand.

## Examples

Run with `cargo run --example=helloworld`.

```rust
extern crate couchbase;
extern crate futures;

use couchbase::{Cluster, Document};
use couchbase::document::BinaryDocument;
use futures::Future;

/// A very simple example which connects to the `default` bucket and writes and loads
/// a document.
fn main() {
    // Initialize the Cluster
    let mut cluster = Cluster::new("localhost").expect("Could not initialize Cluster");

    // If you auth with 5.0 / RBAC, use this:
    cluster.authenticate("Administrator", "password");

    // Open the travel-sample bucket
    let bucket = cluster
        .open_bucket("default", None)
        .expect("Could not open Bucket");

    // Create a document and store it in the bucket
    let document = BinaryDocument::create("hello", None, Some("abc".as_bytes().to_owned()), None);
    println!(
        "Wrote Document {:?}",
        bucket.upsert(document).wait().expect("Upsert failed!")
    );

    // Load the previously written document and print it out
    let document: BinaryDocument = bucket.get("hello").wait().expect("Could not load Document");
    println!("Found Document {:?}", document);
}
```
