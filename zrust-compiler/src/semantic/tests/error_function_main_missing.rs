//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::semantic::Analyzer;
use crate::semantic::Error as SemanticError;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn another() -> u8 {
    42
}
"#;

    let expected: Result<Vec<u8>, Error> = Err(Error::Semantic(SemanticError::FunctionMainMissing));

    let result = Analyzer::default()
        .compile(
            Parser::default()
                .parse(input.to_owned())
                .expect("Syntax error"),
        )
        .map(|instructions| {
            instructions
                .into_iter()
                .map(|instruction| instruction.encode())
                .flatten()
                .collect::<Vec<u8>>()
        });

    assert_eq!(expected, result);
}
