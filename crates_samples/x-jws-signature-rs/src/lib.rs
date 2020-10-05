mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // https://rustwasm.github.io/book/game-of-life/debugging.html#enable-logging-for-panics
    utils::set_panic_hook();

    alert("Hello, {{project-name}}!");
    do_jwk();
}

extern crate biscuit;
#[macro_use]
extern crate serde_derive;

use std::str::FromStr;
use biscuit::*;
use biscuit::jws::*;
use biscuit::jwa::*;

pub fn do_jwk() {
    // Define our own private claims
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    struct PrivateClaims {
        pub company: String,
        pub department: String,
    }

    let signing_secret = Secret::Bytes("secret".to_string().into_bytes());

    let expected_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.\
                          eyJpc3MiOiJodHRwczovL3d3dy5hY21lLmNv\
                          bS8iLCJzdWIiOiJKb2huIERvZSIsImF1ZCI6I\
                          mh0dHM6Ly9hY21lLWN1c3RvbWVyLmNvbS8iLC\
                          JuYmYiOjEyMzQsImNvbXBhbnkiOiJBQ01FIiwi\
                          ZGVwYXJ0bWVudCI6IlRvaWxldCBDbG\
                          VhbmluZyJ9.dnx1OmRZSFxjCD1ivy4lveTT-sxay5Fq6vY6jnJvqeI";

    let expected_claims = ClaimsSet::<PrivateClaims> {
        registered: RegisteredClaims {
            issuer: Some(FromStr::from_str("https://www.acme.com").unwrap()),
            subject: Some(FromStr::from_str("John Doe").unwrap()),
            audience: Some(SingleOrMultiple::Single(
                FromStr::from_str("htts://acme-customer.com").unwrap(),
            )),
            not_before: Some(1234.into()),
            ..Default::default()
        },
        private: PrivateClaims {
            department: "Toilet Cleaning".to_string(),
            company: "ACME".to_string(),
        },
    };

    let expected_jwt = JWT::new_decoded(
        From::from(RegisteredHeader {
            algorithm: SignatureAlgorithm::HS256,
            ..Default::default()
        }),
        expected_claims.clone(),
    );

    let token = expected_jwt.into_encoded(&signing_secret).unwrap();
    alert(format!("token={:?}", token).as_str());
    let token = token.unwrap_encoded().to_string();
    alert(format!("token={:?}", token).as_str());
    assert_eq!(expected_token, token);
    // Now, send `token` to your clients

    // ... some time later, we get token back!

    let token = JWT::<_, biscuit::Empty>::new_encoded(&token);
    let token = token
        .into_decoded(&signing_secret, SignatureAlgorithm::HS256)
        .unwrap();
    alert(format!("token={:?}", token).as_str());
    assert_eq!(*token.payload().unwrap(), expected_claims);
}