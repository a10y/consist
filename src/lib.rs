//! consist: An implementation of consistent hashing in Rust.
//! The goal of consistent hashing is to partition entries in such a way that the addition or
//! removal of buckets minimizes the number of items that must be shifted between buckets, i.e. it
//! optimizes the rehashing stage that is usually needed for hash tables.
//! The algorithm was originally put forth by David Karger et al. in their 1997 paper
//! "Consistent Hashing and Random Trees".

/// The Ring trait lays out the foundations of what a consistent hash ring implementation must
/// contain.
pub trait Ring {
    type Bucket: std::hash::Hash;
    type Item: std::hash::Hash;

    fn add_bucket(bucket: &Self::Bucket);
    fn add_item(item: &Self::Item);
    fn remove_item(item: &Self::Item);
}
