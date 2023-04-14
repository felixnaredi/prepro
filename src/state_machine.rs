mod error;
mod parse_command;
mod parse_text;

use std::sync::mpsc::Receiver;

use error::PreproError;

#[derive(Debug)]
enum StateMachine
{
    ParseText,
    StoreTextChunk(String),
    ParseCommand,
    StoreCommandChunk(String),
    Done,
    NoClosingBracket,
}

#[derive(Debug)]
enum Chunk
{
    Text(String),
    Command(Receiver<String>),
}

#[derive(Debug)]
struct Context
{
    input: Vec<char>,
    chunks: Vec<Chunk>,
}

impl Context
{
    pub fn new(input: &str) -> Context
    {
        Context {
            input: input.chars().rev().collect(),
            chunks: Vec::new(),
        }
    }

    fn read_char(&mut self) -> Option<char>
    {
        self.input.pop()
    }

    fn step(&mut self, state: StateMachine) -> StateMachine
    {
        StateMachine::Done
    }

    fn collapse(self) -> Result<String, PreproError>
    {
        if !self.input.is_empty() {
            panic!("input is not empty");
        }

        let mut out = String::new();

        for chunk in self.chunks {
            match chunk {
                Chunk::Text(s) => out.push_str(&s),
                Chunk::Command(receiver) => out.push_str(&receiver.recv().unwrap()),
            };
        }

        Ok(out)
    }
}

pub fn parse_prepro(input: &str) -> Result<String, Box<dyn std::error::Error>>
{
    let context = Context::new(input);

    Ok("".into())
}
