use rand::prelude::*;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Substract,
    Multiply
}

impl Operation {
    fn symbol(&self) -> String {
        match self {
            Operation::Add => " + ".to_string(),
            Operation::Substract => " - ".to_string(),
            Operation::Multiply => " * ".to_string(),
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Operation::Add),
            '-' => Some(Operation::Substract),
            '*' => Some(Operation::Multiply),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum RollExp {
    Number(Operation, i32),
    Dice(Operation, i32, i32),
}

fn compute(expression: Vec<RollExp>) -> (i32, String) {
    let mut result = 0;
    let mut text = String::new();
    let mut rng = rand::thread_rng();

    for subroll in expression.iter() {
        match subroll {
            RollExp::Dice(op, nb, dice) => {
                let mut subtext = String::new();
                let mut rolled: i32 = 0;
                for _ in 0..*nb {
                    let roll = rng.gen_range(1..=*dice);
                    rolled += roll;
                    subtext = format!("{}{}{}", subtext, if subtext.is_empty() { "" } else { " + " }, roll);
                }
                let symbol = if text.is_empty() { "".to_string() } else { op.symbol() };
                let details = if *nb > 1 { format!(" ({subtext})") } else { "".to_string() };
                text = format!("{text}{symbol}{rolled}{details}");
                match op {
                    Operation::Add => result += rolled,
                    Operation::Substract => result -= rolled,
                    Operation::Multiply => result *= rolled,
                }
            }
            RollExp::Number(op, nb) => {
                match op {
                    Operation::Add => result += *nb,
                    Operation::Substract => result -= *nb,
                    Operation::Multiply => result *= *nb,
                }
                text = format!("{}{}{}", text,
                               if text.is_empty() { "".to_string() } else { op.symbol() },
                               nb);
            }
        }
    }

    return (result, text);
}

enum ParseElement {
    Character(char),
    Number(i32),
}

fn parse(str: &String) -> Result<Vec<RollExp>, String> {
    let mut buffer: i32 = 0;
    let mut elements = Vec::new();
    for (i, char) in str.chars().enumerate() {
        match char {
            ' ' => {}
            'd' => elements.push(ParseElement::Character('d')),
            '+' => elements.push(ParseElement::Character('+')),
            '*' => elements.push(ParseElement::Character('*')),
            '-' => elements.push(ParseElement::Character('-')),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                buffer = buffer * 10 + char.to_digit(10).unwrap() as i32;
                if str.chars().nth(i + 1).is_none() || !str.chars().nth(i + 1).unwrap().is_numeric() {
                    elements.push(ParseElement::Number(buffer));
                    buffer = 0;
                }
            }
            _ => return Err(format!("Unexpected character. ({})", char.to_string()))
        }
    }

    let mut expression = Vec::new();
    let mut lastOperation = Operation::Add;

    for (i, elem) in elements.iter().enumerate() {
        match elem {
            ParseElement::Character('d') => {
                let n1 = if i == 0 { 1 } else {
                    match elements.get(i - 1) {
                        Some(ParseElement::Number(n)) => *n,
                        _ => 1
                    }
                };
                let n2 = match elements.get(i + 1) {
                    Some(ParseElement::Number(n)) => *n,
                    _ => return Err("Dice value expected".to_string())
                };
                expression.push(RollExp::Dice(lastOperation, n1, n2));
            }
            ParseElement::Character(c) => {
                lastOperation = match Operation::from_char(*c) {
                    Some(op) => op,
                    _ => return Err("Invalid operation".to_string())
                };
                match elements.get(i + 1) {
                    Some(ParseElement::Character('d')) => continue,
                    _ => {},
                }
                match elements.get(i + 2) {
                    Some(ParseElement::Character('d')) => continue,
                    _ => {
                        let n = match elements.get(i + 1) {
                            Some(ParseElement::Number(n)) => *n,
                            _ => return Err("Number expected".to_string())
                        };
                        expression.push(RollExp::Number(lastOperation, n));
                    }
                }
            }
            ParseElement::Number(nb) => {
                if i == 0 {
                    match elements.get(i + 1) {
                        Some(ParseElement::Character('d')) => continue,
                        _ => expression.push(RollExp::Number(lastOperation, *nb)),
                    }
                }
            }
            _ => return Err("Can't parse expression".to_string())
        }
    }

    return if expression.is_empty() { Err("Empty expression.".to_string()) } else { Ok(expression) }
}

pub fn roll(str: &String) -> Result<String, String> {
    return match parse(str) {
        Ok(expression) => {
            let result = compute(expression);
            Ok(format!("**{}** [{}]", result.0, result.1))
        }
        Err(message) => Err(message)
    }
}