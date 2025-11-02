use anyhow::Result;
use clap::{Parser, Subcommand}; 
use pest::Parser as PestParser; 
use std::fs;
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
        input: String,
        #[arg(short, long, default_value = "output.csv")]
        output: String,
    },
    Credits,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { input, output } => {
            let txt = fs::read_to_string(&input)?;
            let pairs = Txt2CsvParser::parse(Rule::file, &txt)?;
            let csv = to_csv(pairs);
            fs::write(&output, csv)?;
            println!("✅ Converted {input} → {output}");
        }
        Commands::Credits => {
            println!("txt2csv © 2025");
            println!("Developed by Your Name");
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