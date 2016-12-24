use bit_vec::BitVec;
use siphasher::sip::SipHasher;

pub struct SimpleBloomFilter {
    bitvector: BitVec,
    nbits: u64,
    hasher: SipHasher
}

impl SimpleBloomFilter {

    pub fn new(nbits: u64) -> SimpleBloomFilter {
        assert!(nbits > 0);

        SimpleBloomFilter {
            bitvector: BitVec::from_elem(nbits as usize, false),
            nbits: nbits,
            hasher: SipHasher::new()
        }
    }

}
