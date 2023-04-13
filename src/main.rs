use crate::state_machine::Context;

mod state_machine;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let mut context = Context::new("hello");

    context.parse_text();

    println!("Hello, world!");

    Ok(())
}
