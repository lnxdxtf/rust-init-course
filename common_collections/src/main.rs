#![allow(dead_code)] // allow dead code level crate -- all in the file

use std::collections::{HashMap, HashSet};

fn print_debug<T: std::fmt::Debug>(value: &T) {
    println!("{:?}", value);
}

fn section_hash_map() {
    let mut hash_map_test = HashMap::new();
    hash_map_test.insert("olokinhomeu", 10); // insert a key-value hashmap.insert(T,U)
    hash_map_test.insert("olokinhomeu2", 120); // insert a key-value hashmap.insert(T,U)
    print_debug(&hash_map_test.get("olokinhomeu").unwrap()); // hashmap.get(key) - return the value from a key   || use unwrap to get just the value. no option<t>
    print_debug(&hash_map_test.len());
    hash_map_test.remove("olokinhomeu"); // remove a value from a key
    print_debug(&hash_map_test);
}

fn section_hash_set() {
    let mut hash_set_1 = HashSet::new();
    let mut hash_set_2 = HashSet::new();
    for i in 0..10 {
        // use 1..10 (n..N) to create a sequence of number iter
        hash_set_1.insert(i); // insert a value in hashset
        hash_set_2.insert(i / 2); // insert a value in hashset
    }
    print_debug(&hash_set_1);
    print_debug(&hash_set_2);

    //intersections
    println!("---------");
    println!("intersections hashset styles");
    //style-1
    // for x in hash_set_1.intersection(&hash_set_2) {
    //     println!("Intersection: {}", x);
    // }
    //style-2
    let intersection = &hash_set_1 & &hash_set_2;
    for x in intersection {
        println!("Intersection: {}", x);
    }

    println!("---------");
    println!("union hashset styles");
    //style-1
    // for x in hash_set_1.union(&hash_set_2) {
    //     println!("Union: {}", x);
    // }
    //style-2
    let union = &hash_set_1 | &hash_set_2;
    for x in union {
        println!("Union: {}", x);
    }
    println!("---------");
    println!("difference hashset styles");
    //style-1
    // for x in hash_set_1.difference(&hash_set_2) {
    //     println!("Difference: {}", x);
    // }
    let difference = &hash_set_1 - &hash_set_2;
    for x in difference {
        println!("Difference: {}", x);
    }
}

fn main() {
    // section_hash_map();
    section_hash_set();
}
