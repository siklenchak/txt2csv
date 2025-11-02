use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Txt2CsvParser;

pub use self::Rule as Txt2CsvRule;

#[derive(Debug, Error)]
pub enum Txt2CsvError {
    #[error("File error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] pest::error::Error<Rule>),
}
