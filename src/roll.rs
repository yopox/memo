use rand::prelude::*;

enum RollExp {
    Dice(i32, i32),
    Add(i32),
    Factor(i32),
}

fn compute(expression: Vec<RollExp>) -> (i32, String) {
    let mut result = 0;
    let mut text = String::new();
    let mut rng = rand::thread_rng();

    for subroll in expression.iter() {
        match *subroll {
            RollExp::Dice(nb, dice) => {
                let mut subtext = String::new();
                for _ in 0..nb {
                    let roll = rng.gen_range(1..=dice);
                    result += roll;
                    subtext = format!("{}{}{}", subtext, if subtext.is_empty() { "" } else { " + " }, roll);
                }
                if nb > 1 {
                    text = format!("{}{}{} ({})", text,
                                   if text.is_empty() { "" } else { " + " },
                                   result, subtext);
                } else {
                    text = format!("{}{}{}", text,
                                   if text.is_empty() { "" } else { " + " },
                                   result);
                }
            }
            RollExp::Add(constant) => {
                result += constant;
                if text.is_empty() {
                    text = format!("{}", constant);
                } else {
                    text = format!("{} {} {}", text,
                                   if constant >= 0 { "+" } else { "-" },
                                   if constant >= 0 { constant } else { constant * -1 });
                }
            }
            RollExp::Factor(constant) => {
                result *= constant;
                text = format!("{} * {}", text, constant);
            }
        }
    }

    return (result, text);
}

enum ParseState {
    INIT,
    NB,
    D,
    MINUS,
    PLUS,
    TIMES,
}

enum ParseElement {
    Char(char),
    Number(i32),
}

fn parse(str: &String) -> Result<Vec<RollExp>, String> {
    let mut expression = Vec::new();

    let mut buffer: i32 = 0;
    let mut elements = Vec::new();
    for (i, char) in str.chars().enumerate() {
        match char {
            ' ' => {}
            'd' => elements.push(ParseElement::Char('d')),
            '+' => elements.push(ParseElement::Char('+')),
            '*' => elements.push(ParseElement::Char('*')),
            '-' => elements.push(ParseElement::Char('-')),
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

    let mut state = ParseState::INIT;
    let mut buffer: i32 = 0;

    for element in elements {
        match state {
            ParseState::INIT => match element {
                ParseElement::Char('+') => state = ParseState::PLUS,
                ParseElement::Char('*') => state = ParseState::TIMES,
                ParseElement::Char('-') => state = ParseState::MINUS,
                ParseElement::Char('d') => {
                    buffer = 1;
                    state = ParseState::D;
                }
                ParseElement::Number(nb) => {
                    buffer = nb;
                    state = ParseState::NB;
                }
                _ => return Err(format!("Bad expression."))
            }
            ParseState::NB => match element {
                ParseElement::Char('+') => {
                    expression.push(RollExp::Add(buffer));
                    state = ParseState::PLUS;
                },
                ParseElement::Char('*') => {
                    expression.push(RollExp::Add(buffer));
                    state = ParseState::TIMES
                },
                ParseElement::Char('-') => {
                    expression.push(RollExp::Add(buffer));
                    state = ParseState::MINUS
                },
                ParseElement::Char('d') => state = ParseState::D,
                _ => return Err(format!("Bad expression."))
            }
            ParseState::D => match element {
                ParseElement::Number(nb) => {
                    if buffer > 0 {
                        expression.push(RollExp::Dice(buffer, nb));
                    }
                    if buffer > 500 { return Err(format!("Dice number too high.")) }
                    state = ParseState::INIT;
                }
                _ => return Err(format!("Bad expression."))
            }
            ParseState::MINUS => match element {
                ParseElement::Number(nb) => {
                    expression.push(RollExp::Add(nb * -1));
                    state = ParseState::INIT;
                }
                _ => return Err(format!("Bad expression."))
            }
            ParseState::PLUS => match element {
                ParseElement::Number(nb) => {
                    expression.push(RollExp::Add(nb));
                    state = ParseState::INIT;
                }
                _ => return Err(format!("Bad expression."))
            }
            ParseState::TIMES => match element {
                ParseElement::Number(nb) => {
                    expression.push(RollExp::Factor(nb));
                    state = ParseState::INIT;
                }
                _ => return Err(format!("Bad expression."))
            }
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