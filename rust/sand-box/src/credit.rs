// https://cs50.jp/x/2021/week1/problem-set/credit/

use std::io::{self, Write};

pub fn run() {
    let input_result = input_loop();

    if !input_result.is_ok {
        println!("INVALID");
        return;
    }

    let card_company: String = check_card_number(&input_result.number);
    // let card_company: String = check_card_number("4003600000000014");

    println!("{}", card_company);
}

struct InputResult {
    number: i128,
    is_ok: bool,
}

fn input_loop() -> InputResult {
    const MIN_LEN: usize = 14;
    const MAX_LEN: usize = 19;
    loop {
        print!("Number: ");
        io::stdout().flush().unwrap();

        let mut input_number = String::new();
        io::stdin().read_line(&mut input_number);

        let mut result = InputResult {
            number: 0,
            is_ok: true,
        };

        let input_number: i128 = match i128::from_str_radix(&input_number.trim(), 10) {
            Ok(num) => {
                // 14~19桁以外はErrとする
                if num.to_string().len() < MIN_LEN || num.to_string().len() > MAX_LEN {
                    result.is_ok = false;
                }
                num
            }
            Err(_) => continue,
        };

        result.number = input_number;
        return result;
    }
}

fn check_card_number(number: &i128) -> String {
    let n: String = number.to_string();
    let n: Vec<_> = n.as_str().chars().rev().collect();

    let mut even: Vec<&char> = Vec::new();
    let mut odd: Vec<&char> = Vec::new();
    for (i, s) in n.iter().enumerate() {
        if i % 2 != 0 {
            // let n = s.to_digit(10).unwrap() * 2;
            // let n = char::from_digit(n, 10).unwrap();
            odd.push(s);
        } else {
            even.push(s)
        }
        // println!("i: {:?} s: {:?}", i, s);
    }
    // println!("even: {:?} odd: {:?}", even, odd);
    // 偶数桁を2倍する
    let odd: Vec<u32> = odd.iter().map(|v| v.to_digit(10).unwrap() * 2).collect();
    let odd_sum = odd.iter().fold(0, |mut acc, &v| {
        // 2桁の場合は一桁ずつ加算する
        if v > 9 {
            acc += 1 + v - 10;
        } else {
            acc += v;
        }
        acc
    });

    let total_sum = even
        .iter()
        .fold(odd_sum, |acc, &v| acc + v.to_digit(10).unwrap());

    let is_validity: bool = total_sum % 10 == 0;

    if !is_validity {
        return "INVALID".to_string();
    }

    // 34 or 37: American Express
    // 51~55: MasterCard
    // 4: Visa
    let first: String = number.to_string().chars().nth(0).unwrap().to_string();
    let second: String = number.to_string().chars().nth(1).unwrap().to_string();
    let join: String = format!("{}{}", first, second);

    // println!("first: {:?}", first);
    // println!("second: {:?}", second);
    // println!("join: {:?}", join);

    if first == "4".to_string() {
        return "VISA".to_string();
    } else if join == "51".to_string()
        || join == "52".to_string()
        || join == "53".to_string()
        || join == "54".to_string()
        || join == "55".to_string()
    {
        return "MasterCard".to_string();
    } else if join == "34".to_string() || join == "37".to_string() {
        return "AmericanExpress".to_string();
    }

    return "another_brand".to_string();
}
