pub fn parse_pair(line: &str) -> ((u32, u32), (u32, u32)){
    let numbers = line
        .split(&['-', ','][..])
        .map(|t| t.parse().expect(format!("parse error {t}").as_str()))
        .collect::<Vec<u32>>();
    ((numbers[0], numbers[1]), (numbers[2], numbers[3]))
}