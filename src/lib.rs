use std::result::Result;
use std::any::type_name;

pub fn type_of<T: ?Sized>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

pub fn first_line(s: &str) -> String {
    for line in s.lines() {
        return line.to_string()
    }
    s.to_string()
}

pub fn last_line(s: &str) -> String {
    let mut current_line = "";
    for line in s.lines() {
        current_line = line;
    }
    current_line.to_string()
}

pub fn count_lines(s: &str) -> i32 {
    let mut count = 0;
    for _line in s.lines() {
        count += 1;
    }
    count
}

pub fn line_number(s: &str, number: usize) -> Result<String, String> {
    match s.lines().nth(number) {
        Some(line) => Ok(line.to_string()),
        None => Err(format!("Line number {} is out of range.", number)),
    }
}

pub fn trim_leading_spaces(s: &str) -> (String, usize) {
    let mut count = 0;
    for char in s.chars() {
        if char != ' ' {
            break;
        }
        count += 1;
    }
    (s[count..].to_string(), count)
}

pub fn trim_leading_chars(s: &str, ch: char, amount: usize) -> String {
    let mut count = 0;
    for char in s.chars() {
        if char != ch {
            break;
        }
        count += 1;
        if count == amount {
            break;
        }
    }
    s[count..].to_string()
}

pub fn trim_leading_empty_lines(s: &str) -> String {
    let mut good_lines = vec![];
    let mut found_start = false;
    for line in s.lines() {
        if found_start {
            good_lines.push(line);
            continue;
        }
        if line.trim() == "" {
            // skip
        } else {
            found_start = true;
            good_lines.push(line);
        }
    }
    good_lines.join("\n")
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_default() {
        let str = format!("
       
        ive got two empty lines");
        trim_leading_empty_lines(&str);
    }

    #[test]
    fn test_first_line() {
        let mut str = "some text\non a new line";
        assert_eq!(first_line(str), "some text");
        str = "\nshould be empty?";
        assert_eq!(first_line(str), "");
    }


    #[test]
    fn test_last_line() {
        let str = "some text\nthe last line";
        assert_eq!(last_line(str), "the last line");
    }

    #[test]
    fn test_count_lines() {
        let str = "some text\nwith a few lines\nto test";
        assert_eq!(count_lines(str), 3);
    }

    #[test]
    fn test_line_number() {
        let str = "line one\nline two\nline three";
        assert_eq!(line_number(str, 1).unwrap(), "line two");
    }

    #[test]
    fn test_trim_leading_spaces() {
        let str = "    a string with leading spaces";
        assert_eq!(trim_leading_spaces(str).0, "a string with leading spaces");
        assert_eq!(trim_leading_spaces(str).1, 4);
    }

    #[test]
    fn test_trim_leading_chars() {
        let str = "  two leading spaces";
        let str2 = "       seven leading spaces";
        assert_eq!(trim_leading_chars(str, ' ', 6), "two leading spaces");
        assert_eq!(trim_leading_chars(str2, ' ', 6), " seven leading spaces");
    }

    #[test]
    fn test_trim_leading_empty_lines() {
        let str = format!("
        ive got two empty lines");
        assert_eq!(trim_leading_empty_lines(&str), "        ive got two empty lines");
    }



}
