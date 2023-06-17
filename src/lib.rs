mod parser;
mod replace;

use self::{
    parser::{elements, Element},
};

pub use replace::replace_cmd;

use nom::Finish;

pub fn parse<'src>(s: &'src str) -> Result<Vec<Element<'src>>, Box<dyn std::error::Error + 'src>> {
    let (_, res) = elements(s).finish()?;
    Ok(res)
}