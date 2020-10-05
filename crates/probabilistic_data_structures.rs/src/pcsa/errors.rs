use std::{error::Error, fmt};

// #[derive(Debug)]
#[derive(Default, Debug, Clone)]
pub struct ErrorParameterInvalid {
    b: u32,
}

impl ErrorParameterInvalid {
    pub fn new(b: u32) -> Self {
        Self {
            b,
        }
    }
}

impl fmt::Display for ErrorParameterInvalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pcsa.ErrorParameterInvalid: expected=(4 <= b <= 16), got b=({:?})",
            self.b
        )
    }
}

impl Error for ErrorParameterInvalid {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
