// https://cs50.jp/x/2021/week1/problem-set/mario-more-comfortable/
use std::io::{self, Write};

pub fn run() {
    let input_height = String::new();
    let input_height: u8 = input_loop(&input_height);

    display_block(input_height);
}

fn display_block(len: u8) {
    for i in 1..=len {
        let hash = "#".repeat(i as usize);
        let space = " ".repeat((len - i) as usize);
        println!("{}{}  {}{}", space, hash, hash, space);
    }
}

fn input_loop(input_height: &str) -> u8 {
    loop {
        print!("height: ");
        io::stdout().flush().unwrap();

        let mut input_height = input_height.to_string();
        io::stdin().read_line(&mut input_height);
        let input_height: u8 = match input_height.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input_height > 8 || input_height == 0 {
            continue;
        }

        return input_height;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // -1 (または他の負の数)
//     #[test]
//     fn test_negative() {}

//     // 0
//     #[test]
//     fn test_zero() {}

//     // 9または他の正の数
//     #[test]
//     fn test_over_number() {}

//     // 文字・文字列
//     #[test]
//     fn test_not_number() {}

//     // 何も入力せず、Enterキーだけを押したとき
//     #[test]
//     fn test_not_input() {}
// }
