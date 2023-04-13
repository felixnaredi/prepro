use super::Context;

#[derive(Debug)]
enum ParseCommand
{
    GetChar,
    IncrementDepth,
    DecrementDepth,
    Append(char),
    RightBracket,
}

impl Context {
    pub fn parse_command(&mut self)
    {}
}