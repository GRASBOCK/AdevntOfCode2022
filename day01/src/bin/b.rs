fn main() {
    let mut biggest = [0; 3]; // [0] < [1] < [2]
    include_str!("../../input.txt")
        .lines()
        .fold(0, |acc, text|{
            let r = text.parse::<u32>();
            if r.is_ok(){
                acc + r.unwrap() // add
            }else{
                let mut rank = 0;
                for i in 0..3{
                    if acc > biggest[i]{
                        rank += 1;
                    }
                }
                for i in 0..rank{
                    if i > 0{
                        biggest[i - 1] = biggest[i]; // shift
                    }
                    biggest[i] = acc; // insert
                }
                0 // reset
            }
        });
    let sum = biggest.iter().fold(0, |acc, x| acc + x);
    println!("{}", sum);
}
