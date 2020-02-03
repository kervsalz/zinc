use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Lt;

impl InstructionInfo for Lt {
    fn to_assembly(&self) -> String {
        "lt".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Lt
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Lt as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::Lt((*self).clone())
    }
}