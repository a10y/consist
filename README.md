[![Crates.io](https://img.shields.io/crates/v/consist.svg)](https://crates.io/crates/consist)
[![Circle CI](https://circleci.com/gh/andreweduffy/consist/tree/master.svg?style=svg)](https://circleci.com/gh/andreweduffy/consist/tree/master)

consist: Rust consistent hashing
================================

Consistent Hashing is a technique invented by David Karger and colleagues their 1997 paper 
[Consistent Hashing and Random Trees](https://www.akamai.com/es/es/multimedia/documents/technical-publication/consistent-hashing-and-random-trees-distributed-caching-protocols-for-relieving-hot-spots-on-the-world-wide-web-technical-publication.pdf).

consist is a **simple** library implementing a consistent hash ring. Hash rings are commonly used
for sharding in distributed systems, and have been implemented as part of
[Varnish](https://github.com/varnishcache/varnish-cache/blob/master/lib/libvmod_directors/shard_hash.c)
as well as in various client-side libraries for Redis, notably
[redis-rb](https://github.com/redis/redis-rb/blob/master/lib/redis/hash_ring.rb).

Filing PRs and issues is welcome.

----
* [Crates.io](https://crates.io/crates/consist)
* [Documentation](https://docs.rs/consist/)
* [Usage Examples](https://github.com/andreweduffy/consist/tree/master/examples)
