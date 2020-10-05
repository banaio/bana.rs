// extern crate probabilistic_data_structures;

pub mod bloom_filter;
pub mod count_min_sketch;
pub mod hyperloglog;
pub mod lsh;
pub mod minhash;
pub mod pcsa;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
