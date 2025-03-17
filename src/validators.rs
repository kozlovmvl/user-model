use crate::core::{Error, Validator};

pub struct UsernameLengthValidator {
    min_length: usize,
}

pub struct UsernameSymbolValidator {
    valid_symbols: Vec<char>,
}

impl UsernameLengthValidator {
    pub fn new() -> Self {
        Self { min_length: 3 }
    }
}

impl UsernameSymbolValidator {
    pub fn new() -> Self {
        Self {
            valid_symbols: Vec::from_iter(('a'..='z').chain('A'..='Z').chain('0'..='9')),
        }
    }
}

impl Validator<String> for UsernameLengthValidator {
    fn validate(&self, value: &String) -> Result<bool, Error> {
        if value.len() < self.min_length {
            return Err(Error::ValidationError("Wrong length of username"));
        }
        Ok(true)
    }
}

impl Validator<String> for UsernameSymbolValidator {
    fn validate(&self, value: &String) -> Result<bool, Error> {
        for c in value.chars() {
            if !self.valid_symbols.contains(&c) {
                return Err(Error::ValidationError("Invalid symbol in username"));
            }
        }
        Ok(true)
    }
}
