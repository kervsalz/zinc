//!
//! The syntax witness.
//!

use crate::syntax::Identificator;
use crate::syntax::Type;

#[derive(Debug)]
pub struct Witness {
    id: Identificator,
    r#type: Type,
}
