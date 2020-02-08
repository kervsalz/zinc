//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let value = [1, 2, 3][true];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Value(
            ValueError::OperatorIndexSecondOperandExpectedIntegerOrRange(
                Constant::Boolean(true).to_string(),
            ),
        ),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
