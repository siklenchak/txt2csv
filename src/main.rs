use pest::Parser;
use txt2csv::{Txt2CsvParser, Txt2CsvRule as Rule};

fn main() {
    let input = "[ID], [Name], [Age]\n1, Alice, 25\n2, Bob, 30";

    match Txt2CsvParser::parse(Rule::file, input) {
        Ok(pairs) => {
            println!("Parsed successfully!\n");

            let csv = to_csv(pairs);
            println!("{}", csv);
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}

fn to_csv(pairs: pest::iterators::Pairs<Rule>) -> String {
    let mut lines = Vec::new();

    for pair in pairs.flatten() {
        if pair.as_rule() == Rule::row {
            let mut fields = Vec::new();

            for inner in pair.into_inner() {
                if inner.as_rule() == Rule::field {
                    let mut text = inner.as_str().trim().to_string();
                    text = text.trim_matches('[').trim_matches(']').to_string();
                    fields.push(text);
                }
            }

            lines.push(fields.join(",")); 
        }
    }

    lines.join("\n")
}