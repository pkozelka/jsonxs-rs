use std::collections::HashMap;
use std::io::Result;

use jsonxs::{JsonXsSerializer, JsonXsValue};

pub fn json_save(map: &HashMap<String, String>) -> Result<()> {
    // let mut json = JsonXsSerializer::use_file("hello.json")?;
    let mut json = JsonXsSerializer::use_stdout();
    json.open_obj(JsonXsValue::NA)?; // opens root object, "{"

    json.open_obj("map")?;
    for (k, v) in map {
        json.element(k, v)?;
    }
    json.close()?;

    json.close()?; // closes root object, "}"
    json.done() // checks that nesting went well
}

fn main() {
    let mut map = HashMap::new();
    map.insert("hello".to_string(), "world".to_string());
    map.insert("how".to_string(), "are you".to_string());

    json_save(&map).unwrap();
}
