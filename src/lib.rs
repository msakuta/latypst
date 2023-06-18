mod parser;
mod replace;

use std::error::Error;

use self::parser::{elements, Element};

use regex::Regex;
pub use replace::{default_replace_rules, replace_cmd};

use nom::Finish;

pub fn parse<'src>(s: &'src str) -> Result<Vec<Element<'src>>, Box<dyn Error + 'src>> {
    let (_, res) = elements(s).finish()?;
    Ok(res)
}

pub fn parse_replace_rules(s: &str) -> Result<Vec<(Regex, String)>, Box<dyn Error>> {
    let mut replace_rules = vec![];
    for line in s.split("\n") {
        let mut sections = line.split("/");
        if let (Some(from), Some(to)) = (sections.next(), sections.next()) {
            replace_rules.push((Regex::new(from)?, to.to_owned()));
        }
    }
    Ok(replace_rules)
}
