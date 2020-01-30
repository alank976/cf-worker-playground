use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestFromJs {
    pub url: String,
    pub method: String,
    pub redirect: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Foo {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<f32>>,
    pub field3: [f32; 4],
}

#[wasm_bindgen]
pub fn handle_and_log_request(val: &JsValue) -> JsValue {
    let request: RequestFromJs = val.into_serde().unwrap();
    let msg = format!("After serde, request from JS in WASM={:?}", request);
    log(&msg);

    let mut field1 = HashMap::new();
    let url_parts: Vec<&str> = request.url.split_terminator("/").collect();
    let last_part = url_parts.last().unwrap_or(&"");
    field1.insert(0, last_part.to_string());
    let example = Foo {
        field1,
        field2: vec![vec![1., 2.], vec![3., 4.]],
        field3: [1., 2., 3., 4.],
    };

    JsValue::from_serde(&example).unwrap()
}
