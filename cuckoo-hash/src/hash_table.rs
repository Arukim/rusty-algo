use uni_hash::HashClip;

pub struct HashTable {
    data: Vec<usize>,
    count: usize,
    bucket_size: usize,
    hash_clip: HashClip,
    current_hash: usize
}

impl HashTable {
    const INITIAL_SIZE: usize = 4;

    pub fn new() -> HashTable {
        HashTable {
            data: vec![0; HashTable::INITIAL_SIZE],
            count: 0,
            bucket_size: HashTable::INITIAL_SIZE/2,
            hash_clip: HashClip::new(),
            current_hash: 0
        }
    }

    pub fn add(&mut self, value: usize) {
        if self.count == self.bucket_size {
            //grow the node
            self.bucket_size *= 2;
            self.data.resize(self.bucket_size * 2, 0);
            for i in 0..self.bucket_size {
                let tmp = self.data[i];
                if tmp != 0 {
                    self.data[i] = 0;
                    self.insert(tmp);
                }
            }
        }

        self.insert(value);
        self.count += 1;
    }

    pub fn check(&mut self, value: usize) -> bool {
        let (l_hash, r_hash) = self.get_hash(value);

        self.data[l_hash % self.bucket_size] != 0 ||
        self.data[(r_hash % self.bucket_size) + self.bucket_size] != 0
    }

    fn insert(&mut self, value: usize) {
        let (l_hash, r_hash) = self.get_hash(value);

        let left_pos = l_hash % self.bucket_size;
        let right_pos = r_hash % self.bucket_size + self.bucket_size;

        if self.data[left_pos] == 0 {
            self.data[left_pos] = value;
            return;
        }

        if self.data[right_pos] == 0 {
            self.data[right_pos] = value;
            return;
        }
        
        panic!("failed to insert the value");
    }

    fn get_hash(&self, value: usize) -> (usize, usize) {
        let (left, right) = (self.current_hash % self.hash_clip.size(), 
                            (self.current_hash + 1) % self.hash_clip.size());
        (self.hash_clip.hash(left, value), self.hash_clip.hash(right, value))
    }
}
