use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
