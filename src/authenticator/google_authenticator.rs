use google_authenticator::GAError;
use google_authenticator::GoogleAuthenticator;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait CodeGenerator {
    fn get_code(&self, secret: &str) -> Result<String, GAError>;
}

impl CodeGenerator for GoogleAuthenticator {
    fn get_code(&self, secret: &str) -> Result<String, GAError> {
        self.get_code(secret, 0)
    }
}

pub struct Authenticator {
    pub generator: Box<dyn CodeGenerator>,
}

impl Default for Authenticator {
    fn default() -> Self {
        Self{ generator: Box::new(GoogleAuthenticator::new()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_code() {
        let mut generator_mock = MockCodeGenerator::new();
        generator_mock
            .expect_get_code()
            .times(1)
            .returning(|_| Ok(String::from("987123")));
        let authenticator = Authenticator { generator: Box::new(generator_mock) };
        let result = authenticator
            .generator
            .get_code("ABC")
            .expect("It should not fail.");
        let expected = String::from("987123");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_code_failed() {
        let mut generator_mock = MockCodeGenerator::new();
        generator_mock.
            expect_get_code()
            .times(1)
            .returning(|_| Err(GAError::Error("Failed to get code")));
        let authenticator = Authenticator { generator: Box::new(generator_mock) };
        assert!(authenticator.generator.get_code("ABC").is_err());
    }
}
