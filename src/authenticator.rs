use google_authenticator::GAError;
cfg_if::cfg_if! {
    if #[cfg(not(test))] {
        use google_authenticator::GoogleAuthenticator;
    } else {
        // GoogleAuthenticator mock
        struct GoogleAuthenticator {}
        impl Default for GoogleAuthenticator {
            fn default() -> Self {
                Self {}
            }
        }
        impl GoogleAuthenticator {
             pub fn new() -> Self {
                Self::default()
            }
            pub fn get_code(&self, _secret: &str, _times_slice: u64) -> Result<String, GAError> {
                Ok(String::from("123456"))
            }
        }
    }
}

pub fn get_code(secret: &str) -> Result<String, GAError> {
    let auth = GoogleAuthenticator::new();
    match auth.get_code(&secret, 0) {
        Ok(code) => Ok(code),
        Err(error) => Err(error)
    }
}


#[cfg(test)]
mod tests {
    use crate::authenticator::get_code;

    #[test]
    fn test_get_code() {
        assert_eq!(get_code("ABC").expect("It should not fail."), String::from("123456"));
    }
}
