use super::{Context, StateMachine};

#[derive(Debug)]
enum ParseCommand
{
    GetChar,
    RightBracket,
    DecrementDepth,
    IncrementDepth,
    Append(char),
}

impl Context
{
    pub fn parse_command(&mut self) -> StateMachine
    {
        use ParseCommand::*;

        let mut state = GetChar;
        let mut output = String::new();
        let mut depth = 0;

        loop {
            match state {
                GetChar => match self.read_char() {
                    Some('{') => state = IncrementDepth,
                    Some('}') => state = RightBracket,
                    Some(c) => state = Append(c),
                    None => return StateMachine::NoClosingBracket,
                },
                RightBracket => {
                    if depth > 0 {
                        state = DecrementDepth;
                    } else {
                        return StateMachine::StoreCommandChunk(output);
                    }
                },
                DecrementDepth => {
                    depth -= 1;
                    state = Append('}');
                },
                IncrementDepth => {
                    depth += 1;
                    state = Append('{');
                },
                Append(c) => {
                    output.push(c);
                    state = GetChar;
                }
            }
        }
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
