use pest::Parser;
use txt2csv::{Rule, Txt2CsvParser};

#[test]
fn test_whitespace() {
    let input = " \t";
    let pairs = Txt2CsvParser::parse(Rule::WHITESPACE, input)
        .expect("Failed to parse WHITESPACE");
    assert_eq!(pairs.as_str(), " \t");
}

#[test]
fn test_newline() {
    let input = "\n";
    let pairs = Txt2CsvParser::parse(Rule::NEWLINE, input)
        .expect("Failed to parse NEWLINE");
    assert_eq!(pairs.as_str(), "\n");
}

#[test]
fn test_comma() {
    let input = " , ";
    let pairs = Txt2CsvParser::parse(Rule::comma, input)
        .expect("Failed to parse comma");
    assert_eq!(pairs.as_str(), " , ");
}

#[test]
fn test_bracket_field() {
    let input = "[Example]";
    let pairs = Txt2CsvParser::parse(Rule::bracket_field, input)
        .expect("Failed to parse bracket_field");
    assert_eq!(pairs.as_str(), "[Example]");
}

#[test]
fn test_simple_field() {
    let input = "Hello";
    let pairs = Txt2CsvParser::parse(Rule::simple_field, input)
        .expect("Failed to parse simple_field");
    assert_eq!(pairs.as_str(), "Hello");
}

#[test]
fn test_field() {
    let input = "[Data]";
    let pairs = Txt2CsvParser::parse(Rule::field, input)
        .expect("Failed to parse field");
    assert_eq!(pairs.as_str(), "[Data]");
}

#[test]
fn test_row() {
    let input = "A, B, C";
    let pairs = Txt2CsvParser::parse(Rule::row, input)
        .expect("Failed to parse row");
    assert_eq!(pairs.as_str(), "A, B, C");
}

#[test]
fn test_file() {
    let input = "[ID], [Name], [Age]\n1, Alice, 25";
    let pairs = Txt2CsvParser::parse(Rule::file, input)
        .expect("Failed to parse file");
    assert!(pairs.as_str().contains("Alice"));
}
