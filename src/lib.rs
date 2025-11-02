use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Txt2CsvParser;

pub use self::Rule as Txt2CsvRule;