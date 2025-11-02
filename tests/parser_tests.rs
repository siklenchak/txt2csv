use pest::Parser;
use txt2csv::{Txt2CsvParser, Rule};

#[test]
fn test_parse_bracket_fields() {
    let input = "[ID], [Name], [Age]\n1, Alice, 25";
    let pairs = Txt2CsvParser::parse(Rule::file, input)
        .expect("Failed to parse input");
    assert!(pairs.as_str().contains("[ID]"));
}