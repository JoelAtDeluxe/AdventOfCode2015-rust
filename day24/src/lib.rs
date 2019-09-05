mod combo_status;
pub use combo_status::ComboStatus;

use std::collections::HashMap;


pub fn combo_to_subset(v: &Vec<u32>, combo_number: usize) -> Vec<u32> {
    let mut rtn = Vec::new();

    binary_iter(combo_number, &mut|n| {
        rtn.push(v[*n].clone())
    });

    rtn
}

fn sum_digits(i: usize) -> usize {
    let mut sum = 0;
    binary_iter(i, &mut |_|{ sum += 1;});
    sum 
}

pub fn n_choose_m_combo_generator(n: usize, m: usize) -> Vec<usize> { // TODO: see if this can be made more efficient
    let base: Vec<u8> = vec![0; n as usize]; // n choose 0
    
    let mut last_set: HashMap<usize, Vec<u8>>;
    let mut next_set: HashMap<usize, Vec<u8>> = HashMap::new();
    next_set.insert(0, base);

    for _ in 0 .. m {
        last_set = next_set;
        next_set = HashMap::new();
        for (idx, element) in last_set {
            let starting_point = element.clone();
            // this is going to put a bunch of junk in here
            for n_index in 0 .. n {
                let n_index = n_index as usize;
                if starting_point[n_index] == 1 {
                    continue;
                }
                let mut modified = starting_point.clone();
                modified[n_index] = 1;
                let key = magic(&modified);
                next_set.entry(key).or_insert(modified);
                // next_set.insert(key, modified);
            }
        }
    }
    
    next_set.keys().map(|k| k.clone()).collect()
}

pub fn magic(digits: &Vec<u8>) -> usize { // TODO: rename
    let mut sum = 0;

    for (idx, v) in digits.iter().enumerate() {
        if *v == 1 {
            sum += 1 << idx;
        }
    }

    sum
}

pub fn get_number_of_combos(v_len: usize) -> u32 {
    (1 << v_len) as u32
}

pub fn mask_num(mask_len: usize, num: usize) -> usize {
    num & ((1 << mask_len) - 1)
}

pub fn mk_index_map(vec_length: usize, combo_number: usize) -> Vec<usize> {
    let mut index_map = Vec::new();

    binary_iter( mask_num(vec_length, combo_number), &mut |i| {index_map.push(*i)});

    index_map
}


pub fn mk_full_index_map(size: usize) -> Vec<usize> {
    (0..size).collect()
}

pub fn mk_trimmed_index_map(original: &Vec<usize>, combo: usize) -> Vec<usize> {
    let mut rtn = Vec::new();

    binary_iter(combo, &mut |index|{
        rtn.push(original[*index])
    });

    rtn
}

pub fn binary_iter<T> (number: usize, func: &mut T) 
    where T: FnMut( &mut usize)
{
    let mut n = number;
    let mut index = 0;
    
    while n > 0 {
        if (n & 0x1) != 0 {
            func(&mut index);
        }
        n = n >> 1;
        index += 1;
    }
}


pub fn is_power_of_two(num: u32) -> bool {
    // assumes num > 0
    // https://stackoverflow.com/questions/600293/how-to-check-if-a-number-is-a-power-of-2
    num & (num - 1) == 0
}


pub fn mk_value_map<T>(base: &Vec<T>, index_map: &Vec<usize>) -> Vec<T>
where
    T: Clone,
{
    base.iter()
        .enumerate()
        .filter(|(i, _)| index_map.contains(i))
        .map(|(_, v)| v.clone())
        .collect()
}



pub fn mk_combo_from_subcombo(index_map: &Vec<usize>, sub_combo: usize) -> usize {
    let mut sum: usize = 0;
    let junk = index_map.len() == 12;

    binary_iter(sub_combo, &mut |n| {
        sum += 1 << index_map[*n]
    });

    // let mut n = sub_combo;
    // let mut index = 0;
    
    // while n > 0 {
    //     if (n & 0x1) != 0 {
    //         sum += 1 << index_map[index];
    //     }
    //     n = n >> 1;
    //     index += 1;
    // }

    sum
}

pub fn mark_combo_proper_subsets_invalid(index_map: &Vec<usize>, checked_values: &mut Vec<ComboStatus>) {
    let num_subsets = get_number_of_combos(index_map.len()) as usize;
    
    for i in 1 .. (num_subsets-1) {
        let translated_subset = mk_combo_from_subcombo(index_map, i);
        checked_values[translated_subset] = ComboStatus::KnownBad;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combo_to_subset() {
        let v: Vec<u32> = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
        let combo = 0b0100100100; // multiples of 3
        let expected: Vec<u32> = vec![6, 12, 18];
        let actual = combo_to_subset(&v, combo);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mk_index_map() {
        let base: Vec<u32> = vec![4, 8, 15, 16, 23, 42];
        //  1124  (read top to bottom, _then_ left to right)
        //485632
        let combo = 0b001100;
        let expected: Vec<usize> = vec![2, 3];
        let actual = mk_index_map(base.len(), combo);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mk_index_map_not() {
        let base: Vec<u32> = vec![4, 8, 15, 16, 23, 42];
        //  1124  (read top to bottom, _then_ left to right)
        //485632
        let combo = 0b001100;
        let not_combo = !combo;
        let expected: Vec<usize> = vec![0, 1, 4, 5];
        let actual = mk_index_map(base.len(), not_combo);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mk_trimmed_index_map() {
        let base_index_map = vec![2, 3, 5];
        let combo = 0b101;
        let expected = vec![2, 5];
        let actual = mk_trimmed_index_map(&base_index_map, combo);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mk_value_map() {
        let base: Vec<u32> = vec![4, 8, 15, 16, 23, 42];
        let index_map:Vec<usize> = vec![2, 3];
        let expected: Vec<u32> = vec![15, 16];
        let actual = mk_value_map(&base, &index_map);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mk_combo_from_subcombo() {
        // let base: Vec<u32> = vec![4, 8, 15, 16, 23, 42]; // full set, for reference
        let index_map:Vec<usize> = vec![2, 3, 5]; 
        let subcombo = 0b101;
        let expected = 0b100100;
        let actual = mk_combo_from_subcombo(&index_map, subcombo);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mini_routine() {
        let base: Vec<u32> = vec![4, 8, 15, 16, 23, 42]; // full set, for reference
        let starting_combo: usize = 0b010110; // (represents: 8, 15, 23)
        let index_map = mk_index_map(base.len(), starting_combo); // [1, 2, 4]
        let value_map = mk_value_map(&base, &index_map); // [8, 15, 23]
        
        assert_eq!(value_map, vec![8, 15, 23]);

        let subcombo = 0b101;
        let finished_combo = mk_combo_from_subcombo(&index_map, subcombo);
        let finished_value = combo_to_subset(&base, finished_combo);

        let expected_combo = 0b010010;
        let expected_value = vec![8, 23];
        
        assert_eq!(finished_combo, expected_combo);
        assert_eq!(finished_value, expected_value);
    }

    #[test]
    fn test_n_choose_m_combo_generator_0() {
        let actual = n_choose_m_combo_generator(5, 0);
        let expected = vec![0];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_n_choose_m_combo_generator_1() {
        let mut actual = n_choose_m_combo_generator(5, 1);
        let mut expected = vec![
            0b00001,
            0b00010,
            0b00100,
            0b01000,
            0b10000,
        ];
        actual.sort();
        expected.sort();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_n_choose_m_combo_generator_2() {
        let mut actual = n_choose_m_combo_generator(5, 2);
        let mut expected = vec![
            0b00011, 
            0b00101, 0b00110, 
            0b01001, 0b01010, 0b01100,
            0b10001, 0b10010, 0b10100, 0b11000,
        ];
        actual.sort();
        expected.sort();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_n_choose_m_combo_generator_3() {
        let mut actual = n_choose_m_combo_generator(5, 3);
        let mut expected = vec![
            0b11100,                                // 0b00011, 
            0b11010, 0b11001,                       // 0b00101, 0b00110, 
            0b10110, 0b10101, 0b10011,              // 0b01001, 0b01010, 0b01100,
            0b01110, 0b01101, 0b01011, 0b00111,     // 0b10001, 0b10010, 0b10100, 0b11000,
        ];
        actual.sort();
        expected.sort();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_n_choose_m_combo_generator_4() {
        let mut actual = n_choose_m_combo_generator(5, 4);
        let mut expected = vec![
            0b11110,
            0b11101,
            0b11011,
            0b10111,
            0b01111,
        ];
        actual.sort();
        expected.sort();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mini_routine_2() {
        let base: Vec<u32> = vec![4, 8, 15, 16, 23, 42]; // full set, for reference
        let starting_combo: usize = 0b010110; // (represents: 8, 15, 23)
        let opposite_combo = mask_num(base.len(), !starting_combo); // should be 0b101001
        assert_eq!(opposite_combo, 0b101001);
        let subcombo = 0b001001;
        assert_eq!(subcombo & opposite_combo, subcombo);
        let flipped_combo = mask_num(base.len(), subcombo ^ opposite_combo);

        println!("{:8b}", (1 << base.len()) - 1);
        println!("{:8b}", starting_combo);
        println!("{:8b}", opposite_combo);
        println!("{:8b}", subcombo);
        println!("{:8b}", flipped_combo);

        assert_eq!(flipped_combo, 0b100000);
    }
}
