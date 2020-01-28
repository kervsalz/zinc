//!
//! The semantic analyzer function type element.
//!

mod assert;
mod debug;
mod standard;
mod user;

pub use self::assert::AssertInstructionFunction;
pub use self::debug::DebugInstructionFunction;
pub use self::standard::PedersenStandardLibraryFunction;
pub use self::standard::Sha256StandardLibraryFunction;
pub use self::standard::StandardLibraryFunction;
pub use self::user::UserDefinedFunction;

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::Type;

#[derive(Debug, Clone)]
pub enum Function {
    DebugInstruction(DebugInstructionFunction),
    AssertInstruction(AssertInstructionFunction),
    StandardLibrary(StandardLibraryFunction),
    UserDefined(UserDefinedFunction),
}

impl Function {
    pub fn new_dbg() -> Self {
        Self::DebugInstruction(DebugInstructionFunction::new())
    }

    pub fn new_assert() -> Self {
        Self::AssertInstruction(AssertInstructionFunction::new())
    }

    pub fn new_std(identifier: BuiltinIdentifier) -> Self {
        Self::StandardLibrary(match identifier {
            BuiltinIdentifier::CryptoSha256 => StandardLibraryFunction::new_sha256(),
            BuiltinIdentifier::CryptoPedersen => StandardLibraryFunction::new_pedersen(),
            BuiltinIdentifier::UnsignedFromBits => StandardLibraryFunction::new_from_bits(),
            BuiltinIdentifier::ToBits => StandardLibraryFunction::new_to_bits(),
            function => unimplemented!("function `{}` is not implemented", function)
        })
    }

    pub fn new_user_defined(
        identifier: String,
        arguments: Vec<(String, Type)>,
        return_type: Type,
    ) -> Self {
        Self::UserDefined(UserDefinedFunction::new(identifier, arguments, return_type))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DebugInstruction(inner) => write!(f, "{}", inner),
            Self::AssertInstruction(inner) => write!(f, "{}", inner),
            Self::StandardLibrary(inner) => write!(f, "{}", inner),
            Self::UserDefined(inner) => write!(f, "{}", inner),
        }
    }
}
