
use anyhow::{Context};
use board_game_parser::{Grammar, Rule};
use pest::Parser;

#[test]
fn test_whitespace_parsing() -> anyhow::Result<()> {
    let input = "\t";
    let pair = Grammar::parse(Rule::WHITESPACE, input)
        .context("Failed to parse WHITESPACE")?
        .next()
        .unwrap();
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn test_space_parsing() -> anyhow::Result<()> {
    let input = " \t \t ";
    let pair = Grammar::parse(Rule::SPACE, input)
        .context("Failed to parse SPACE")?
        .next()
        .unwrap();
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn test_number_parsing() -> anyhow::Result<()> {
    let inputs = vec!["42", "-42", "3.14", "-3.14"];
    for input in inputs {
        let pair = Grammar::parse(Rule::number, input)
            .context(format!("Failed to parse number: {}", input))?
            .next()
            .unwrap();
        assert_eq!(pair.as_str(), input);
    }
    Ok(())
}

#[test]
fn test_currency_parsing() -> anyhow::Result<()> {
    let inputs = vec!["UAH", "EUR", "USD"];
    for input in inputs {
        let pair = Grammar::parse(Rule::currency, input)
            .context(format!("Failed to parse currency: {}", input))?
            .next()
            .unwrap();
        assert_eq!(pair.as_str(), input);
    }
    Ok(())
}

#[test]
fn test_any_text_parsing() -> anyhow::Result<()> {
    let input = "This is a line of text\n";
    let pair = Grammar::parse(Rule::any_text, input)
        .context("Failed to parse any_text")?
        .next()
        .unwrap();
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn test_name_parsing() -> anyhow::Result<()> {
    let input = "Name: Chess\n";
    let pair = Grammar::parse(Rule::name, input)
        .context("Failed to parse name")?
        .next()
        .unwrap();
    assert_eq!(pair.as_str(), "Name: Chess\n");
    Ok(())
}

#[test]
fn test_author_parsing() -> anyhow::Result<()> {
    let input = "Author: John Doe\n";
    let pair = Grammar::parse(Rule::author, input)
        .context("Failed to parse author")?
        .next()
        .unwrap();
    assert_eq!(pair.as_str(), "Author: John Doe\n");
    Ok(())
}

#[test]
fn test_age_parsing() -> anyhow::Result<()> {
    let input = "Age: 10\n";
    let pair = Grammar::parse(Rule::age, input)
        .context("Failed to parse age")?
        .next()
        .unwrap();
    assert_eq!(pair.as_str(), "Age: 10\n");
    Ok(())
}

#[test]
fn test_time_parsing() -> anyhow::Result<()> {
    let input = "Time: 30-60\n";
    let pair = Grammar::parse(Rule::time, input)
        .context("Failed to parse time")?
        .next()
        .unwrap();
    assert_eq!(pair.as_str(), "Time: 30-60\n");
    Ok(())
}

#[test]
fn test_players_parsing() -> anyhow::Result<()> {
    let input = "Players: 2-4\n";
    let pair = Grammar::parse(Rule::players, input)
        .context("Failed to parse players")?
        .next()
        .unwrap();
    assert_eq!(pair.as_str(), "Players: 2-4\n");
    Ok(())
}

#[test]
fn test_price_parsing() -> anyhow::Result<()> {
    let input = "Price: 30 USD\n";
    let pair = Grammar::parse(Rule::price, input)
        .context("Failed to parse price")?
        .next()
        .unwrap();
    assert_eq!(pair.as_str(), "Price: 30 USD\n");
    Ok(())
}

#[test]
fn test_game_parsing() -> anyhow::Result<()> {
    let input = "Name: Chess\nAuthor: John Doe\nAge: 10\nTime: 30-60\nPlayers: 2-4\nPrice: 30 USD\n";
    let pair = Grammar::parse(Rule::game, input)
        .context("Failed to parse game")?
        .next()
        .unwrap();
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn test_games_parsing() -> anyhow::Result<()> {
    let input = "\
Name: Monopoly
Author: Charles Darrow
Age: 8
Time: 30-90
Players: 2-8
Price: 499.99 UAH

Name: Settlers of Catan
Author: Klaus Teuber
Age: 10
Time: 60
Players: 3-4
Price: 999.99 UAH

Name: Carcassonne
Author: Klaus-Jürgen Wrede
Age: 7
Time: 35
Players: 2-5
Price: 899.99 UAH

";
    let pair = Grammar::parse(Rule::games, input)
        .context("Failed to parse games")?
        .next()
        .unwrap();
    assert_eq!(pair.as_str(), input);
    Ok(())
}
