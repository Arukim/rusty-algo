
use std::num::Wrapping;

const BIG_PRIME: usize = 0x7FEFFFFD;

#[derive(Copy, Clone)]
pub struct UniHash {
    a: usize,
    b: usize
}

impl UniHash {
    pub fn new(a: usize, b: usize) -> UniHash {
        UniHash {
            a: a,
            b: b
        }
    }

    pub fn get_hash(self, value: usize) -> usize{        
        (Wrapping(self.a) * Wrapping(value) + Wrapping(self.b)).0 % BIG_PRIME
    }
}

pub struct HashClip {
    data: Vec<UniHash>
}

impl HashClip {
    pub fn new() -> HashClip {
        let mut data = Vec::new();

        data.push(UniHash::new(24379,5689));
        data.push(UniHash::new(30661,90023));
        data.push(UniHash::new(34313,25903));
        data.push(UniHash::new(37889,102433));
        data.push(UniHash::new(41579,4871));

        HashClip {
            data : data
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, num: usize) -> UniHash {
        if num >= self.data.len() {
            panic!("Index out of bounds")
        }        
        self.data[num]
    }

    pub fn hash(&self, num: usize, value: usize) -> usize {
        self.get(num).get_hash(value)
    }
}