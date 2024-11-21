use anyhow::{anyhow, Result};
use board_game_parser::*;
use pest::Parser;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return Ok(());
    }

    match args[1].as_str() {
        "--help" => help(),
        "--credits" => {
            println!(
                "\
            Board Game Parser - Credits

            Author:
                - Maksym Karpinskyi (karpinskyimaksym@gmail.com)
            "
            );
        }

        _ => {
            if args.len() < 3 {
                eprintln!("Error: Missing output file path.");
                help();
                return Ok(());
            }
            let input_p = &args[1];
            let output_p = &args[2];
            parse_file(input_p, output_p)?;
        }
    }

    Ok(())
}

fn parse_file(input_p: &str, output_p: &str) -> Result<()> {
    let mut input = String::new();
    File::open(input_p)?.read_to_string(&mut input)?;

    let pairs = Grammar::parse(Rule::games, &input)
        .map_err(|e| anyhow!("Parsing failed: {}", e))?
        .next()
        .ok_or_else(|| anyhow!("No games found in input file"))?;

    let games: Vec<Game> = pairs.into_inner().map(Game::from_pair).collect();

    let json_output = serde_json::to_string_pretty(&games)
        .map_err(|e| anyhow!("Failed to serialize data to JSON: {}", e))?;

    let mut output_file = File::create(output_p)?;
    output_file.write_all(json_output.as_bytes())?;

    println!("Parsed data has been written to {}", output_p);

    Ok(())
}

fn help() {
    println!(
        "\
    Board Game Parser - A tool for processing board game data.

    Usage:
        cargo run -- <input_file> <output_file>   Parses the <input_file> and writes the result to <output_file>.
        cargo run -- --credits                   Displays information about the authors of this tool.
        cargo run -- --help                      Displays this help message.

    Examples:
        cargo run input.txt output.json          Parses 'input.txt' and writes the result to 'output.json'.
    "
    );
}
