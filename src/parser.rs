use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, multispace0, none_of},
    multi::many0,
    IResult, Parser,
};
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
pub enum Element<'src> {
    Str(&'src str),
    Char(char),
    Brace(Vec<Element<'src>>),
    IMath(Vec<Element<'src>>),
    DMath(Vec<Element<'src>>),
}

fn brace(i: &str) -> IResult<&str, Element> {
    let (i, _) = char('{')(i)?;
    let (i, elems) = elements(i)?;
    let (i, _) = char('}')(i)?;
    Ok((i, Element::Brace(elems)))
}

fn display_math(i: &str) -> IResult<&str, Element> {
    let (i, _) = tag("$$")(i)?;
    let (i, elems) = math_elements(i)?;
    let (i, _) = tag("$$")(i)?;
    Ok((i, Element::DMath(elems)))
}

fn inline_math(i: &str) -> IResult<&str, Element> {
    let (i, _) = char('$')(i)?;
    let (i, elems) = math_elements(i)?;
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
        display_math, // DMath comes before IMath
        inline_math,
        escaped_element.map(|r| Element::Str(r)),
        any_ch,
    ))(i)?;
    let (i, _) = multispace0(i)?;
    Ok((i, res))
}

fn math_element(i: &str) -> IResult<&str, Element> {
    let (i, _) = multispace0(i)?;
    let (i, res) = alt((brace, escaped_element.map(|r| Element::Str(r)), any_ch))(i)?;
    let (i, _) = multispace0(i)?;
    Ok((i, res))
}

pub fn elements(i: &str) -> IResult<&str, Vec<Element>> {
    let (i, res) = many0(element)(i)?;
    Ok((i, res))
}

fn math_elements(i: &str) -> IResult<&str, Vec<Element>> {
    let (i, res) = many0(math_element)(i)?;
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
fn test_dmath() {
    let s = "$$\\Hello \\World$$";
    assert_eq!(
        elements(s),
        Ok((
            "",
            vec![Element::DMath(vec![
                Element::Str("Hello"),
                Element::Str("World")
            ])]
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
