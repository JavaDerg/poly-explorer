pub struct Lfsr<const N: usize> {
    start: u32,
    state: u32,
    poly: u32,
}

impl<const N: usize> Lfsr<N> {
    pub fn new(poly: u32, start: u32) -> Self {
        assert_eq!(32 - poly.leading_zeros(), N as u32);
        Self {
            start: start & mask(N),
            state: start,
            poly,
        }
    }
    pub fn get_n(&self, bits: usize) -> u32 {
        assert!(bits <= 32);
        self.state & mask(bits)
    }
}

impl<const N: usize> Iterator for Lfsr<N> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.state & mask(N);
        self.state = (self.state << 1) | ((self.state & self.poly).count_ones() & 1);
        if self.start == (self.state & mask(N)) {
            return None;
        }
        Some(out)
    }
}

pub(crate) const fn mask(n: usize) -> u32 {
    ((1u64 << n) - 1) as u32
}

#[test]
fn test_mask() {
    assert_eq!(mask(17).count_ones(), 17);
}
