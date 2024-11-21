
# Board Game Parser

## Overview

The **Board Game Parser** is a Rust-based utility designed to parse structured information about board games and convert it into a serialized JSON format. This tool processes text files containing data about board games and extracts details such as the game's name, author, age suitability, playtime, player count, and price.

### Links

- [Documentation on docs.rs](https://docs.rs/board_game_parser/0.1.0/board_game_parser/)
- [Crate on crates.io](https://crates.io/crates/board_game_parser)

## Features

- Parses text files with structured board game data.
- Outputs data as a JSON file.
- Supports flexible input with various whitespace formatting.

## Parsing Details

### Input Format
The parser expects an input file with the following structure:
```
Name: <game name>
Author: <author name>
Age: <minimum age>
Time: <minimum time>-<maximum time>
Players: <minimum players>-<maximum players>
Price: <price> <currency>
```

- **`Name:`** The name of the board game.
- **`Author:`** The creator or publisher of the game.
- **`Age:`** Minimum age requirement for the game.
- **`Time:`** Expected playtime, with optional range (e.g., `30-60` minutes).
- **`Players:`** Minimum and maximum player count.
- **`Price:`** Cost of the game, with optional currency (`UAH`, `USD`, `EUR`).

Example:
```
Name: Chess
Author: Unknown
Age: 6
Time: 10-60
Players: 2
Price: 20 USD
```

### Parsing Process
1. **Grammar Definition**: 
   The grammar is defined in `grammar.pest` using the [Pest parser](https://pest.rs/). It uses rules for:
   - Capturing sections such as `Name`, `Author`, `Age`, etc.
   - Handling optional whitespace and ranges.

2. **Data Extraction**: 
   The parser tokenizes input based on these rules and extracts relevant fields. Each game entry is represented as a `Pair` object in Pest, which is then converted to a `Game` struct using the `from_pair` method.

3. **Serialization**: 
   The extracted data is serialized into JSON format using the [Serde](https://serde.rs/) library.

### Output Format
The resulting JSON file contains an array of board games, each represented as an object with the following structure:
```json
[
  {
    "name": "Chess",
    "author": "Unknown",
    "age": 6,
    "min_time": 10,
    "max_time": 60,
    "min_players": 2,
    "max_players": null,
    "price": 20.0
  }
]
```

## Usage

### Commands
1. Parse a file:
   ```bash
   cargo run -- <input_file> <output_file>
   ```
   Example:
   ```bash
   cargo run input.txt output.json
   ```

2. Display credits:
   ```bash
   cargo run -- --credits
   ```

3. Display help:
   ```bash
   cargo run -- --help
   ```

### Examples
- Input file:
  ```
  Name: Catan
  Author: Klaus Teuber
  Age: 10
  Time: 60-120
  Players: 3-4
  Price: 30 USD
  ```

- Command:
  ```bash
  cargo run input.txt output.json
  ```

- Output file (`output.json`):
  ```json
  [
    {
      "name": "Catan",
      "author": "Klaus Teuber",
      "age": 10,
      "min_time": 60,
      "max_time": 120,
      "min_players": 3,
      "max_players": 4,
      "price": 30.0
    }
  ]
  ```

## Technical Details
This project leverages:
- **Pest**: For defining the parsing grammar and tokenizing input.
- **Serde**: For serialization and deserialization of parsed data.
- **Anyhow**: For improved error handling.

## Author
- Maksym Karpinskyi (karpinskyimaksym@gmail.com)

## License
MIT License
