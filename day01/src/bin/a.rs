fn main() {
    let mut biggest = 0;
    include_str!("../../input.txt")
        .lines()
        .fold(0, |acc, text|{
            let r = text.parse::<u32>();
            if r.is_ok(){
                acc + r.unwrap() // add
            }else{
                if acc > biggest{
                    biggest = acc;
                }
                0 // reset
            }
        });
    println!("{}", biggest);
}
