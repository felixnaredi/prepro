use super::{Context, StateMachine};

#[derive(Debug)]
enum ParseCommand
{
    GetChar,
    IncrementDepth,
    DecrementDepth,
    Append(char),
    RightBracket,
}

impl Context
{
    pub fn parse_command(&mut self) -> StateMachine
    {
        StateMachine::Done
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn parse_empty_command()
    {
        let mut context = Context::new("}");
        assert!(matches!(context.parse_command(), StateMachine::StoreCommandChunk(s) if s == ""));
    }

    #[test]
    fn parse_simple_command()
    {
        let mut context = Context::new("cat some-file.txt }");
        assert!(matches!(
            context.parse_command(), 
            StateMachine::StoreCommandChunk(s) if s == "cat some-file.txt "),
        );
    }

    #[test]
    fn parse_command_missing_closing_bracket()
    {
        let mut context = Context::new("this has no closing bracket!!");
        assert!(matches!(context.parse_command(), StateMachine::NoClosingBracket));
    }

    #[test]
    fn parse_command_containing_brackets()
    {
        let mut context = Context::new("echo {some} aaa{a{  b}zz } qqq}");
        assert!(matches!(
            context.parse_command(),
            StateMachine::StoreCommandChunk(s) if s == "echo {some} aaa{a{  b}zz } qqq"),
        );
    }

    fn parse_command_containing_brackets_with_no_closing()
    {
        let mut context = Context::new("find {aa{ ii} a{bbbb } } {uuuu}");
        assert!(matches!(context.parse_command(), StateMachine::NoClosingBracket));
        assert!(context.chunks.is_empty());
    }
}
