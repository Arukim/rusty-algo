use cuckoo_hash::hash_table::{HashTable};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut table = HashTable::new();

    println!("Adding 16 values");
    for i in 0..16 {
        table.add(i,i)
    }

    println!("Checking 16 values");
    for i in 0..16 {
        let found = table.check(i);
        if !found {
            println!("key {0} not found", i);
            panic!("key not found!");
        }
    }

    println!("Adding random values");
    table = HashTable::new();
    for i in 0..1024 {        
        table.add(rng.gen::<usize>(), i)
    }
}
