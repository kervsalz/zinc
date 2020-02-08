//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let a = 0;
    a .. 42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 7),
        ElementError::OperatorRangeFirstOperandExpectedConstant(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
