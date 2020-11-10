use franklin_crypto::bellman::pairing::ff::PrimeField;

use zinc_build::ScalarType;

use crate::error::Error;
use crate::IEngine;

pub trait ITypeExpectation: Sized {
    fn expect_same(left: Self, right: Self) -> Result<Self, Error>;

    fn assert_type(&self, expected: Self) -> Result<(), Error>;

    fn assert_signed(&self, is_signed: bool) -> Result<(), Error>;

    fn bitlength<E: IEngine>(&self) -> usize;
}

impl ITypeExpectation for ScalarType {
    fn expect_same(left: Self, right: Self) -> Result<Self, Error> {
        if left != right {
            return Err(Error::TypeError {
                expected: left.to_string(),
                found: right.to_string(),
            });
        }

        Ok(left)
    }

    fn assert_type(&self, expected: Self) -> Result<(), Error> {
        if self != &expected {
            return Err(Error::TypeError {
                expected: expected.to_string(),
                found: self.to_string(),
            });
        }

        Ok(())
    }

    fn assert_signed(&self, is_signed: bool) -> Result<(), Error> {
        let is_signed = match self {
            ScalarType::Field | ScalarType::Boolean => false,
            ScalarType::Integer(int_type) => int_type.is_signed == is_signed,
        };

        if !is_signed {
            return Err(Error::TypeError {
                expected: if is_signed {
                    "signed integer".to_owned()
                } else {
                    "unsigned integer".to_owned()
                },
                found: self.to_string(),
            });
        }

        Ok(())
    }

    fn bitlength<E: IEngine>(&self) -> usize {
        match self {
            ScalarType::Boolean => 1,
            ScalarType::Integer(inner) => inner.bitlength,
            ScalarType::Field => E::Fr::NUM_BITS as usize,
        }
    }
}
