use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use pest::Parser as PestParser;
use std::fs;
use std::io::{self, Write};
use txt2csv::{Txt2CsvParser, Txt2CsvRule as Rule};

#[derive(Parser)]
#[command(name = "txt2csv", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        input: Option<String>,

        #[arg(short, long, default_value = "output.csv")]
        output: String,
    },

    Credits,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { input, output } => {
            let input = match input {
                Some(path) => path,
                None => {
                    print!("Enter the name of the .txt file to parse: ");
                    io::stdout().flush()?; 
                    let mut filename = String::new();
                    io::stdin().read_line(&mut filename)?;
                    filename.trim().to_string()
                }
            };

            if !input.ends_with(".txt") {
                return Err(anyhow!("Please provide a file with .txt extension"));
            }

            if !std::path::Path::new(&input).exists() {
                return Err(anyhow!("File not found: {input}"));
            }

            let txt = fs::read_to_string(&input)?;
            let pairs = Txt2CsvParser::parse(Rule::file, &txt)?;
            let csv = to_csv(pairs);

            fs::write(&output, csv)?;
            println!("Converted {input} → {output}");
        }

        Commands::Credits => {
            println!("txt2csv © 2025");
            println!("Developed by siklenchak");
            println!("Built with pest + clap + anyhow");
        }
    }

    Ok(())
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

