[![Crates.io](https://img.shields.io/crates/d/consist.svg)]()
[![Circle CI](https://circleci.com/gh/andreweduffy/consist/tree/master.svg?style=svg)](https://circleci.com/gh/andreweduffy/consist/tree/master)

consist: Consistent Hashing in Rust
===================================

Consistent Hashing is a technique invented by David Karger and colleagues their 1997 paper 
[Consistent Hashing and Random Trees](https://www.akamai.com/es/es/multimedia/documents/technical-publication/consistent-hashing-and-random-trees-distributed-caching-protocols-for-relieving-hot-spots-on-the-world-wide-web-technical-publication.pdf).
The original use case was to create a hashing scheme for cache servers so that the cache could be safely sharded over
several nodes, which minimizing the amount of entries that would need to be shifted around when a node joins or leaves
the network.

consist is a **simple**, **zero-dependency** library implementing Consistent Hashing in Rust.

Usage
-------

Here is an example of usage of the library:

```rust
extern crate consist;

use consist::HashRing;

#[derive(Hash)]
struct Server(&'static str);

fn make_cache() {
    let ring = HashRing::<Server>::new();
    ring.add_bucket(Server("10.0.1.1"));
    ring.add_bucket(Server("10.0.1.2"));
    ring.add_bucket(Server("10.0.1.3"));

    // Search the cache
    let url: &str = ...
    let server = ring.get_bucket(str);
    // Send a redirect to the cache server.
}
```
