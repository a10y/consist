//! consist: An implementation of consistent hashing in Rust.
//! The goal of consistent hashing is to partition entries in such a way that the addition or
//! removal of buckets minimizes the number of items that must be shifted between buckets, i.e. it
//! optimizes the rehashing stage that is usually needed for hash tables.
//! The algorithm was originally put forth by David Karger et al. in their 1997 paper
//! "Consistent Hashing and Random Trees".
#![feature(btree_range, collections_bound)]

use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::Bound::{Included, Excluded, Unbounded};

pub struct HashRing<B: Hash> {
    buckets: BTreeMap<u64, B>,
}

impl<B> HashRing<B>
    where B: Hash
{
    pub fn new() -> HashRing<B> {
        HashRing::<B> { buckets: BTreeMap::new() }
    }
    /// Adds the specified bucket to the hash ring.
    pub fn add_bucket(&mut self, bucket: B) {
        let hash_code = HashRing::<B>::get_hash_code(&bucket);
        self.buckets.insert(hash_code, bucket);
    }

    /// Finds the corresponding bucket for this item
    pub fn get_bucket<T>(&self, item: &T) -> Option<&B>
        where T: Hash
    {
        let hash_code = HashRing::<B>::get_hash_code(item);
        // If there are no buckets in the end of the circle from here to the end, we wrap around
        // and try from the beginning.
        let back_half = self.buckets.range(Included(&hash_code), Unbounded).next().map(|val| val.1);
        let front_half = self.buckets
                             .range(Included(&0), Excluded(&hash_code))
                             .next()
                             .map(|val| val.1);
        back_half.or(front_half)
    }

    fn get_hash_code<H>(value: &H) -> u64
        where H: Hash
    {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }
}
