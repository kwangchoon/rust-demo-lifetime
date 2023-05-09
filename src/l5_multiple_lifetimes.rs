pub struct StrTok {
    remaining: Option<String>,
    delimiter: String,
}

impl StrTok {
    pub fn new(haystack: String, delimiter: String) -> StrTok {
        StrTok {
            remaining: Some(haystack),
            delimiter,
        }
    }
}

impl Iterator for StrTok {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!("Implement the next method for StrTok")
    }
}

pub fn until_char(haystack: &str, c: char) -> &str {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn split() {
        let s = " ";
        let (b, a) = s.split_at(0);
        println!("b = {:?}, a = {:?}", b, a);
        let (b, a) = s.split_at(1);
        println!("b = {:?}, a = {:?}", b, a);
    }

    #[test]
    fn test1() {
        let haystack = "Quick brown fox";
        let mut words = StrTok::new(haystack.to_string(), " ".to_string());

        assert_eq!(words.next(), Some("Quick".to_string()));
        assert_eq!(words.next(), Some("brown".to_string()));
        assert_eq!(words.next(), Some("fox".to_string()));
        assert_eq!(words.next(), None);
    }

    #[test]
    #[ignore]
    fn test2() {
        let haystack = "Quick brown fox ";
        let mut words = StrTok::new(haystack.to_string(), " ".to_string());

        assert_eq!(words.next(), Some("Quick".to_string()));
        assert_eq!(words.next(), Some("brown".to_string()));
        assert_eq!(words.next(), Some("fox".to_string()));
        assert_eq!(words.next(), Some("".to_string()));
        assert_eq!(words.next(), None);
    }

    #[test]
    #[ignore]
    fn test3() {
        let haystack = " ";
        let mut words = StrTok::new(haystack.to_string(), " ".to_string());

        assert_eq!(words.next(), Some("".to_string()));
        assert_eq!(words.next(), Some("".to_string()));
        assert_eq!(words.next(), None);
    }

    #[test]
    #[ignore]
    fn test4() {
        let haystack = "Quick brown fox ";
        let delim = ' ';

        let result = until_char(haystack, delim);
        assert_eq!(result, "Quick");
    }
}
