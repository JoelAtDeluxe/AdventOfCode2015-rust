

pub fn advance_to_next_reasonable_password(original: &str) -> String {
    // we don't really need to implement this -- input is already okay
    
    if !password_rule_no_iol(original) {
        let mut rtn = String::new();
        for (i, ch) in original.chars().enumerate() {
            match ch {
                'o' | 'i' | 'l' => {
                    rtn.push( ((ch as u8) + 1) as char);
                    rtn.push_str( &vec!['a'; original.len() - i - 1].into_iter().collect::<String>());
                    break;
                },
                _ => rtn.push(ch)
            };
        }
        return rtn
    }

    String::from(original) 
}

pub fn advance_password(old: &str) -> String {
    let mut rtn = vec!['?'; old.len()];

    let as_chars: Vec<char> = old.chars().collect();
    let mut inc_char = true;
    let mut iteration = 0;
    while inc_char {
        let edit_position = as_chars.len() - iteration - 1; // will count [0, n)
        let mut next = as_chars[edit_position] as u8 + 1;
        match next {
            ch if ch > 122 => next = 97, // greater than z ==> inc next char -- reset this one to 'a'
            111 | 105 | 108 => {  // character is i, o, or l, so need to
                next += 1; 
                inc_char = false;
                // you probably don't need to do this -- If we ignore the case where we start with o/i/l
                // then the only way we'll get to this point is if we overflow into an o/i/l, in which
                // case, all previous characters are 'a' anyway

                // for i in edit_position + 1 .. as_chars.len() {
                //     rtn[i] = 'a';
                // }
            },
            _ => inc_char = false

        };
        rtn[edit_position] = next as char;
        iteration += 1; 
    }
    // copy remainder into output
    for i in 0..(as_chars.len() - iteration) {
        rtn[i] = as_chars[i];
    }

    rtn.into_iter().collect::<String>()
}

pub fn password_rule_no_iol(password: &str) -> bool {
    for ch in password.chars() {
        match ch {
            'o' | 'i' | 'l' => return false,
            _ => {}
        }
    }

    return true;
}

pub fn password_rule_three_in_order(password: &str) -> bool {
    if password.len() < 3 {
        return false;
    }
    let chars: Vec<u8> = password.chars().map(|ch| ch as u8).collect();

    let mut start_index = 0;
    for i in 1..chars.len() {
        let distance = i - start_index;
        if chars[i] != chars[i-distance] + distance as u8{
            start_index = i;
        }
        else {
            if distance == 2 {
                return true;
            }
        }
    }

    false
}

pub fn password_rule_two_doubles(password: &str) -> bool {
    let mut double_char_1: Option<char> = None;
    
    let mut pass_itr = password.chars();
    let mut last_char = pass_itr.next();
    let mut skip_next = false;
    for ch in pass_itr {
        if skip_next {
            last_char = None;
            skip_next = false;
            continue;
        }
        if Some(ch) == last_char {
            match double_char_1 {
                None => {
                    double_char_1 = last_char;
                    skip_next = true;
                },
                Some(x) if x != ch => return true,
                Some(_) => skip_next = true,
            }
        }
        else {
            last_char = Some(ch);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advance_to_next_reasonable_password_various() {
        assert_eq!("jpaa", advance_to_next_reasonable_password("joel"));
        assert_eq!("jmaa", advance_to_next_reasonable_password("jlel"));
        assert_eq!("jjaa", advance_to_next_reasonable_password("jiel"));

        assert_eq!("paa", advance_to_next_reasonable_password("oil"));
        assert_eq!("ahpa", advance_to_next_reasonable_password("ahoy"));
        assert_eq!("jaj", advance_to_next_reasonable_password("jai"));
    }

    #[test]
    fn advance_password_various() {
        assert_eq!("vzbxkghc", advance_password("vzbxkghb")); //plain, simple
        assert_eq!("caa", advance_password("bzz")); //double overflow
        assert_eq!("jjaa", advance_password("jhzz")); // o/i/l stepover
    }

    #[test]
    fn check_for_double_pairs_various() {
        assert!(password_rule_two_doubles("aabcc"));
        assert!(!password_rule_two_doubles("abcd"));
        assert!(!password_rule_two_doubles("aabb"));
        assert!(!password_rule_two_doubles("aabaa"));
        assert!(password_rule_two_doubles("ghiaabcc"));
        assert!(password_rule_two_doubles("abbceffg"));
    }
}