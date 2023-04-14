use super::{
    Context,
    StateMachine,
};

#[derive(Debug)]
enum ParseText
{
    GetChar,
    Dollar,
    LeftBracket,
    Append(char),
    AppendDollar(char),
}

impl Context
{
    pub fn parse_text(&mut self) -> StateMachine
    {
        StateMachine::Done
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use crate::state_machine::Chunk;

    #[test]
    fn parse_empty()
    {
        let mut context = Context::new("");
        assert!(matches!(context.parse_text(), StateMachine::StoreTextChunk(s) if s == ""));
    }

    #[test]
    fn parse_sentence()
    {
        let input = "This is a simple sentence.";
        let mut context = Context::new(input);
        assert!(matches!(context.parse_text(), StateMachine::StoreTextChunk(s) if s == input));
    }

    #[test]
    fn parse_text_can_be_split_with_command()
    {
        let mut context = Context::new("Calling parse text ${twice here");
        assert!(matches!(
            &context.parse_text(),
            StateMachine::StoreTextChunk(s) if s == "Calling parse text ",
        ));
        assert!(
            matches!(&context.parse_text(), StateMachine::StoreTextChunk(s) if s == "twice here")
        );
    }

    #[test]
    fn parse_text_with_dollar_signs_and_brackets()
    {
        let mut context = Context::new("$$}{$}{{$${}}$}{$");
        assert!(
            matches!(context.parse_text(), StateMachine::StoreTextChunk(s) if s == "$$}{$}{{$")
        );
    }
}
