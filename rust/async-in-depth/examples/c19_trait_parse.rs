use regex::Regex;

pub trait Parser {
    fn parse(s: &str) -> Self;
}

impl Parser for u8 {
    fn parse(s: &str) -> Self {
        let re: Regex = Regex::new(r"^[0-9]+").unwrap();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(0, |s| s.as_str().parse().unwrap_or(0))
        } else {
            0
        }
    }
}

#[test]
fn parse_should_work() {
    assert_eq!(u8::parse("123abc"), 123);
    assert_eq!(u8::parse("asdf123abc"), 0);
    // assert_eq!(u8::parse("123asd123abc"), 123);
}

fn main() {
    println!("Hello, world!");
}
