use std::io;
use std::str;

pub struct Scanner<R: io::BufRead> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Scanner {
            reader,
            buffer: Vec::new(),
        }
    }
    pub fn next<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Token Parsing Failure");
            }
            let mut line = String::new();
            self.reader.read_line(&mut line).expect("Line Read Failure");
            self.buffer = line
                .split_ascii_whitespace()
                .rev()
                .map(String::from)
                .collect();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_in_memory() {
        let input: &[u8] = b"50\t 8 \n1 2 3";
        let mut scanner = Scanner::new(input);
        let answer = (0..4).map(|_| scanner.next()).collect::<Vec<u32>>();
        let expected = vec![50, 8, 1, 2];
        assert!(answer == expected);
        assert!(3 == scanner.next());
    }
}
