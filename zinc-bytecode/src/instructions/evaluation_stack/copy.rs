//!
//! The 'evaluation stack copy' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

/// Copies the top element from the stack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Copy;

impl Copy {
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for Copy {
    fn into(self) -> Instruction {
        Instruction::Copy(self)
    }
}

impl fmt::Display for Copy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "copy",)
    }
}