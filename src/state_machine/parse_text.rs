use super::Context;

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
    pub fn parse_text(&mut self) {}
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
        context.parse_text();
        assert!(context.chunks.is_empty());
    }

    #[test]
    fn parse_sentence()
    {
        let input = "This is a simple sentence.";
        let mut context = Context::new(input);
        context.parse_text();
        assert_eq!(context.chunks.len(), 1);
        assert!(matches!(&context.chunks[0], Chunk::TextChunk(s) if s == input));
    }

    #[test]
    fn parse_text_can_be_split_with_command()
    {
        let mut context = Context::new("Calling parse text ${twice here");
        context.parse_text();
        context.parse_text();
        assert_eq!(context.chunks.len(), 2);
        assert!(matches!(&context.chunks[0], Chunk::TextChunk(s) if s == "Calling parse text "));
        assert!(matches!(&context.chunks[1], Chunk::TextChunk(s) if s == "twice here"));
    }

    #[test]
    fn parse_text_with_dollar_signs_and_brackets()
    {
        let mut context = Context::new("$$}{$}{{$${}}$}{$");
        context.parse_text();
        assert_eq!(context.chunks.len(), 1);
        assert!(matches!(&context.chunks[0], Chunk::TextChunk(s) if s == "$$}{$}{{$"));
    }
}
