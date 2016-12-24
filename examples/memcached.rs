extern crate consist;

use consist::HashRing;

fn main() {
    // List of servers
    let mut ring = HashRing::new();
    for i in 1..4 {
        let bucket = format!("192.168.0.{}", i);
        println!("bucket {}", bucket);
        ring.add_bucket(format!("192.168.0.{}", i));
    }
    // Perform lookups for database queries
    for search in vec!["Alpha",
                       "Beta",
                       "Gamma",
                       "James",
                       "Earl",
                       "Jones",
                       "Darth Vader",
                       "Harry Potter",
                       "blargh man"]
                      .iter() {
        println!("{:15} => {}", search, ring.get_bucket(search).unwrap());
    }
}
