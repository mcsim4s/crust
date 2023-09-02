use crate::util::*;
use std::io::Result;

pub enum Position {
    Start,
    Fen(String),
}

pub struct Move {
    from: String,
    to: String,
}

pub struct GoCommand {
    white_time: u64,
    black_time: u64,
}

pub enum Command {
    Uci,
    IsReady,
    NewGame,
    SetPosition {
        position: Position,
        moves: Vec<Move>,
    },
    Go(GoCommand),
}

impl Command {
    pub fn parse(raw: &str) -> Result<Command> {
        let mut split: std::str::SplitWhitespace<'_> = raw.split_whitespace();
        match split
            .next()
            .ok_or(errors::invalid_input(format!("Unexpected empty uci input")))?
        {
            "uci" => Result::Ok(Command::Uci),
            "isready" => Result::Ok(Command::IsReady),
            "ucinewgame" => Result::Ok(Command::NewGame),
            "position" => parse_position_command(split),
            "go" => parse_go_command(split),
            other => Result::Err(errors::invalid_input(format!(
                "Unexpected uci input: '{}'",
                other
            ))),
        }
    }
}

fn parse_position_command(mut split: std::str::SplitWhitespace<'_>) -> Result<Command> {
    let position: Position = match split.next().ok_or(errors::invalid_input(format!(
        "Unexpected empty input after 'position'"
    )))? {
        "fen" => todo!(),
        "startpos" => Position::Start,
        other => {
            return Result::Err(errors::invalid_input(format!(
                "Unexpected input after 'position': '{other}'"
            )))
        }
    };
    let moves: Vec<Move> = Vec::new();
    Result::Ok(Command::SetPosition { position, moves })
}

fn parse_go_command(mut split: std::str::SplitWhitespace<'_>) -> Result<Command> {
    let mut result = GoCommand {
        white_time: 0,
        black_time: 0,
    };
    while let Some(arg) = split.next() {
        match arg {
            "wtime" => {
                result = GoCommand {
                    white_time: parse_time(&mut split)?,
                    ..result
                }
            }
            _ => (),
        }
    }
    Result::Ok(Command::Go(result))
}

fn parse_time(source: &mut std::str::SplitWhitespace<'_>) -> Result<u64> {
    let time_string = source.next().ok_or(errors::invalid_input(format!(
        "Unexpected time input after 'wtime'"
    )))?;
    let time_u64: u64 = time_string
        .parse()
        .or(Result::Err(errors::invalid_input(format!(
            "wtime was not a number but '{time_string}'"
        ))))?;
    Ok(time_u64)
}
