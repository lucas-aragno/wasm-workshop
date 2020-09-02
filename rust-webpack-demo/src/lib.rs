#[macro_use]
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

/* ======== ONLY FOR DEBUG ==========*/
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Month {
  pub month: serde_json::value::Value,
  pub value: i32,
}

#[wasm_bindgen]
pub fn calculate_max(months: &JsValue) -> JsValue {
  console_log!("{:?} months", months);
  let months_vector : Vec<Month> = months.into_serde().unwrap();
  console_log!("{:?} vector", months_vector);
  let mut max_month : String = "".to_string();
  let mut max_value : i32 = 0;
  for element in months_vector.iter() {
    if element.value > max_value {
      max_value = element.value;
      max_month = element.month.to_string();
    }
  }
  console_log!("{:?} month max", max_month);
  JsValue::from_serde(&max_month).unwrap()
}