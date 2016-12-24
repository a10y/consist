//! Tests for HashRing
extern crate consist;

use consist::HashRing;

#[derive(Debug, Hash, Clone)]
struct Person(&'static str, u8);

#[test]
fn hash_ring() {
    let mut ring = HashRing::<Person>::new();
    let bucket = Person("John Smith", 21);

    // Test adding a bucket and getting an item in the bucket
    ring.add_bucket(bucket.clone());
    match ring.get_bucket(&4) {
        None => assert!(false),
        Some(&Person(name, _)) => assert!(name == "John Smith"),
    }

    // After removal, nothing should hash to the bucket
    ring.remove_bucket(&bucket);
    match ring.get_bucket(&4) {
        None => {}
        Some(..) => assert!(false),
    };
}
