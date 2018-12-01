use std::vec::Vec;
use std::ops::Index;

#[derive(PartialEq, Clone, Hash)]
pub struct BitVec<'a> {
    vec: &'a mut Vec<u64>,
}

impl BitVec {
    fn new(capacity: u32) -> BitVec {
        let capacity = (capacity as f64 / 64.0).ceil() as usize;
        let vec = Vec::with_capacity(capacity);
        BitVec { vec }
    }

    fn capacity(&self) -> usize {
        return self.vec.capacity() * 64;
    }

    fn set(&self, i: u32, v: bool) {
        let (byte, bit) = BitVec::to_index(i);
        let mask:u64 = (1 as u64) << bit;

        let val = self.get(byte);

        let val = if v {
            val | mask;
        } else {
            val & !mask;
        };

        vec[byte] = val;
    }

    fn get(&self, i: u32) -> bool {
        let (byte, bit) = BitVec::to_index(i);
        let mask:u64 = (1 as u64) << bit;

        return self.vec[byte] & mask == mask;
    }

    fn to_index(i: u32) -> (u32, u32) {
        return (i / 64, i % 64);
    }
}

#[cfg(test)]
mod tests {
    fn v(cap: u32) -> super::BitVec {
        return super::BitVec::new(cap);
    }

    #[test]
    fn new() {
        let vec = v(16);

        assert_eq!(64, vec.capacity());

        let vec = v(257);

        assert_eq!(256 + 64, vec.capacity())
    }

    #[test]
    fn get() {
        let vec = v(16);

        assert_eq!(false, vec.get(1));

        vec.set(1, true);

        assert_eq!(true, vec.get(1));
    }
}