fn main() {
    let mut input = String::new();
    let mut expression : Vec<String> = Vec::new();
    std::io::stdin().read_line(&mut input).unwrap();
    for i in input.split_whitespace() {
        expression.push(String::from(i));
    }
    let mut offset : usize = 0;
    for mut i in 0..expression.len() {
        println!("INDEX IS {}", i - offset);
        let before : String = expression[i - offset].clone();
        println!("BEFORE: {}  |  ", expression[i - offset]);

        expression[i - offset] = String::from(evaluate(i - offset, &mut expression).to_string());

        let after : String = expression[i - offset].clone();
        println!("AFTER: {}  |  ", expression[i - offset]);
        
        print!("OVERALL: ");
        for j in expression.clone() {
            print!("{} ", j);
        }

        if before != after {
            println!("I AM DELETING PREVIOUS TWO");
            expression.remove(0);
            expression.remove(0);
            offset += 2;
        }

        print!("OVERALL AFTER DELETION: ");
        for j in expression.clone() {
            print!("{} ", j);
        }

        println!();
    }
}

fn evaluate(inp : usize, expr : &mut Vec<String>) -> f64 {
    if expr[inp].parse::<f64>().is_err() {
        println!("ack!");
        if expr[inp] == "+" {
            evaluate(inp - 2, expr) + evaluate(inp - 1, expr)
        } else {
            println!("  super ack!!");
            -0.123456789
        }
    } else {
        println!("yey");
        expr[inp].parse::<f64>().unwrap()
    }
}