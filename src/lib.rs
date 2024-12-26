use std::result::Result;
use std::error::Error;
use std::any::type_name;

fn type_of<T: ?Sized>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

fn first_line(s: &str) -> String {
    for line in s.lines() {
        return line.to_string()
    }
    s.to_string()
}

fn last_line(s: &str) -> String {
    let mut current_line = "";
    for line in s.lines() {
        current_line = line;
    }
    current_line.to_string()
}

fn count_lines(s: &str) -> i32 {
    let mut count = 0;
    for _line in s.lines() {
        count += 1;
    }
    count
}

fn line_number(s: &str, number: usize) -> Result<String, String> {
    match s.lines().nth(number) {
        Some(line) => Ok(line.to_string()),
        None => Err(format!("Line number {} is out of range.", number)),
    }
}



#[cfg(test)]
mod tests {
    use super::*;

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

}
