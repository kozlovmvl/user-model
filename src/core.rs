pub enum Error {
    ValidationError(&'static str),
}

pub trait FieldControl<T> {
    fn get(&self) -> Option<T>;

    fn set(&mut self, value: Option<T>) -> &mut Self;

    fn validate(&mut self) -> Result<&mut Self, Error>
    where
        Self: Sized;
}

pub trait Validator<T> {
    fn validate(&self, value: &T) -> Result<bool, Error>;
}
