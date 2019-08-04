//!
//! The syntax input builder.
//!

use failure::Fail;

use crate::syntax::Identifier;
use crate::syntax::Input;
use crate::syntax::Type;

#[derive(Default)]
pub struct Builder {
    identifier: Option<Identifier>,
    r#type: Option<Type>,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "missing identifier")]
    MissingIdentifier,
    #[fail(display = "missing type")]
    MissingType,
}

impl Builder {
    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_type(&mut self, value: Type) {
        self.r#type = Some(value);
    }

    pub fn build(&mut self) -> Result<Input, Error> {
        Ok(Input {
            identifier: match self.identifier.take() {
                Some(identifier) => identifier,
                None => return Err(Error::MissingIdentifier),
            },
            r#type: match self.r#type.take() {
                Some(r#type) => r#type,
                None => return Err(Error::MissingType),
            },
        })
    }
}
