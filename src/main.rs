fn main() {
    let mut input = String::new();
    let mut expression : Vec<String> = Vec::new();
    std::io::stdin().read_line(&mut input).unwrap();
    for i in input.split_whitespace() {
        expression.push(String::from(i));
    };
    let mut info : Vec<usize> = Vec::new();
    let mut keywords : Vec<String> = Vec::new();
    keywords.push(String::from("+"));
    keywords.push(String::from("-"));
    keywords.push(String::from("*"));
    keywords.push(String::from("/"));
    keywords.push(String::from("sin"));
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
        /*
        println!("INFO");
        for i in &info {
            print!("{}", i);
        }
        println!("LEFTOVER");
        for i in expression.iter().enumerate() {
            if !info.contains(&i.0) {
                print!("{}", i.1);
            }
        }
        */
    }
}

fn evaluate(inp : usize, expr : &mut Vec<String>, info : &mut Vec<usize>) -> Option<f64> {
    if expr[inp].parse::<f64>().is_err() {
        //println!("ack!");
        if expr[inp] == "+" {
            info.push(inp);
            if inp <= 1 {
                return None;
            }
            Some(evaluate(inp - 2, expr, info)? + evaluate(inp - 1, expr, info)?)

        } else if expr[inp] == "-" {
            info.push(inp);
            if inp <= 1 {
                return None;
            }
            Some(evaluate(inp - 2, expr, info)? - evaluate(inp - 1, expr, info)?)

        } else if expr[inp] == "*" {
            info.push(inp);
            if inp <= 1 {
                return None;
            }
            Some(evaluate(inp - 2, expr, info)? * evaluate(inp - 1, expr, info)?)

        } else if expr[inp] == "/" {
            info.push(inp);
            if inp <= 1 {
                return None;
            }
            Some(evaluate(inp - 2, expr, info)? / evaluate(inp - 1, expr, info)?)

        } else if expr[inp] == "sin" {
            info.push(inp);
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