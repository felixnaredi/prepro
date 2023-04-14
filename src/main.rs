mod state_machine;

use crate::state_machine::parse_prepro;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    println!("{}", parse_prepro("hello ${ echo world }")?);

    Ok(())
}
