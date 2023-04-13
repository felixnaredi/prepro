mod parse_text;
mod parse_command;

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
pub enum Chunk
{
    TextChunk(String),
    CommandChunk(String),
}

#[derive(Debug)]
pub struct Context
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
}
