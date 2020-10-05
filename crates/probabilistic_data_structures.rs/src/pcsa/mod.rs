// pub mod pcsa;
mod pcsa;
// pub mod errors;
mod errors;
// pub mod hash_partition;
mod hash_partition;

pub use self::pcsa::*;
// pub use pcsa::PCSA;

// pub use self::errors::{*} as errors;
pub use self::errors::*;

pub use self::hash_partition::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
