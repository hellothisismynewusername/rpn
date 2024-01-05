#[derive(Clone)]
struct Fraction {
    numer : String,
    denom : String
}

impl Fraction {
    fn is_undefined(&self) -> bool {
        if self.denom == "0.0" || self.denom == "0." || self.denom == "0" { true } else { false }
    }
    fn over_one(inp : String) -> Fraction {
        Fraction { numer: inp, denom: "1".to_string() }
    }
    fn as_decimal(&self) -> Option<f64> {
        if self.is_undefined() {
            None
        } else {
            if self.numer.parse::<f64>().is_err() || self.denom.parse::<f64>().is_err() {
                None
            } else {
                Some(self.numer.parse::<f64>().unwrap() / self.denom.parse::<f64>().unwrap())
            }
        }
    }
    fn from_decimal(inp : String) -> Option<Fraction> { //please ensure the inp is a number
        if inp.parse::<f64>().is_err() {
            None
        } else {
            let num : f64 = inp.parse::<f64>().unwrap();
            let mut multiplier : u64 = 1;
            let mut counting : bool = false;
            for i in 0..inp.len() {
                if counting {
                    multiplier *= 10;
                }
                if inp.chars().nth(i).unwrap() == '.' {
                    counting = true;
                }
            }
            Some( Fraction {
                numer: (num * multiplier as f64).to_string(),
                denom: (multiplier as f64).to_string()
            })
        }
    }
}

fn main() {
    let mut verbose : bool = true;
    for arg in std::env::args() {
        if arg == "-v" {
            verbose = true;
        }
    }

    let mut input = String::new();
    let mut expression : Vec<String> = Vec::new();
    std::io::stdin().read_line(&mut input).unwrap();
    for i in input.split_whitespace() {
        expression.push(String::from(i));
    };
    let mut keywords : Vec<String> = Vec::new();
    keywords.push(String::from("+"));
    keywords.push(String::from("-"));
    keywords.push(String::from("*"));
    keywords.push(String::from("/"));
    keywords.push(String::from("sin"));
    keywords.push(String::from("cos"));
    keywords.push(String::from("tan"));
    keywords.push(String::from("csc"));
    keywords.push(String::from("sec"));
    keywords.push(String::from("cot"));
    keywords.push(String::from("pow"));
    keywords.push(String::from("root"));

    /*
    let mut info : Vec<usize> = Vec::new();
    let mut met_at_least_one_keyword = false;
    for i in 0..expression.len() {
        for keyword in &keywords {
            if &expression[i] == keyword {
                met_at_least_one_keyword = true;
            }
        }
        if expression[i].parse::<f64>().is_err() && !met_at_least_one_keyword {
            println!("'{}' could not be parsed", expression[i]);
        }
    }
    if evaluate(expression.len() - 1, &mut expression, &mut info).is_none() {
        println!("The expression could not be calculated");
    } else {
        println!("{}", evaluate(expression.len() - 1, &mut expression, &mut info).unwrap());
        
        println!("LEFTOVER:");
        for i in expression.iter().enumerate() {
            if !info.contains(&i.0) {
                print!("{}", i.1);
            }
        }
        
    }
    */

    let mut stack : Vec<Fraction> = Vec::new();
    let mut counter : usize = 0;
    while counter < expression.len() {
        if verbose {
            println!("Stack on iteration {} BEFORE changes is: ", counter);
            for i in &stack {
                print!("({}/{})", i.numer, i.denom);
            }
        }
        if verbose { println!(); }
        
        if is_a_keyword(&expression[counter], &keywords) {
            stack.push(Fraction::over_one(expression[counter].clone()));
        } else {
            stack.push(Fraction::from_decimal(expression[counter].clone()).unwrap_or(Fraction::over_one(String::from("Err"))));
        }

        let stack_length = stack.len();
        let mut inputs_amount : usize = 0;
        stack[stack_length - 1] = if is_a_keyword(&stack[stack.len() - 1].numer, &keywords) {
            if evaluate(stack_length - 1, &stack, &mut inputs_amount, verbose).is_none() {
                Fraction::over_one("Err".to_string())
            } else {
                evaluate(stack_length - 1, &stack, &mut inputs_amount, verbose).unwrap()
            }
        } else {
            stack[stack_length - 1].clone()
        };
        if verbose { println!("input amount is {}", inputs_amount); }
        for _i in 0..inputs_amount {
            stack.remove(stack_length - inputs_amount - 1);
        };
        if verbose {
            println!("Stack on iteration {} AFTER changes is: ", counter);
            for i in &stack {
                print!("({}/{})", i.numer, i.denom);
            }
        }
        counter += 1;
    }
    println!("\nOutput:");
    for i in &stack {
        print!("({}/{}) = ", i.numer, i.denom);
        if i.as_decimal().is_none() {
            print!("Err, ");
        } else {
            print!("{}, ", i.as_decimal().unwrap());
        }
    }
}

fn is_a_keyword(inp : &String, keywords : &Vec<String>) -> bool {
    for keyword in keywords {
        if inp == keyword {
            return true;
        }
    }
    return false;
}

fn evaluate(inp : usize, expr : &Vec<Fraction>, inputs_amount : &mut usize, verbose : bool) -> Option<Fraction> {
    if verbose { println!("Im at {}, and i see ({}/{})", inp, expr[inp].numer, expr[inp].denom); }
    if expr[inp].numer.parse::<f64>().is_err() {
        //println!("ack!");
        if expr[inp].numer == "+" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: ((evaluate(inp - 2, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() * evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()) +
                    evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() * evaluate(inp - 2, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).to_string(),
                    denom: (evaluate(inp - 2, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap() * evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).to_string()
                })
            }
        } else if expr[inp].numer == "-" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: ((evaluate(inp - 2, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() * evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()) -
                    evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() * evaluate(inp - 2, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).to_string(),
                    denom: (evaluate(inp - 2, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap() * evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).to_string()
                })
            }
        } else if expr[inp].numer == "/" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: (evaluate(inp - 2, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() * evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).to_string(),
                    denom: (evaluate(inp - 2, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap() * evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap()).to_string()
                })
            }
        } else if expr[inp].numer == "*" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: (evaluate(inp - 2, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() * evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap()).to_string(),
                    denom: (evaluate(inp - 2, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap() * evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).to_string()
                })
            }
        } else {
            *inputs_amount = 0;
            None
        }
    } else {
        Some(Fraction {
            numer: expr[inp].numer.clone(),
            denom: expr[inp].denom.clone()
        })
    }
}

/*

BOO OLD VERSION THAT DOESNT USE FRACTIONS AND SAYS THAT 0.1 + 0.2 = 0.3000000000000000004 OR SOMETHING

fn main() {
    let mut verbose : bool = false;
    for arg in std::env::args() {
        if arg == "-v" {
            verbose = true;
        }
    }

    let mut input = String::new();
    let mut expression : Vec<String> = Vec::new();
    std::io::stdin().read_line(&mut input).unwrap();
    for i in input.split_whitespace() {
        expression.push(String::from(i));
    };
    let mut keywords : Vec<String> = Vec::new();
    keywords.push(String::from("+"));
    keywords.push(String::from("-"));
    keywords.push(String::from("*"));
    keywords.push(String::from("/"));
    keywords.push(String::from("sin"));
    keywords.push(String::from("cos"));
    keywords.push(String::from("tan"));
    keywords.push(String::from("csc"));
    keywords.push(String::from("sec"));
    keywords.push(String::from("cot"));
    keywords.push(String::from("pow"));
    keywords.push(String::from("root"));

    /*
    let mut info : Vec<usize> = Vec::new();
    let mut met_at_least_one_keyword = false;
    for i in 0..expression.len() {
        for keyword in &keywords {
            if &expression[i] == keyword {
                met_at_least_one_keyword = true;
            }
        }
        if expression[i].parse::<f64>().is_err() && !met_at_least_one_keyword {
            println!("'{}' could not be parsed", expression[i]);
        }
    }
    if evaluate(expression.len() - 1, &mut expression, &mut info).is_none() {
        println!("The expression could not be calculated");
    } else {
        println!("{}", evaluate(expression.len() - 1, &mut expression, &mut info).unwrap());
        
        println!("LEFTOVER:");
        for i in expression.iter().enumerate() {
            if !info.contains(&i.0) {
                print!("{}", i.1);
            }
        }
        
    }
    */

    let mut stack : Vec<String> = Vec::new();
    let mut counter : usize = 0;
    while counter < expression.len() {
        if verbose {
            println!("Stack on iteration {} BEFORE changes is: ", counter);
            for i in &stack {
                print!("{} ", i);
            }
        }
        if verbose { println!(); }
        stack.push(expression[counter].clone());
        let stack_length = stack.len();
        let mut inputs_amount : usize = 0;
        stack[stack_length - 1] = if is_a_keyword(&stack[stack.len() - 1], &keywords) {
            if evaluate(stack_length - 1, &stack, &mut inputs_amount, verbose).is_none() {
                "Err".to_string()
            } else {
                evaluate(stack_length - 1, &stack, &mut inputs_amount, verbose).unwrap().to_string()
            }
        } else {
            stack[stack_length - 1].clone()
        };
        if verbose { println!("input amount is {}", inputs_amount); }
        for _i in 0..inputs_amount {
            stack.remove(stack_length - inputs_amount - 1);
        };
        if verbose {
            println!("Stack on iteration {} AFTER changes is: ", counter);
            for i in &stack {
                print!("{} ", i);
            }
        }
        counter += 1;
    }
    println!("Output:");
    for i in &stack {
        print!("{} ", i);
    }
}

fn is_a_keyword(inp : &String, keywords : &Vec<String>) -> bool {
    for keyword in keywords {
        if inp == keyword {
            return true;
        }
    }
    return false;
}

fn evaluate(inp : usize, expr : &Vec<String>, inputs_amount : &mut usize, verbose : bool) -> Option<f64> {
    if verbose { println!("Im at {}, and i see {}", inp, expr[inp]); }
    if expr[inp].parse::<f64>().is_err() {
        //println!("ack!");
        if expr[inp] == "+" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(evaluate(inp - 2, expr, inputs_amount, verbose)? + evaluate(inp - 1, expr, inputs_amount, verbose)?)
            }

        } else if expr[inp] == "-" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(evaluate(inp - 2, expr, inputs_amount, verbose)? - evaluate(inp - 1, expr, inputs_amount, verbose)?)
            }

        } else if expr[inp] == "*" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(evaluate(inp - 2, expr, inputs_amount, verbose)? * evaluate(inp - 1, expr, inputs_amount, verbose)?)
            }

        } else if expr[inp] == "/" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(evaluate(inp - 2, expr, inputs_amount, verbose)? / evaluate(inp - 1, expr, inputs_amount, verbose)?)
            }

        } else if expr[inp] == "pow" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(evaluate(inp - 2, expr, inputs_amount, verbose)?.powf(evaluate(inp - 1, expr, inputs_amount, verbose)?))
            }

        } else if expr[inp] == "root" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(evaluate(inp - 2, expr, inputs_amount, verbose)?.powf(1. / evaluate(inp - 1, expr, inputs_amount, verbose)?))
            }

        } else if expr[inp] == "sin" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(evaluate(inp - 1, expr, inputs_amount, verbose)?.sin())
            }

        } else if expr[inp] == "cos" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(evaluate(inp - 1, expr, inputs_amount, verbose)?.cos())
            }

        } else if expr[inp] == "tan" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(evaluate(inp - 1, expr, inputs_amount, verbose)?.tan())
            }

        } else if expr[inp] == "csc" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(1. / (evaluate(inp - 1, expr, inputs_amount, verbose)?.sin()))
            }

        } else if expr[inp] == "sec" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(1. / (evaluate(inp - 1, expr, inputs_amount, verbose)?.cos()))
            }

        } else if expr[inp] == "cot" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(1. / (evaluate(inp - 1, expr, inputs_amount, verbose)?.tan()))
            }

        } else {
            *inputs_amount = 0;
            None
        }
    } else {
        Some(expr[inp].parse::<f64>().unwrap())
    }
}

/*
fn evaluate(inp : usize, expr : &mut Vec<String>, info : &mut Vec<usize>) -> Option<f64> {
    println!("Im at {}, and i see {}", inp, expr[inp]);
    info.push(inp);
    if expr[inp].parse::<f64>().is_err() {
        //println!("ack!");
        if expr[inp] == "+" {
            if inp <= 1 {
                return None;
            }
            Some(evaluate(inp - 2, expr, info)? + evaluate(inp - 1, expr, info)?)

        } else if expr[inp] == "-" {
            if inp <= 1 {
                return None;
            }
            Some(evaluate(inp - 2, expr, info)? - evaluate(inp - 1, expr, info)?)

        } else if expr[inp] == "*" {
            println!("Doing * on {} and {}, which is {} and {}", inp - 2, inp - 1, expr[inp - 2], expr[inp - 1]);
            if inp <= 1 {
                return None;
            }
            Some(evaluate(inp - 2, expr, info)? * evaluate(inp - 1, expr, info)?)

        } else if expr[inp] == "/" {
            if inp <= 1 {
                return None;
            }
            Some(evaluate(inp - 2, expr, info)? / evaluate(inp - 1, expr, info)?)

        } else if expr[inp] == "sin" {
            println!("Doing sin on {}, which is {}", inp - 1, expr[inp - 1]);
            if inp <= 0 {
                return None;
            }
            Some(evaluate(inp - 1, expr, info)?.sin())

        } else {
            //println!("  super ack!!");
            None
        }

        /*
        match &expr[inp] as &str {
            "+" => evaluate(inp - 2, expr) + evaluate(inp - 1, expr),
            _ => -0.123456789,
        }
        */
    } else {
        //println!("yey");
        Some(expr[inp].parse::<f64>().unwrap())
    }
}
*/
*/