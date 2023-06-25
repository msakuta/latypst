mod parser;
mod replace;

pub use self::{
    parser::{elements, Element},
    replace::{default_replace_rules, replace_cmd},
};

use std::io::{Read, Write};

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let (_, elems) = elements(&s).unwrap();
    let replaced = replace_cmd(&elems, default_replace_rules());
    let f = std::fs::File::create("out.typ").unwrap();
    let mut ofs = std::io::BufWriter::new(f);
    writeln!(ofs, "{replaced}").unwrap();
}
