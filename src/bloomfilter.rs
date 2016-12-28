use std::hash::Hash;
use std::hash::Hasher;
use std::hash::BuildHasher;
use std::collections::hash_map::DefaultHasher;

use bit_vec::BitVec;

#[derive(Default)]
pub struct BloomFilter {
    bitvector: BitVec,
    nbits: u64,
}

impl BuildHasher for BloomFilter {
    type Hasher = DefaultHasher;

    fn build_hasher(&self) -> DefaultHasher {
        DefaultHasher::default()
    }
}

impl BloomFilter {

    pub fn new(nbits: u64) -> BloomFilter {
        assert!(nbits > 0);

        BloomFilter {
            bitvector: BitVec::from_elem(nbits as usize, false),
            nbits: nbits
        }
    }

    pub fn set<T: Hash>(&mut self, item: T) {
        let hash = self.hash(&item);
        let offset = (hash % self.nbits) as usize;

        self.bitvector.set(offset, true);
    }

    pub fn check<T: Hash>(&mut self, item: T) -> bool {
        let hash = self.hash(&item);
        let offset = (hash % self.nbits) as usize;

        match self.bitvector.get(offset) {
            Some(true) => true,
            _ => false
        }
    }

    pub fn check_and_set<T: Hash>(&mut self, value: T) -> bool {
        let checked = self.check(&value);
        self.set(value);
        checked
    }

    pub fn clear(&mut self) {
        self.bitvector.clear();
    }

    fn hash<T: Hash>(&mut self, item: &T) -> u64 {
        let mut hasher = self.build_hasher();
        item.hash(&mut hasher);
        hasher.finish()
    }

}

#[test]
fn new_filter_test() {
    let filter = BloomFilter::new(16);
    assert_eq!(filter.bitvector.none(), true);
    assert_eq!(filter.nbits, 16);
}

#[test]
fn check_empty_filter_test() {
    let mut filter = BloomFilter::new(16);
    assert_eq!(filter.check("test"), false);
}

#[test]
fn set_value_in_filter_test() {
    let mut filter = BloomFilter::new(16);
    filter.set("test");
    assert_eq!(filter.check("test"), true);
}

#[test]
fn clear_filter_test() {
    let mut filter = BloomFilter::new(16);
    filter.set("test");
    assert_eq!(filter.bitvector.none(), false);
    filter.clear();
    assert_eq!(filter.bitvector.none(), true);
}
