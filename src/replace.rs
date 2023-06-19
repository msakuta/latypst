use crate::Element;
use regex::Regex;
use std::sync::OnceLock;

static REPLACE_RULES: OnceLock<Vec<(Regex, String)>> = OnceLock::new();

pub fn default_replace_rules() -> &'static [(Regex, String)] {
    REPLACE_RULES.get_or_init(|| {
        vec![
            (Regex::new("notag").unwrap(), "".to_string()),
            (Regex::new("partial").unwrap(), "diff".to_string()),
            (Regex::new("varepsilon").unwrap(), "epsilon".to_string()),
            (Regex::new("int").unwrap(), "integral".to_string()),
            (Regex::new("vec").unwrap(), "arrow".to_string()),
            (Regex::new("ddot").unwrap(), "dot.double".to_string()),
            (Regex::new("cdots").unwrap(), "dots.c".to_string()),
            (Regex::new("vdots").unwrap(), "dots.v".to_string()),
            (Regex::new("hdots").unwrap(), "dots.h".to_string()),
            (Regex::new("ddots").unwrap(), "dots.down".to_string()),
            (Regex::new("langle").unwrap(), "angle.l".to_string()),
            (Regex::new("rangle").unwrap(), "angle.r".to_string()),
        ]
    })
}

pub fn replace_cmd(elems: &[Element], replace_rules: &[(Regex, String)]) -> String {
    let mut ret = String::new();
    let mut ptr = 0;
    let put_optional_space = |ret: &mut String| {
        if !ret.chars().last().unwrap_or(' ').is_whitespace() {
            ret.push(' ');
        }
    };
    let mut env_stack = vec![];

    'next_cmd: while ptr < elems.len() {
        let elem = &elems[ptr];
        match elem {
            Element::Char(c) => {
                put_optional_space(&mut ret);
                ret.push(*c);
            }
            Element::Str(s) => {
                match *s {
                    "frac" => {
                        let over = replace_brace(&elems[ptr + 1], replace_rules);
                        let under = replace_brace(&elems[ptr + 2], replace_rules);
                        ret += &over;
                        ret += "/";
                        ret += &under;
                        ptr += 3;
                        continue;
                    }
                    "mathcal" => {
                        put_optional_space(&mut ret);
                        ret += "cal";
                        ret += &replace_brace(&elems[ptr + 1], replace_rules);
                        ptr += 2;
                        continue;
                    }
                    "left" | "right" => {
                        ptr += 1;
                        continue;
                    }
                    "begin" => {
                        let env = &elems[ptr + 1];
                        if is_math_env(env) {
                            ret += "$ ";
                        }
                        env_stack.push(env);
                        ptr += 2;
                        continue;
                    }
                    "end" => {
                        if is_math_env(&elems[ptr + 1]) {
                            ret += " $\n";
                        }
                        env_stack.pop();
                        ptr += 2;
                        continue;
                    }
                    "label" => {
                        ptr += 2;
                        continue;
                    }
                    _ => {
                        for (rule, replacer) in replace_rules.iter() {
                            if rule.is_match(s) {
                                put_optional_space(&mut ret);
                                ret += replacer;
                                ptr += 1;
                                continue 'next_cmd;
                            }
                        }
                    }
                }
                put_optional_space(&mut ret);
                ret += s;
            }
            Element::Brace(br) => {
                ret += "(";
                ret += &replace_cmd(&br, replace_rules);
                ret += ")";
            }
            Element::IMath(im) => {
                ret += "$";
                ret += &replace_cmd(im, replace_rules);
                ret += "$\n";
            }
            Element::DMath(im) => {
                ret += "$ ";
                ret += &replace_cmd(im, replace_rules);
                ret += " $\n";
            }
        }
        ptr += 1;
    }
    ret
}

fn is_math_env(elem: &Element) -> bool {
    match elem {
        Element::Str(env) => {
            matches!(&env as &str, "align" | "align*" | "equation" | "eqnarray")
        }
        Element::Brace(br) => {
            let s: String = br
                .iter()
                .map(|e| match e {
                    Element::Char(c) => *c,
                    _ => ' ',
                })
                .collect();
            matches!(&s as &str, "align" | "align*" | "equation" | "eqnarray")
        }
        _ => false,
    }
}

fn replace_brace(br: &Element, replace_rules: &[(Regex, String)]) -> String {
    let Element::Brace(br) = br else { return "".to_string(); };
    let mut ret = String::new();
    ret += "(";
    ret += &replace_cmd(br, replace_rules);
    ret += ")";
    ret
}
