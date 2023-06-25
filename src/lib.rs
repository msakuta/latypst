mod parser;
mod replace;

use std::error::Error;

use self::parser::{elements, Element};

pub use replace::{default_replace_rules, parse_replace_rules, replace_cmd};

use nom::Finish;

pub fn parse<'src>(s: &'src str) -> Result<Vec<Element<'src>>, Box<dyn Error + 'src>> {
    let (_, res) = elements(s).finish()?;
    Ok(res)
}
