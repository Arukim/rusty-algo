use uni_hash::HashClip;

const INITIAL_SIZE: usize = 4;
const MAX_REHASH: usize = 5;
const MAX_REPLACE_DEPTH: usize = 5;

pub struct HashTable<TValue> {
    data: Vec<Option<HashEntry<TValue>>>,
    count: usize,
    bucket_size: usize,
    hash_clip: HashClip,
    current_hash: usize,
}

#[derive(Copy, Clone, Default)]
struct HashEntry<TValue> {
    key: usize,
    value: TValue,
}

impl<TValue: Clone + Copy + Default> HashTable<TValue> {
    pub fn new() -> HashTable<TValue> {
        HashTable {
            data: vec![Default::default(); INITIAL_SIZE],
            count: 0,
            bucket_size: INITIAL_SIZE / 2,
            hash_clip: HashClip::new(),
            current_hash: 0,
        }
    }

    pub fn add(&mut self, key: usize, value: TValue) {
        if self.count == self.bucket_size {
            //grow the node
            self.resize(self.bucket_size * 2);
        }

        self.insert(HashEntry {
            key: key,
            value: value,
        });
        self.count += 1;
    }

    pub fn check(&mut self, key: usize) -> bool {
        let (l_hash, r_hash) = self.get_hash(key);

        self.check_entry(key, self.left(l_hash)) || self.check_entry(key, self.right(r_hash))
    }

    #[inline]
    fn left(&self, value: usize) -> usize {
        value % self.bucket_size
    }

    #[inline]
    fn right(&self, value: usize) -> usize {
        (value % self.bucket_size) + self.bucket_size
    }

    fn check_entry(&self, key: usize, pos: usize) -> bool {
        match self.data[pos] {
            Some(entry) => entry.key == key,
            None => false,
        }
    }

    fn resize(&mut self, new_size: usize) {
        self.bucket_size = new_size;
        self.rehash();
    }

    fn rehash(&mut self) {
        let prev_data = self.data.to_vec();
        self.data = vec![Default::default(); self.bucket_size * 2];
        for item in prev_data {
            match item {
                None => {
                    continue;
                }
                Some(entry) => {
                    self.insert(entry);
                }
            }
        }
    }

    fn insert_rec(&mut self, entry: HashEntry<TValue>, depth: usize) -> bool {
        let (l_hash, r_hash) = self.get_hash(entry.key);
        let (left_pos, right_pos) = (self.left(l_hash), self.right(r_hash));

        if self.put(entry, left_pos) {
            return true;
        }

        if self.put(entry, right_pos) {
            return true;
        }

        if depth == MAX_REPLACE_DEPTH {
            // we need to restore items back
            return false;
        }

        let replace_pos = if depth % 2 == 0 { left_pos } else { right_pos };

        let evicted = self.data[replace_pos];
        self.data[replace_pos] = Some(entry);
        let res = self.insert_rec(evicted.unwrap(), depth + 1);

        if !res {
            self.data[replace_pos] = evicted;
        }

        return res;
    }

    fn insert(&mut self, entry: HashEntry<TValue>) {
        for _ in 0..MAX_REHASH {
            if self.insert_rec(entry, 0) {
                return;
            } else {
                println!("rehash happens!");
                self.current_hash += 1;
                self.rehash();
            }
        }

        panic!("failed to insert the value");
    }

    fn put(&mut self, entry: HashEntry<TValue>, pos: usize) -> bool {
        match self.data[pos] {
            None => {
                self.data[pos] = Some(entry);
                true
            }
            Some(_) => false,
        }
    }

    fn get_hash(&self, value: usize) -> (usize, usize) {
        let (left, right) = (
            self.current_hash % self.hash_clip.size(),
            (self.current_hash + 1) % self.hash_clip.size(),
        );
        (
            self.hash_clip.hash(left, value),
            self.hash_clip.hash(right, value),
        )
    }
}
