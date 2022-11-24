// https://cs50.jp/x/2021/week1/problem-set/credit/

use std::io::{self, BufRead, Write};

#[derive(Debug)]
struct InputResult {
    number: i128,
    is_ok: bool,
}

struct CreditCardValidator<R, W> {
    reader: R,
    writer: W,
}

impl<R, W> CreditCardValidator<R, W>
where
    R: BufRead,
    W: Write,
{
    fn prompt(&mut self) -> String {
        write!(&mut self.writer, "{}", "Number: ").expect("Unable to write");
        self.writer.flush().unwrap();

        let mut input_number: String = String::new();
        self.reader
            .read_line(&mut input_number)
            .expect("Unable to read");
        input_number
    }

    fn valid_digits(&mut self) -> InputResult {
        const MIN_LEN: usize = 14;
        const MAX_LEN: usize = 19;

        loop {
            let input_number = self.prompt();
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

    fn check_card_number(self, result: InputResult) -> String {
        if !result.is_ok {
            return "INVALID".to_string();
        }

        let number = result.number;
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
        }
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

        if first == "4".to_string() {
            return "VISA".to_string();
        } else if join == "22".to_string()
            || join == "51".to_string()
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
}

pub fn run() {
    let stdio = io::stdin();
    let input = stdio.lock();

    let output = io::stdout();
    let mut credit_card_validator = CreditCardValidator {
        reader: input,
        writer: output,
    };

    let input_result = credit_card_validator.valid_digits();
    let card_company: String = credit_card_validator.check_card_number(input_result);
    // let card_company: String = check_card_number("4003600000000014");

    println!("{}", card_company);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failed_13digits() {
        let input = b"1234567890123";
        let mut output: Vec<u8> = Vec::new();

        let input_result = {
            let mut credit_card_validator = CreditCardValidator {
                reader: &input[..],
                writer: &mut output,
            };

            credit_card_validator.valid_digits()
        };

        assert_eq!(input_result.is_ok, false);
    }
    #[test]
    fn test_failed_20digits() {
        let input = b"12345678901234567890";
        let mut output: Vec<u8> = Vec::new();

        let input_result = {
            let mut credit_card_validator = CreditCardValidator {
                reader: &input[..],
                writer: &mut output,
            };

            credit_card_validator.valid_digits()
        };

        assert_eq!(input_result.is_ok, false);
    }
    // #[test]
    fn test_failed_no_number() {
        let input = b"awa";
        let mut output: Vec<u8> = Vec::new();

        let input_result = {
            let mut credit_card_validator = CreditCardValidator {
                reader: &input[..],
                writer: &mut output,
            };

            credit_card_validator.valid_digits()
        };

        assert_eq!(input_result.is_ok, false);
    }
    #[test]
    fn test_passed_14digits() {
        let input = b"40055192000000";
        let mut output: Vec<u8> = Vec::new();

        let input_result = {
            let mut credit_card_validator = CreditCardValidator {
                reader: &input[..],
                writer: &mut output,
            };

            credit_card_validator.valid_digits()
        };

        assert_eq!(input_result.is_ok, true);
    }
    #[test]
    fn test_passed_19digits() {
        let input = b"4005519200000011111";
        let mut output: Vec<u8> = Vec::new();

        let input_result = {
            let mut credit_card_validator = CreditCardValidator {
                reader: &input[..],
                writer: &mut output,
            };

            credit_card_validator.valid_digits()
        };

        assert_eq!(input_result.is_ok, true);
    }
    #[test]
    fn test_passed_Visa() {
        let input = b"4005519200000004";
        let mut output: Vec<u8> = Vec::new();

        let result = {
            let mut credit_card_validator = CreditCardValidator {
                reader: &input[..],
                writer: &mut output,
            };

            let number = credit_card_validator.valid_digits();
            credit_card_validator.check_card_number(number)
        };

        assert_eq!(result, "VISA".to_string());
    }
    #[test]
    fn test_passed_MasterCard() {
        let input = b"2223000048400011";
        let mut output: Vec<u8> = Vec::new();

        let result = {
            let mut credit_card_validator = CreditCardValidator {
                reader: &input[..],
                writer: &mut output,
            };

            let number = credit_card_validator.valid_digits();
            credit_card_validator.check_card_number(number)
        };

        assert_eq!(result, "MasterCard".to_string());
    }
    #[test]
    fn test_passed_AmericanExpress() {
        let input = b"371449635398431";
        let mut output: Vec<u8> = Vec::new();

        let result = {
            let mut credit_card_validator = CreditCardValidator {
                reader: &input[..],
                writer: &mut output,
            };

            let number = credit_card_validator.valid_digits();
            credit_card_validator.check_card_number(number)
        };

        assert_eq!(result, "AmericanExpress".to_string());
    }
    #[test]
    fn test_passed_AnotherBrand() {
        let input = b"36259600000004";
        let mut output: Vec<u8> = Vec::new();

        let result = {
            let mut credit_card_validator = CreditCardValidator {
                reader: &input[..],
                writer: &mut output,
            };

            let number = credit_card_validator.valid_digits();
            credit_card_validator.check_card_number(number)
        };

        assert_eq!(result, "another_brand".to_string());
    }
    #[test]
    fn test_failed_invalid() {
        let input = b"36159600000004";
        let mut output: Vec<u8> = Vec::new();

        let result = {
            let mut credit_card_validator = CreditCardValidator {
                reader: &input[..],
                writer: &mut output,
            };

            let number = credit_card_validator.valid_digits();
            credit_card_validator.check_card_number(number)
        };

        assert_eq!(result, "INVALID".to_string());
    }
}
