use std::fmt::Display;

#[derive(Debug)]
pub enum PreproError
{
    NoClosingBracket,
}

impl Display for PreproError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        use PreproError::*;

        match self {
            NoClosingBracket => write!(f, "no closing bracket found for command"),
        }
    }
}

impl std::error::Error for PreproError {}
