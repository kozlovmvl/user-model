use crate::{
    core::{Error, FieldControl, Validator},
    validators::{
        EmailStructureValidator, EmailSymbolValidator, UsernameLengthValidator,
        UsernameSymbolValidator,
    },
};
use uuid::Uuid;

pub struct User {
    id: Uuid,
    username: Username,
    email: Email,
}

pub struct Username {
    value: Option<String>,
}

pub struct Email {
    value: Option<String>,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            username: Username::new(None),
            email: Email::new(None),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&mut self) -> &mut Username {
        &mut self.username
    }

    pub fn email(&mut self) -> &mut Email {
        &mut self.email
    }
}

impl Username {
    pub fn new(value: Option<String>) -> Self {
        Self { value }
    }
}

impl Email {
    pub fn new(value: Option<String>) -> Self {
        Self { value }
    }
}

impl FieldControl<String> for Username {
    fn get(&self) -> Option<String> {
        self.value.clone()
    }

    fn set(&mut self, value: Option<String>) -> &mut Self {
        self.value = value;
        self
    }

    fn validate(&mut self) -> Result<&mut Self, Error> {
        if self.value.is_none() {
            return Ok(self);
        }
        let validator = UsernameLengthValidator::new();
        validator.validate(self.value.as_ref().unwrap())?;
        let validator = UsernameSymbolValidator::new();
        validator.validate(self.value.as_ref().unwrap())?;
        Ok(self)
    }
}

impl FieldControl<String> for Email {
    fn get(&self) -> Option<String> {
        self.value.clone()
    }

    fn set(&mut self, value: Option<String>) -> &mut Self {
        self.value = value;
        self
    }

    fn validate(&mut self) -> Result<&mut Self, Error> {
        if self.value.is_none() {
            return Ok(self);
        }
        let validator = EmailSymbolValidator::new();
        validator.validate(self.value.as_ref().unwrap())?;
        let validator = EmailStructureValidator::new();
        validator.validate(self.value.as_ref().unwrap())?;
        Ok(self)
    }
}
