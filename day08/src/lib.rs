pub fn analyze_line(s: &str) -> (u16, u16) {
    // assumption: all strings are valid in the input
    // as such, every string begins and ends with a " character
    let str_length = s.len() as u16;
    let line_chars: Vec<char> = s.chars().collect();

    let mut skip = 0;
    let mut ignore_count = 0;
    for i in 1..line_chars.len() - 1 {
        if skip > 0 {
            skip -= 1;
        } else if line_chars[i] == '\\' {
            skip = match line_chars[i + 1] {
                'x' => 3,
                '\\' | '"' => 1,
                _ => 0,
            }; // skip sequences always produce 1 character, so we ignore all but 1 character in memory
            ignore_count += skip;
        }
    }

    (str_length, (str_length - ignore_count - 2))
}

pub fn escape_line(s: &str) -> String {
    let mut new_line = String::new();

    for ch in s.chars() {
        match ch {
            '\\' => new_line.push_str("\\\\"),
            '"' => new_line.push_str("\\\""),
            _ => new_line.push(ch),
        }
    }

    new_line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_line_with_no_escapes() {
        assert_eq!("Howdy", escape_line(r#"Howdy"#));
    }

    #[test]
    fn escape_line_examples() {
        assert_eq!(r#"\"\""#, escape_line(r#""""#));
        assert_eq!(r#"\"abc\""#, escape_line(r#""abc""#));
        assert_eq!(r#"\"aaa\\\"aaa\""#, escape_line(r#""aaa\"aaa""#));
        assert_eq!(r#"\"\\x27\""#, escape_line(r#""\x27""#));
    }

    #[test]
    fn analyze_line_plain() {
        assert_eq!((6, 4), analyze_line(r#""joel""#));
    }

    #[test]
    fn analyze_line_escape_backslash() {
        assert_eq!((8, 5), analyze_line(r#""jo\\el""#));
    }

    #[test]
    fn analyze_line_escape_provided() {
        assert_eq!((2, 0), analyze_line(r#""""#));
        assert_eq!((5, 3), analyze_line(r#""abc""#));
        assert_eq!((10, 7), analyze_line(r#""aaa\"aaa""#));
        assert_eq!((6, 1), analyze_line(r#""\x27""#));
    }

    #[test]
    fn analyze_line_input_test() {
        assert_eq!(
            (38, 29),
            analyze_line(r#""sjdivfriyaaqa\xd2v\"k\"mpcu\"yyu\"en""#)
        );
    }

}
