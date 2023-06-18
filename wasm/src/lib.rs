use latypst::{parse, parse_replace_rules, replace_cmd};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
}

#[wasm_bindgen]
pub fn entry(src: &str, replace_rules_str: &str) -> Result<JsValue, JsValue> {
    let parse_result =
        parse(src).map_err(|e| JsValue::from_str(&format!("Parse error: {:?}", e)))?;
    let replace_rules = parse_replace_rules(replace_rules_str)
        .map_err(|e| JsValue::from_str(&format!("Replace rules error: {:?}", e)))?;
    let ret = replace_cmd(&parse_result, &replace_rules);
    Ok(JsValue::from_str(&ret))
}

#[wasm_bindgen]
pub fn default_replace_rules() -> JsValue {
    let replace_rules = latypst::default_replace_rules();
    let mut ret = String::new();
    for rule in replace_rules {
        ret += &format!("{}/{}\n", rule.0, rule.1);
    }
    JsValue::from_str(&ret)
}
