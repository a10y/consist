//! Tests for HashRing
extern crate consist;

use consist::HashRing;

#[derive(Debug, Hash)]
struct Person(&'static str, u8);

#[test]
fn hash_ring() {
    let mut ring = HashRing::<Person>::new();
    ring.add_bucket(Person("John Smith", 21));
    match ring.get_bucket(&4) {
        None => assert!(false),
        Some(&Person(name, _)) => assert!(name == "John Smith")
    }
}
