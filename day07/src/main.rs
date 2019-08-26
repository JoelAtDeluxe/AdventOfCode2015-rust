use std::collections::HashMap;

pub mod operation;
use operation::Op;

fn main() {
    let path = "input.txt";
    let input =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));

    let mut all_gates = make_gates(input);

    let result = evaluate(&mut all_gates, "a");
    println!("Initial Value of Gate-A is: {}", result);

    for v in all_gates.values_mut() {
        v.value = None
    }
    all_gates.get_mut("b").unwrap().value = Some(result);

    let result = evaluate(&mut all_gates, "a");
    println!("Updated value of Gate-A is: {}", result);
}

fn as_int(s: &str) -> u16 {
    s.parse::<u16>().unwrap()
}

// passing the name of the gate
fn evaluate(gate_map: &mut HashMap<String, Gate>, gate_name: &str) -> u16 {
    let maybe_gate = gate_map.get(gate_name);
    let mut gate_copy = match maybe_gate {
        Some(gate) => gate.clone(),
        None => return as_int(gate_name),
    };

    match gate_copy.value {
        Some(val) => val,
        None => {
            let val = match gate_copy.op {
                Op::SET => match gate_copy.input_a.parse::<u16>() {
                    Ok(v) => v,
                    Err(_) => evaluate(gate_map, &gate_copy.input_a),
                },
                Op::NOT => !evaluate(gate_map, &gate_copy.input_a),
                Op::LSHIFT => evaluate(gate_map, &gate_copy.input_a) << as_int(&gate_copy.input_b),
                Op::RSHIFT => evaluate(gate_map, &gate_copy.input_a) >> as_int(&gate_copy.input_b),
                Op::OR => {
                    evaluate(gate_map, &gate_copy.input_a) | evaluate(gate_map, &gate_copy.input_b)
                }
                Op::AND => {
                    evaluate(gate_map, &gate_copy.input_a) & evaluate(gate_map, &gate_copy.input_b)
                }
                Op::NOP => panic!("Can't evaluate register; no Op defined for this register"),
            };
            gate_copy.value = Some(val);
            gate_map.insert(String::from(gate_name), gate_copy);
            val
        }
    }
}

#[derive(Clone, Debug)]
struct Gate {
    input_a: String,
    input_b: String,
    op: Op,
    value: Option<u16>,
}

impl Gate {
    pub fn new(input_a: String, input_b: String, op: &str) -> Gate {
        Gate {
            input_a,
            input_b,
            op: Op::from(&op),
            value: None,
        }
    }
}

fn make_gates(file_data: String) -> HashMap<String, Gate> {
    let mut rtn = HashMap::new();

    for line in file_data.lines() {
        let parts: Vec<&str> = line.split(' ').collect();

        let (store_in, gate) = match parts.as_slice() {
            [reg_a, binary_op, reg_b, "->", store_in] => (
                store_in,
                Gate::new(String::from(*reg_a), String::from(*reg_b), *binary_op),
            ),
            [unary_op, reg, "->", store_in] => (
                store_in,
                Gate::new(String::from(*reg), String::from(""), *unary_op),
            ),
            [reg, "->", store_in] => (
                store_in,
                Gate::new(String::from(*reg), String::from(""), "SET"),
            ),
            _ => panic!("Could not match line to pattern"),
        };

        rtn.insert(String::from(*store_in), gate);
    }

    rtn
}
