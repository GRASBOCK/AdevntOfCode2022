use std::collections::HashSet;

use ascii::{AsAsciiStr, AsciiStr, AsciiString, AsciiChar};

pub fn find_marker(input: &AsciiStr, marker_length: usize) -> usize{
    for offset in 0..input.len(){
        let marker_index = offset + marker_length;
        let patch = &input[offset..marker_index];
        let set = patch.chars().collect::<HashSet::<AsciiChar>>();
        if set.len() == marker_length{
            return marker_index;
        }
    }
    panic!("No marker found");
}

pub fn parse_input() -> AsciiString{
    AsciiString::from(include_str!("../input.txt").as_ascii_str().unwrap())
}