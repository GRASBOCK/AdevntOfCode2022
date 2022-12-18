use ascii::{AsAsciiStr, AsciiStr, AsciiString};

pub fn parse_input() -> AsciiString{
    AsciiString::from(include_str!("../input.txt").as_ascii_str().unwrap())
}