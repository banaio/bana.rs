use log::{debug, trace};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{BuildHasher, BuildHasherDefault, Hash, Hasher},
};

// use super::errors::ErrorParameterInvalid;
// use super::ErrorParameterInvalid;
// use super::hash_partition::HashPartition;
use super::ErrorParameterInvalid;

// use self::HashPartition;
use super::HashPartition;

// #[derive(Default, Debug, Clone)]
// #[derive(Debug)]
pub struct PCSA {
    pub b: u32,
    pub m: u64,
    pub bitmaps_size: u64,
    pub bitmaps: Vec<u64>,
    pub hasher: Box<dyn BuildHasher<Hasher = HasherType>>,
}

/// PCSA -
/// * [Sketch of the Day: Probabilistic Counting with Stochastic Averaging (PCSA)](https://research.neustar.biz/2013/04/02/sketch-of-the-day-probabilistic-counting-with-stochastic-averaging-pcsa/)
/// * [Sketch of the Day: Probabilistic Counting with Stochastic Averaging (PCSA)](https://agkn.wordpress.com/2013/04/02/sketch-of-the-day-probabilistic-counting-with-stochastic-averaging-pcsa/)
/// * [Flajolet–Martin algorithm](https://en.wikipedia.org/wiki/Flajolet%E2%80%93Martin_algorithm)
/// * [Probabilistic Counting Algorithms for Data Base Applications](http://algo.inria.fr/flajolet/Publications/src/FlMa85.pdf)
impl PCSA {
    pub fn new(b: u32) -> Result<Self, ErrorParameterInvalid> {
        // m = 2^b # with b in [4...16]
        if b < 4 || b > 16 {
            return Err(ErrorParameterInvalid::new(b));
        }

        let m = 2u64.pow(b); // 32
        let bitmaps_size = m;
        let bitmaps = [0].repeat(bitmaps_size as usize);
        trace!(
            "PCSA.new: b={:?}, m={:?}, bitmaps_size={:?}, bitmaps={:?}",
            b,
            m,
            bitmaps_size,
            bitmaps
        );

        type Builder = BuildHasherDefault<HasherType>;
        let hasher = Builder::default();

        Ok(Self {
            b,
            m,
            bitmaps_size,
            bitmaps,
            hasher: Box::new(hasher),
        })
    }

    pub fn cardinality(&self) -> std::io::Result<u64> {
        // Determine the cardinality
        // phi = 0.77351
        // DV = m / phi * 2 ^ (sum( least_sig_bit( bitmap ) ) / m) # the DV
        // estimate
        let result = 0;
        trace!("PCSA.cardinality: result={:?}", result);
        Ok(result)
    }

    pub fn hash(&mut self, record: String) -> u64 {
        let mut hasher = self.hasher.build_hasher();
        hasher.write(record.as_bytes());
        let hash = hasher.finish();
        hash
    }

    pub fn add(&mut self, record: String) -> std::io::Result<()> {
        // https://doc.rust-lang.org/std/primitive.u64.html
        // https://stackoverflow.com/questions/50458144/what-is-the-easiest-way-to-pad-a-string-with-0-to-the-left
        // use std::ops::{BitOr, BitOrAssign};
        let hash = self.hash(record.clone());
        let partition = HashPartition::new(self.b, hash);
        PCSA::debug_hashed_record(record.clone(), partition.clone());

        let leading_zeros = hash.leading_zeros();
        let bitmap_index = partition.bottom as usize;
        let bitmap_old = self.bitmaps[bitmap_index].clone();

        let leading_zeros_new = partition.top.leading_zeros();
        let bitmap_new = bitmap_old | (1 << leading_zeros_new);

        debug!(
            "PCSA:add: bitmap_index={:?}, leading_zeros={:?}, \
             leading_zeros_new={:?}, hash_binary={:?}",
            bitmap_index,
            leading_zeros,
            leading_zeros_new,
            format!("{:064b}", hash),
        );
        PCSA::debug_bitmap_update(
            partition.clone(),
            bitmap_old,
            bitmap_new,
            leading_zeros,
        );
        // set the bitmap bit based on the run length observed
        self.bitmaps[bitmap_index] = bitmap_new;

        Ok(())
    }

    pub fn debug_bitmap_update(
        partition: HashPartition,
        bitmap_old: u64,
        bitmap_new: u64,
        leading_zeros: u32,
    ) {
        let hash_bytes = partition.hash.to_be_bytes();
        let run_length = 0;
        let leading_zeros_bytes = leading_zeros.to_be_bytes();
        let leading_zeros_new = partition.top.leading_zeros();

        trace!(
            "
\t run_length             ={:?}
\t hash_bytes             ={:?}
\t bitmap_old             ={:?}
\t bitmap_new             ={:?}
\t leading_zeros_bytes    ={:?}
\t leading_zeros_binary   ={:?}
\t leading_zeros          ={:?}
\t leading_zeros_new      ={:?}
\t bitmap_value_old_binary={:?}
\t bitmap_value_new_binary={:?}",
            run_length,
            hash_bytes,
            bitmap_old,
            bitmap_new,
            leading_zeros_bytes,
            format!("{:064b}", leading_zeros),
            leading_zeros,
            leading_zeros_new,
            format!("{:064b}", bitmap_old),
            format!("{:064b}", bitmap_new),
        );
    }

    pub fn debug_hashed_record(record: String, partition: HashPartition) {
        let hash = partition.hash;

        trace!(
            "
\t record                 ={:?}
\t hash                   ={:?}
\t hash_binary            ={:?}
\t count_ones             ={:?}
\t count_zeros            ={:?}
\t leading_zeros          ={:?}
\t trailing_zeros         ={:?}
\t leading_ones           ={:?}
\t trailing_ones          ={:?}
\t partition              ={:?}",
            record,
            hash,
            format!("{:064b}", hash),
            hash.count_ones(),
            hash.count_zeros(),
            hash.leading_zeros(),
            hash.trailing_zeros(),
            -1,
            -1,
            partition,
        );
    }
}

impl std::default::Default for PCSA {
    fn default() -> Self {
        // let m = 32;
        // m = 2^b # with b in [4...16]
        const B: u32 = 5;
        let m = 2u64.pow(B); // 32
        let bitmaps_size = m;
        let bitmaps = [0].repeat(bitmaps_size as usize);

        type Builder = BuildHasherDefault<HasherType>;
        let hasher = Builder::default();

        PCSA {
            b: B,
            m,
            bitmaps_size,
            bitmaps,
            hasher: Box::new(hasher),
        }
    }
}

impl std::fmt::Debug for PCSA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let bitmaps_as_str = self
            .bitmaps
            .iter()
            .map(|bitmap_value| format!("{:064b}", bitmap_value))
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "[
              b={:?}
              m={:?}
              bitmaps_size={:?}
              bitmaps={:?}
            ]",
            self.b, self.m, self.bitmaps_size, bitmaps_as_str,
        )
    }
}

impl std::fmt::Display for PCSA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let bitmaps_as_str = self
            .bitmaps
            .iter()
            .map(|bitmap_value| format!("{:064b}", bitmap_value))
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "[
              b={:?}
              m={:?}
              bitmaps_size={:?}
              bitmaps={:?}
            ]",
            self.b, self.m, self.bitmaps_size, bitmaps_as_str,
        )
    }
}

type HasherType = DefaultHasher;

#[derive(Default, Debug, Clone, Hash)]
pub struct BitMaps(pub Vec<u64>);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
