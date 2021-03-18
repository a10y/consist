//! consist: An implementation of consistent hashing in Rust.
//! The goal of consistent hashing is to partition entries in such a way that the addition or
//! removal of buckets minimizes the number of items that must be shifted between buckets, i.e. it
//! optimizes the rehashing stage that is usually needed for hash tables.
//! The algorithm was originally put forth by David Karger et al. in their 1997 paper
//! "Consistent Hashing and Random Trees".
//! As of version 0.3.0, we use CRC64 with ECMA polynomial, so that updating the rust version does
//! not change behavior.
#![feature(btree_range, collections_bound)]

extern crate crc;

use crc::crc64::{ECMA, Digest, Hasher64};

use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::ops::Bound::{Included, Excluded, Unbounded};

/// A newtype that wraps a Hasher64 and provides implementation of std::hash::Hasher methods.
struct Hasher64StdHashHasher<T: Hasher64>(T);

impl<T> Hasher for Hasher64StdHashHasher<T>
    where T: Hasher64
{
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes);
    }

    fn finish(&self) -> u64 {
        self.0.sum64()
    }
}

/// HashRing is a type that tracks a set of buckets corresponding to a collection of resources,
/// usually servers. Items are hashed to these buckets in such a way that few items change their
/// bucket mapping when one is eliminated.
pub struct HashRing<B: Hash> {
    buckets: BTreeMap<u64, B>,
}

impl<B> HashRing<B>
    where B: Hash
{
    /// Get a new HashRing.
    pub fn new() -> HashRing<B> {
        HashRing::<B> { buckets: BTreeMap::new() }
    }

    /// Adds the specified bucket to the hash ring.
    pub fn add_bucket(&mut self, bucket: B) {
        let hash_code = HashRing::<B>::get_hash_code(&bucket);
        self.buckets.insert(hash_code, bucket);
    }

    /// Finds the corresponding bucket for this item.
    pub fn get_bucket<T>(&self, item: &T) -> Option<&B>
        where T: Hash
    {
        let hash_code = HashRing::<B>::get_hash_code(item);
        // If there are no buckets in the end of the circle from here to the end, we wrap around
        // and try from the beginning.
        let back_half = self.buckets.range((Included(&hash_code), Unbounded)).next().map(|val| val.1);
        let front_half = self.buckets
                             .range((Included(&0), Excluded(&hash_code)))
                             .next()
                             .map(|val| val.1);
        back_half.or(front_half)
    }

    /// Eliminate a bucket from the ring. All items mapped to this bucket will be mapped to the
    /// next bucket in hash order (with wraparound).
    /// Returns true if removal is successful, or false if the removal has no effect.
    pub fn remove_bucket(&mut self, bucket: &B) -> bool {
        let code = HashRing::<B>::get_hash_code(bucket);
        match self.buckets.remove(&code) {
            None => false,
            Some(..) => true,
        }
    }

    fn get_hash_code<H>(value: &H) -> u64
        where H: Hash
    {
        let mut hasher = Hasher64StdHashHasher(Digest::new(ECMA));
        value.hash(&mut hasher);
        hasher.finish()
    }
}
