fn main() {
    let path = "input.txt";
    let input = std::fs::read_to_string(path).expect(&format!("Failed to open: {}", path));
    let do_part_one = true;
    let name_count;

    match do_part_one {
        true => name_count = nice_names_v1(&input),
        false => name_count = nice_names_v2(&input),
    }

    println!("Number of nice names: {}", name_count);
}

fn nice_names_v1(raw_list: &str) -> usize {
    let all_names = raw_list.split("\n");
    // let nice_names: Vec<&str> = all_names.filter(|i| check_name_v1(&i)).collect();
    let nice_names: Vec<&str> = all_names
        .filter(has_double_letter)
        .filter(has_three_vowels)
        .filter(not_blacklisted)
        .collect();

    nice_names.len()
}

fn nice_names_v2(raw_list: &str) -> usize {
    let all_names = raw_list.split("\n");
    let nice_names: Vec<&str> = all_names
        .filter(name_has_letter_pair)
        .filter(name_has_nonadjacent_repeated_substring)
        .collect();

    nice_names.len()
}

fn has_three_vowels(name: &&str) -> bool {
    let mut vowel_count = 0;
    for ch in name.chars() {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
            _ => (),
        }
        if vowel_count > 2 {
            return true
        }
    }
    
    false
}

fn has_double_letter(name: &&str) -> bool {
    let mut iter = name.chars();
    let mut last_ch = iter.next().unwrap();
    for ch in iter {
        if ch == last_ch {
            return true;
        }
        last_ch = ch;
    }
    false
}

fn not_blacklisted(name: &&str) -> bool {
    let mut iter = name.chars();
    let mut last_ch = iter.next().unwrap(); // something that won't match any rule

    for ch in iter {
        let v = vec!(last_ch, ch);
        match v.as_slice() {
            ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'] => return false,
            _ => (),
        }
        last_ch = ch;
    }

    true
}

fn name_has_letter_pair(name: &&str) -> bool {
    let char_list: Vec<char> = name.chars().collect();
    for (idx, ch) in name.chars().enumerate() {
        if idx < 2 {
            continue;
        }

        if char_list[idx - 2] == ch {
            return true;
        }
    }
    false
}

fn name_has_nonadjacent_repeated_substring(name: &&str) -> bool {
    for i in 0..name.len()-2 {
        let target = name.get(i..i+2);
        for j in (2 + i)..name.len() {
            let test = name.get(j..j+2);
            if target == test {
                return true;
            }
        }
    }

    false
}

// old solution for part 1
// fn check_name_v1(name: &str) -> bool {
//     let mut last_ch = '!'; // something that won't match any rule
//     let mut vowel_count = 0;
//     let mut has_double_letter = false;

//     for ch in name.chars() {
//         // check for vowels
//         match ch {
//             'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
//             _ => (),
//         }
//         //check for double letters
//         if ch == last_ch {
//             has_double_letter = true;
//         }

//         // check for disallowed strings
//         let v = vec![last_ch, ch];

//         match v.as_slice() {
//             ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'] => return false,
//             _ => (),
//         }
//         last_ch = ch;
//     }

//     vowel_count > 2 && has_double_letter
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonadjacent_test_simple() {
        assert_eq!(true, name_has_nonadjacent_repeated_substring(&"xyxy"));
    }
    #[test]
    fn nonadjacent_test_bad_adjacent() {
        assert_eq!(false, name_has_nonadjacent_repeated_substring(&"aaa"));
    }

    #[test]
    fn nonadjacent_test_bad_no_repeat() {
        assert_eq!(false, name_has_nonadjacent_repeated_substring(&"abcdefg"));
    }

    // foil checks (first, inner, outer, last)
    #[test]
    fn nonadjacent_test_good_first() {
        assert_eq!(true, name_has_nonadjacent_repeated_substring(&"aabcdaaefg"));
    }
    #[test]
    fn nonadjacent_test_good_inner() {
        assert_eq!(true, name_has_nonadjacent_repeated_substring(&"abcmmnmmcba"));
    }

    #[test]
    fn nonadjacent_test_good_outer() {
        assert_eq!(true, name_has_nonadjacent_repeated_substring(&"aabcdefgaa"));
    }

    #[test]
    fn nonadjacent_test_good_last() {
        assert_eq!(true, name_has_nonadjacent_repeated_substring(&"abbcdefgbb"));
    }

    #[test]
    fn nonadjacent_test_provided_1() {
        assert_eq!(true, name_has_nonadjacent_repeated_substring(&"qjhvhtzxzqqjkmpb"));
    }
    
    #[test]
    fn nonadjacent_test_provided_2() {
        assert_eq!(true, name_has_nonadjacent_repeated_substring(&"xxyxx"));
    }

    #[test]
    fn nonadjacent_test_provided_3() {
        assert_eq!(true, name_has_nonadjacent_repeated_substring(&"xxyxx"));
    }

    #[test]
    fn nonadjacent_test_provided_4() {
        assert_eq!(true, name_has_nonadjacent_repeated_substring(&"uurcxstgmygtbstg"));
    }

    #[test]
    fn nonadjacent_test_provided_5() {
        assert_eq!(false, name_has_nonadjacent_repeated_substring(&"ieodomkazucvgmuy"));
    }
}