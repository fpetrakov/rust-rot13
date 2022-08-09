use std::io;

fn main() {
    println!("Enter string to encode!");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Reading input error!");

    println!("Here's your encoded string: {}", rot13(input.trim_end()));
}

fn rot13(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            'A'..='M' | 'a'..='m' => ((c as u8) + 13) as char,
            'N'..='Z' | 'n'..='z' => ((c as u8) - 13) as char,
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lower_case() {
        assert_eq!("uryyb", rot13("hello"))
    }

    #[test]
    fn test_upper_case() {
        assert_eq!("URYYB", rot13("HELLO"))
    }
}
