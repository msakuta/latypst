use latypst::{parse, replace_cmd};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
}

#[wasm_bindgen]
pub fn entry(src: &str) -> Result<JsValue, JsValue> {
    let parse_result = parse(src)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {:?}", e)))?;
    let ret = replace_cmd(&parse_result);
    Ok(JsValue::from_str(&ret))
}
