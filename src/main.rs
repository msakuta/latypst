use std::io::{Read, Write};

use nom::{
    branch::alt,
    character::complete::{alpha1, char, multispace0, none_of},
    multi::many0,
    IResult, Parser,
};

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let (_, elems) = elements(&s).unwrap();
    eprintln!("{s} -> {:?}", elems);
    let replaced = replace_cmd(&elems);
    let f = std::fs::File::create("out.typ").unwrap();
    let mut ofs = std::io::BufWriter::new(f);
    writeln!(ofs, "{replaced}").unwrap();
}

fn replace_cmd(elems: &[Element]) -> String {
    let mut ret = String::new();
    let mut ptr = 0;
    let put_optional_space = |ret: &mut String| {
        if !ret.chars().last().unwrap_or(' ').is_whitespace() {
            ret.push(' ');
        }
    };

    while ptr < elems.len() {
        let elem = &elems[ptr];
        match elem {
            Element::Char(c) => {
                put_optional_space(&mut ret);
                ret.push(*c);
            }
            Element::Str(s) => {
                match *s {
                    "frac" => {
                        let over = replace_brace(&elems[ptr + 1]);
                        let under = replace_brace(&elems[ptr + 2]);
                        ret += &over;
                        ret += "/";
                        ret += &under;
                        ptr += 3;
                        continue;
                    }
                    "partial" => {
                        put_optional_space(&mut ret);
                        ret += "diff";
                        ptr += 1;
                        continue;
                    }
                    _ => (),
                }
                ret += s;
            }
            Element::Brace(br) => {
                ret += "(";
                ret += &replace_cmd(&br);
                ret += ")";
            }
            Element::IMath(im) => {
                ret += "$";
                ret += &replace_cmd(im);
                ret += "$";
            }
        }
        ptr += 1;
    }
    ret
}

fn replace_brace(br: &Element) -> String {
    let Element::Brace(br) = br else { return "".to_string(); };
    let mut ret = String::new();
    ret += "(";
    ret += &replace_cmd(br);
    ret += ")";
    ret
}

fn escaped_element(i: &str) -> IResult<&str, &str> {
    let (i, _) = char('\\')(i)?;
    let (i, word) = alpha1(i)?;
    Ok((i, word))
}

#[test]
fn test_element() {
    let s = "\\hello";
    assert_eq!(escaped_element(s), Ok(("", "hello")));
}

#[derive(PartialEq, Eq, Debug)]
enum Element<'src> {
    Str(&'src str),
    Char(char),
    Brace(Vec<Element<'src>>),
    IMath(Vec<Element<'src>>),
}

fn brace(i: &str) -> IResult<&str, Element> {
    let (i, _) = char('{')(i)?;
    let (i, elems) = elements(i)?;
    let (i, _) = char('}')(i)?;
    Ok((i, Element::Brace(elems)))
}

fn inline_math(i: &str) -> IResult<&str, Element> {
    let (i, _) = char('$')(i)?;
    let (i, elems) = elements(i)?;
    let (i, _) = char('$')(i)?;
    Ok((i, Element::IMath(elems)))
}

fn any_ch(i: &str) -> IResult<&str, Element> {
    let (i, c) = none_of("}$")(i)?;
    Ok((i, Element::Char(c)))
}

fn element(i: &str) -> IResult<&str, Element> {
    let (i, _) = multispace0(i)?;
    let (i, res) = alt((
        brace,
        inline_math,
        escaped_element.map(|r| Element::Str(r)),
        any_ch,
    ))(i)?;
    Ok((i, res))
}

fn elements(i: &str) -> IResult<&str, Vec<Element>> {
    let (i, res) = many0(element)(i)?;
    Ok((i, res))
}

#[test]
fn test_elements() {
    let s = "a\\Hello \\World";
    assert_eq!(
        elements(s),
        Ok((
            "",
            vec![
                Element::Char('a'),
                Element::Str("Hello"),
                Element::Str("World")
            ]
        ))
    );
}

#[test]
fn test_imath() {
    let s = "$\\Hello \\World$";
    assert_eq!(
        elements(s),
        Ok((
            "",
            vec![Element::IMath(vec![
                Element::Str("Hello"),
                Element::Str("World")
            ])]
        ))
    );
}

#[test]
fn test_brace() {
    let s = "{a}";
    assert_eq!(
        elements(s),
        Ok(("", vec![Element::Brace(vec![Element::Char('a')])]))
    );
    let s = "\\begin{document}";
    let mut expected = vec![Element::Str("begin")];
    expected.push(Element::Brace(
        "document".chars().map(|c| Element::Char(c)).collect(),
    ));
    assert_eq!(elements(s), Ok(("", expected)));
}
