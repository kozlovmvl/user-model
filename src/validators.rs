use crate::core::{Error, Validator};

pub struct UsernameLengthValidator {
    min_length: usize,
}

pub struct UsernameSymbolValidator {
    valid_symbols: Vec<char>,
}

pub struct EmailSymbolValidator {}

pub struct EmailStructureValidator {}

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

impl EmailSymbolValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl EmailStructureValidator {
    pub fn new() -> Self {
        Self {}
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

impl Validator<String> for EmailSymbolValidator {
    fn validate(&self, value: &String) -> Result<bool, Error> {
        if !value.contains('@') {
            return Err(Error::ValidationError("Email must contain symbol '@' "));
        }
        Ok(true)
    }
}

impl Validator<String> for EmailStructureValidator {
    fn validate(&self, value: &String) -> Result<bool, Error> {
        let parts: Vec<&str> = value.rsplit('@').collect();
        if parts.len() != 2 {
            return Err(Error::ValidationError("Wrong format of email"));
        }
        Ok(true)
    }
}
