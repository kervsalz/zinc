//!
//! The instructions.
//!

pub mod assert;
pub mod call_std;
pub mod contract_storage;
pub mod data_stack;
pub mod dbg;
pub mod evaluation_stack;
pub mod flow;
pub mod markers;
pub mod noop;
pub mod operators;
pub mod state;

use zinc_build::Instruction;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;

pub trait IExecutable<VM: IVirtualMachine> {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError>;
}

impl<VM: IVirtualMachine> IExecutable<VM> for Instruction {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        match self {
            Self::NoOperation(inner) => inner.execute(vm),

            Self::Push(inner) => inner.execute(vm),
            Self::Slice(inner) => inner.execute(vm),
            Self::Copy(inner) => inner.execute(vm),

            Self::Load(inner) => inner.execute(vm),
            Self::LoadByIndex(inner) => inner.execute(vm),
            Self::Store(inner) => inner.execute(vm),
            Self::StoreByIndex(inner) => inner.execute(vm),

            Self::StorageStore(inner) => inner.execute(vm),
            Self::StorageLoad(inner) => inner.execute(vm),

            Self::Add(inner) => inner.execute(vm),
            Self::Sub(inner) => inner.execute(vm),
            Self::Mul(inner) => inner.execute(vm),
            Self::Div(inner) => inner.execute(vm),
            Self::Rem(inner) => inner.execute(vm),
            Self::Neg(inner) => inner.execute(vm),

            Self::Not(inner) => inner.execute(vm),
            Self::And(inner) => inner.execute(vm),
            Self::Or(inner) => inner.execute(vm),
            Self::Xor(inner) => inner.execute(vm),

            Self::Lt(inner) => inner.execute(vm),
            Self::Le(inner) => inner.execute(vm),
            Self::Eq(inner) => inner.execute(vm),
            Self::Ne(inner) => inner.execute(vm),
            Self::Ge(inner) => inner.execute(vm),
            Self::Gt(inner) => inner.execute(vm),

            Self::BitwiseShiftLeft(inner) => inner.execute(vm),
            Self::BitwiseShiftRight(inner) => inner.execute(vm),
            Self::BitwiseAnd(inner) => inner.execute(vm),
            Self::BitwiseOr(inner) => inner.execute(vm),
            Self::BitwiseXor(inner) => inner.execute(vm),
            Self::BitwiseNot(inner) => inner.execute(vm),

            Self::Cast(inner) => inner.execute(vm),

            Self::If(inner) => inner.execute(vm),
            Self::Else(inner) => inner.execute(vm),
            Self::EndIf(inner) => inner.execute(vm),
            Self::LoopBegin(inner) => inner.execute(vm),
            Self::LoopEnd(inner) => inner.execute(vm),
            Self::Call(inner) => inner.execute(vm),
            Self::Return(inner) => inner.execute(vm),
            Self::Exit(inner) => inner.execute(vm),

            Self::CallStd(inner) => inner.execute(vm),
            Self::Assert(inner) => inner.execute(vm),
            Self::Dbg(inner) => inner.execute(vm),

            Self::SetUnconstrained(inner) => inner.execute(vm),
            Self::UnsetUnconstrained(inner) => inner.execute(vm),

            Self::FileMarker(inner) => inner.execute(vm),
            Self::FunctionMarker(inner) => inner.execute(vm),
            Self::LineMarker(inner) => inner.execute(vm),
            Self::ColumnMarker(inner) => inner.execute(vm),
        }
    }
}
