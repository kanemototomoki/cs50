use std::{io};
fn main() -> io::Result<()> {
    let input_height: u8 = input_loop();

    mario(input_height);
    Ok(())
}

fn mario(len: u8) {
    println!("len!: {}", len);

    for i in 1..=len {
        let hash = "#".repeat(i as usize);
        let space = " ".repeat((len - i) as usize);
        println!("{}{}  {}{}", space, hash, hash, space);
    }
}

fn input_loop() -> u8 {
    loop {
        let mut input_height = String::new();
        println!("height: ");

        io::stdin().read_line(&mut input_height);
        let input_height: u8 = match input_height.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
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

//     #[test]
//     fn test_negative() {
//         assert_ne!()
//     }

// }
