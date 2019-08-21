use other_md5::{Md5, Digest};
use md5 as md5crate;
use std::time::Instant;

fn main() {
    let input = "yzbqklnj";
    // let target_hash_prefix = "00000"; // part 1 (5 zeros)
    let target_hash_prefix = "000000"; // part 2 (6 zeros)
    let mut counter = 0;
    
    let mut hash = String::from("");
    
    let now = Instant::now();
    while !hash.starts_with(target_hash_prefix) {
        counter += 1; 
        let val_to_hash = &format!("{}{}", input, counter);
        // hash = md5_create(val_to_hash);
        hash = alt_md5(val_to_hash);
    }
    let duration = now.elapsed().as_millis();
    println!("Earliest `{}` prefix occurs at: {}", target_hash_prefix, counter);
    println!("Time to solve: {} ms", duration);
}


fn alt_md5(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input(s);
    let result = hasher.result();
    hex::encode(result)
}

fn md5_create(s: &str) -> String {
    let digest = md5crate::compute(s);
    format!("{:x}", digest)
}

