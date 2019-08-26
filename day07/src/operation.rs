#[derive(Clone, Debug)]
pub enum Op {
    LSHIFT,
    RSHIFT,
    AND,
    NOT,
    OR,
    SET,
    NOP,
}

impl Op {
    pub fn from(s: &str) -> Op {
        match s {
            "LSHIFT" => Op::LSHIFT,
            "RSHIFT" => Op::RSHIFT,
            "AND" => Op::AND,
            "NOT" => Op::NOT,
            "OR" => Op::OR,
            "SET" => Op::SET,
            _ => Op::NOP,
        }
    }
}
