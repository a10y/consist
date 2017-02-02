[![Crates.io](https://img.shields.io/crates/v/consist.svg)](https://crates.io/crates/consist)
[![Travis CI](https://img.shields.io/travis/a10y/consist.svg)](https://travis-ci.org/a10y/consist/)

consist: Rust consistent hashing
================================

Consistent Hashing is a technique invented by David Karger and colleagues in their 1997 paper
[Consistent Hashing and Random Trees](https://www.akamai.com/es/es/multimedia/documents/technical-publication/consistent-hashing-and-random-trees-distributed-caching-protocols-for-relieving-hot-spots-on-the-world-wide-web-technical-publication.pdf).

consist is a **simple** library implementing a consistent hash ring. Hash rings are commonly used
for sharding in distributed systems, and have been implemented as part of
[Varnish](https://github.com/varnishcache/varnish-cache/blob/master/lib/libvmod_directors/shard_hash.c)
as well as in various client-side libraries for Redis, notably
[redis-rb](https://github.com/redis/redis-rb/blob/master/lib/redis/hash_ring.rb).

It uses the `btree_range` feature, thus **you need to build on nightly until its API is finalized.**

**Note: As of 24 Jan 2017, you need to be on at least the following versions:**

```
$ rustc --version
rustc 1.16.0-nightly (7821a9b99 2017-01-23)

$ cargo --version
cargo 0.17.0-nightly (2324c2b 2017-01-21)
```

Filing PRs and issues is welcome.

----
* [Crates.io](https://crates.io/crates/consist)
* [Documentation](https://docs.rs/consist/)
* [Usage Examples](https://github.com/a10y/consist/tree/master/examples)
