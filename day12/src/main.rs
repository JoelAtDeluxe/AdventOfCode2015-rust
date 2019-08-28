use serde_json::{Result, Value};

fn main() {
    let path = "input.json";
    let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));

    let parsed_json = parse_json(&data);

    match parsed_json {
        Ok(v) => {
            let sum = evaluate_json(&v, false).0;
            println!("total sum is: {}", sum);
            let non_red_sum = evaluate_json(&v, true).0;
            println!("revised sum, after ignoring red, is: {}", non_red_sum);
        }
        _ => panic!("Couldn't parse provided json file"),
    }
}

fn parse_json(data: &str) -> Result<Value> {
    Ok(serde_json::from_str(data)?)
}

fn evaluate_json(body: &Value, ignore_red: bool) -> (i64, bool) {
    let mut sum = 0;
    let mut is_red = false;

    match body {
        Value::String(s) => { if s == "red" {is_red = true}},
        Value::Number(n) => sum += n.as_i64().unwrap(),
        Value::Array(array) => {
            for v in array.iter() {
                sum += evaluate_json(&v, ignore_red).0;
            }
        },
        Value::Object(obj) => {
            let mut obj_sum = 0;
            let mut is_obj_red = false;
            for v in obj.values() {
                let sub_item = evaluate_json(&v, ignore_red);
                if sub_item.1 && ignore_red {
                    is_obj_red = true;
                }
                obj_sum += sub_item.0;
            }
            sum += if is_obj_red {0} else {obj_sum};
        },
        _ => {}
    }

    (sum, is_red)
}
