use cuckoo_hash::hash_table::{HashTable};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut table = HashTable::new();

    for i in 1..16 {
        table.add(i,i)
    }

    for i in 1..16 {
        let found = table.check(i);
        println!("key {0} {1}", i, if found {"found"} else {"not found"});
    }

    table = HashTable::new();
    for i in 0..16 {        
        table.add(rng.gen::<usize>(), i)
    }
}
