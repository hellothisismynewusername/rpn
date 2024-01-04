use std::env;

struct Info {
    argc : u8,
}

impl Info {
    fn default() -> Info {
        Info {
            argc: 2,
        }
    }
}

fn main() {
    let args = env::args();
    let mut show_steps = false;
    for arg in args {
        if arg == "-v" {
            show_steps = true;
        }
    }

    let mut input = String::new();
    let mut expression : Vec<String> = Vec::new();
    std::io::stdin().read_line(&mut input).unwrap();
    for i in input.split_whitespace() {
        expression.push(String::from(i));
    }
    let mut offset : usize = 0;
    let mut info : Info = Info::default();
    for i in 0..expression.len() {
        if show_steps {
            println!("INDEX IS {}", i - offset);
        }
        let before : String = expression[i - offset].clone();
        if show_steps {
            println!("BEFORE: {}  |  ", expression[i - offset]);
        }

        if evaluate(i - offset, &mut expression, &mut info).is_none() {
            println!("Skipping unparsable part: {}", expression[i - offset]);
            if !show_steps {
                println!();
            }
            if show_steps {
                println!("{}", offset);
            }
            expression.remove(i - offset);
            offset += 1;

            if show_steps {
                print!("OVERALL AFTER DELETION: ");
                for j in expression.clone() {
                    print!("{} ", j);
                }
            }
        } else {
            expression[i - offset] = String::from(evaluate(i - offset, &mut expression, &mut info).unwrap().to_string());
            let after : String = expression[i - offset].clone();
            if show_steps {
                println!("AFTER: {}  |  ", expression[i - offset]);
            }
            
            if show_steps {
                print!("OVERALL: ");
                for j in expression.clone() {
                    print!("{} ", j);
                }
            }

            if before.parse::<f64>().is_err() && before != after {
                if show_steps {
                    println!("I AM DELETING PREVIOUS {}", info.argc);
                }
                for _j in 0..info.argc {
                    expression.remove(i - offset - info.argc as usize);
                }
                offset += info.argc as usize;
            }
            
            /*
            if before != after {
                println!("I AM DELETING PREVIOUS {}", info.argc);
                for _j in 0..info.argc {
                    expression.remove(i - offset - 2);
                }
                offset += info.argc as usize;
            }
            */

            if show_steps {
                print!("OVERALL AFTER DELETION: ");
                for j in expression.clone() {
                    print!("{} ", j);
                }
            }

            if show_steps {
                println!();
            }
        }
    }
    println!("{}", expression[expression.len() - 1]);
}

fn evaluate(inp : usize, expr : &mut Vec<String>, info : &mut Info) -> Option<f64> {
    if expr[inp].parse::<f64>().is_err() {
        //println!("ack!");
        if expr[inp] == "+" {
            info.argc = 2;
            if inp <= 1 {
                return None;
            }
            Some(evaluate(inp - 2, expr, info)? + evaluate(inp - 1, expr, info)?)

        } else if expr[inp] == "-" {
            info.argc = 2;
            if inp <= 1 {
                return None;
            }
            Some(evaluate(inp - 2, expr, info)? - evaluate(inp - 1, expr, info)?)

        } else if expr[inp] == "*" {
            info.argc = 2;
            if inp <= 1 {
                return None;
            }
            Some(evaluate(inp - 2, expr, info)? * evaluate(inp - 1, expr, info)?)

        } else if expr[inp] == "/" {
            info.argc = 2;
            if inp <= 1 {
                return None;
            }
            Some(evaluate(inp - 2, expr, info)? / evaluate(inp - 1, expr, info)?)

        } else if expr[inp] == "sin" {
            info.argc = 1;
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