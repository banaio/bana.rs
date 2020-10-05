extern crate term_size;

use log::{error, trace};

pub fn dimensions_or_exit() -> (usize, usize) {
    if let Some((w, h)) = term_size::dimensions() {
        trace!("term_size::dimensions: width={}, height={}", w, h);
        (w, h)
    } else {
        error!("term_size::dimensions: unable to get term size");
        std::process::exit(1);
    }
}

// #[cfg(test)]
// mod tests {
//     use pretty_assertions::assert_eq;
//     use serde_json;

//     #[test]
//     fn test_deserialize_good() {
//         let expected = super::ClientCredentialsGrant {
//             access_token: "c875cc35-b712-4904-9ff1-9de9dc2b6014".to_string(),
//             token_type:   "Bearer".to_string(),
//             expires_in:   std::time::Duration::from_secs(3600),
//         };
//         let s = r#"
// {
//     "access_token": "c875cc35-b712-4904-9ff1-9de9dc2b6014",
//     "token_type": "Bearer",
//     "expires_in": 3600
// }
//         "#;
//         let actual =
// serde_json::from_str::<super::ClientCredentialsGrant>(s).unwrap();

//         assert_eq!(actual, expected);
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
