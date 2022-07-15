use crate::errors::{BuildError, ValidationError};
pub mod implementations;

pub trait Validate {
    fn validate(self) -> Result<Self, ValidationError> where Self: Sized;
}

pub trait AddField<T> {
    fn add_field(self, field: T) -> Self;
}

pub trait BackIntoBuilder<T: Validate, U: ParentBuilder + BackIntoBuilder<T, U>> {
    fn add_field(self, field: T) -> Self;
    fn sub_builder_from(self, body: T) -> Builder<T, U>;
}

pub trait ParentBuilder {}

pub struct Nil;
impl ParentBuilder for Nil{}


pub struct Builder<T, U>
    where T: Validate,
          U: ParentBuilder
{
    pub(crate) body: T,
    pub(crate) builder: Option<U>
}

impl<T: Validate, U: ParentBuilder> ParentBuilder for Builder<T, U> {}

impl<T: Validate, U: ParentBuilder> Builder<T, U> {
    pub async fn build(self) -> Result<T, BuildError> {
        match self.body.validate() {
            Ok(body) => Ok(body),
            Err(_) => Err(BuildError)
        }
    }
}

impl<T: Validate, V: ParentBuilder + BackIntoBuilder<T, V>> Builder<T, V> {
    pub fn into_builder(self) -> Result<V, BuildError> {
        match self.body.validate() {
            Ok(body) => Ok(self.builder.unwrap().add_field(body)),
            Err(_) => Err(BuildError)
        }
    }
}

// impl<T: AddField<V>, U: ParentBuilder, V: Validate> Builder<T, U> {
//     fn sub_builder_from(self, body: V) -> Builder<T, Builder<T, U>> {
//         Builder {
//             body,
//             builder: Some(self),
//         }
//     }
// }

impl<T: Validate> From<T> for Builder<T, Nil> {
    fn from(body: T) -> Self {
        Builder {
            body,
            builder: None::<Nil>
        }
    }
}