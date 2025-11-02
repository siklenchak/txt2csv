use pest::Parser;
use txt2csv::{Rule, Txt2CsvParser};

#[test]
fn test_parse_bracket_fields() {
    let input = "[ID], [Name], [Age]\n1, Alice, 25";
    let pairs = Txt2CsvParser::parse(Rule::file, input).expect("Failed to parse input");
    assert!(pairs.as_str().contains("[ID]"));
}

#[test]
fn test_bracket_field() {
    let input = "[Test]";
    let pairs = Txt2CsvParser::parse(Rule::bracket_field, input).unwrap();
    assert_eq!(pairs.as_str(), "[Test]");
}

#[test]
fn test_simple_field() {
    let input = "Hello";
    let pairs = Txt2CsvParser::parse(Rule::simple_field, input).unwrap();
    assert_eq!(pairs.as_str(), "Hello");
}