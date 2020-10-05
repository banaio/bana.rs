#[derive(Default, Clone, Hash)]
pub struct HashPartition {
    pub hash: u64,
    pub low_m_bits: u64,
    pub hi: u32,
    pub lo: u32,
    pub top: u64,
    pub bottom: u64,
    pub leading_zeros_new: u32,
}

impl HashPartition {
    pub fn new(b: u32, hash: u64) -> Self {
        // (x >> 3) & ((1 << 5)-1); // first 5 bits
        // (x >> 0) & ((1 << 3)-1) // lower 3 bits

        let _hash_bytes = hash.to_be_bytes();
        let leading_zeros = hash.leading_zeros();
        let _leading_zeros_bytes = leading_zeros.to_be_bytes();

        let low_m_bits = hash & ((1 << b) - 1);
        let hi = 64 - b;
        let lo = b;
        let top = (hash >> lo) & ((1 << hi) - 1);
        let bottom = hash & ((1 << lo) - 1);
        let leading_zeros_new = top.leading_zeros();

        Self {
            hash,
            low_m_bits,
            hi,
            lo,
            top,
            bottom,
            leading_zeros_new,
        }
    }

    pub fn format_to_string(&self) -> String {
        format!(
            "\
\t\t hash={:?}
\t\t low_m_bits={:?}
\t\t hi={:?}
\t\t lo={:?}
\t\t top={:?}
\t\t bottom={:?}
\t\t hash_binary={:?}
\t\t low_m_bits_binary={:?}
\t\t hi_binary={:?}
\t\t lo_binary={:?}
\t\t leading_zeros_new={:?}
\t\t top_binary={:?}
\t\t bottom_binary={:?}",
            self.hash,
            self.low_m_bits,
            self.hi,
            self.lo,
            self.top,
            self.bottom,
            format!("{:064b}", self.hash),
            format!("{:064b}", self.low_m_bits),
            format!("{:064b}", self.hi),
            format!("{:064b}", self.lo),
            self.leading_zeros_new,
            format!("{:064b}", self.top),
            format!("{:064b}", self.bottom),
        )
    }
}

impl std::fmt::Debug for HashPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pcsa.HashPartition = {{\n{}\n\t}}", self.format_to_string())
    }
}

impl std::fmt::Display for HashPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.format_to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
