use super::{
    Context,
    StateMachine,
};

#[derive(Debug)]
enum ParseText
{
    GetChar,
    Dollar,
    Append(char),
    AppendDollar(char),
}

impl Context
{
    pub fn parse_text(&mut self) -> StateMachine
    {
        use ParseText::*;

        let mut state = GetChar;
        let mut output = String::new();

        loop {
            match state {
                GetChar => match self.read_char() {
                    Some('$') => state = Dollar,
                    Some(c) => state = Append(c),
                    None => return StateMachine::StoreTextChunk(output),
                },
                Dollar => match self.read_char() {
                    Some('{') => return StateMachine::StoreTextChunk(output),
                    Some('$') => output.push('$'),
                    Some(c) => state = AppendDollar(c),
                    None => return StateMachine::StoreTextChunk(output),
                },
                Append(c) => {
                    output.push(c);
                    state = GetChar;
                }
                AppendDollar(c) => {
                    output.push('$');
                    state = Append(c);
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
            context.parse_text(),
            StateMachine::StoreTextChunk(s) if s == "Calling parse text ",
        ));
        assert!(
            matches!(context.parse_text(), StateMachine::StoreTextChunk(s) if s == "twice here")
        );
    }

    #[test]
    fn parse_text_with_dollar_signs_and_brackets()
    {
        let mut context = Context::new("$$}{$}{{$${}}$}{$");
        let s = context.parse_text();
        assert!(matches!(s, StateMachine::StoreTextChunk(s) if s == "$$}{$}{{$"));
    }
}
