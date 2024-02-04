use std::path::Path;
use std::fs::File;
use std::io::{Write, Read};

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
    fn simplify(&self) -> Fraction {
        if self.numer.parse::<i64>().is_err() || self.denom.parse::<i64>().is_err() {
            return Fraction {
                numer: self.numer.clone(),
                denom: self.denom.clone()
            };
        }
        let numer: i64 = self.numer.parse::<i64>().unwrap();
        let denom: i64 = self.denom.parse::<i64>().unwrap();
        let mut new_numer : i64 = numer;
        let mut new_denom : i64 = denom;
        let mut divisor : u64 = 2;
        if numer <= denom {
            while (divisor as i64) <= numer {
                if (new_numer as f64) % (divisor as f64) == 0. && (new_denom as f64) % (divisor as f64) == 0. {
                    new_numer /= divisor as i64;
                    new_denom /= divisor as i64;
                }
                divisor += 1;
            }
        } else {
            while (divisor as i64) <= denom {
                if (new_numer as f64) % (divisor as f64) == 0. && (new_denom as f64) % (divisor as f64) == 0. {
                    new_numer /= divisor as i64;
                    new_denom /= divisor as i64;
                }
                divisor += 1;
            }
        }
        Fraction {
            numer: new_numer.to_string(),
            denom: new_denom.to_string()
        }
    }
}

struct Macr {
    name : String,
    vars : Vec<String>,
    contents : Vec<String>,
}

impl Macr {
    fn from(name : &str, vars : &str, contents_inp : &str) -> Macr {
        let mut buf = Vec::new();
        for i in contents_inp.split_whitespace() {
            buf.push(String::from(i));
        };
        buf.reverse();
        let mut real_vars = Vec::new();
        for i in vars.split_whitespace() {
            real_vars.push(String::from(i));
        };
        Macr {
            name: name.to_string(),
            vars: real_vars,
            contents: buf
        }
    }
}

fn main() {
    let mut verbose : bool = false;
    for arg in std::env::args() {
        if arg == "-v" {
            verbose = true;
        }
    }

    let macr_starter_text = ">";
    let macr_ending_text = "<";
    
    let mut keywords : Vec<String> = Vec::new();
    keywords.push(String::from("+"));   //the 4 basic operations should keep full accuracy I think
    keywords.push(String::from("-"));
    keywords.push(String::from("*"));
    keywords.push(String::from("/"));
    keywords.push(String::from("sin"));
    keywords.push(String::from("cos"));
    keywords.push(String::from("tan"));
    keywords.push(String::from("asin"));
    keywords.push(String::from("acos"));
    keywords.push(String::from("atan"));
    keywords.push(String::from("atan2"));
    keywords.push(String::from("csc"));
    keywords.push(String::from("sec"));
    keywords.push(String::from("cot"));
    keywords.push(String::from("pow"));
    keywords.push(String::from("root"));
    keywords.push(String::from("log"));
    keywords.push(String::from("simplify")); //this one also keeps full accuracy

    let mut macrs : Vec<Macr> = Vec::new();

    if !Path::new("macros.txt").exists() {
        let mut writefile = if !File::create("macros.txt").is_err() {
            File::create("macros.txt").unwrap()
        } else {
            File::create("macros.txt").expect("Couldn't make file 'macros.txt'")
        };
        let buf : [u8; 1] = [0; 1];
        let res = writefile.write_all(&buf);
        if res.is_err() {
            if verbose { println!("Couldn't write to file 'macros.txt'"); }
        }
    }

    let mut readfile = File::open("macros.txt").expect("Couldn't open 'macros.txt'");
    let mut readfile_buf : Vec<u8> = Vec::new();
    readfile.read_to_end(&mut readfile_buf).expect("Couldn't read 'macros.txt'");
    let readfile_as_text : String = String::from_utf8(readfile_buf).expect("Couldn't get unicode chars from file");
    if verbose { println!("Buffer is: {}", readfile_as_text); }
    let mut readfile_split : Vec<String> = readfile_as_text.split_ascii_whitespace().into_iter().map(|x| {
        x.to_string()
    }).collect();
    readfile_split.push(" ".to_string());
    for i in 0..readfile_split.len() {
        if verbose { println!("item: {}", readfile_split[i]); }
        if readfile_split[i] == macr_starter_text.to_string() {
            let macr_name = &readfile_split[i + 1];

            let mut j = 0;
            let mut macr_vars : String = String::new();
            while readfile_split[i + j + 2] != "=" {
                macr_vars.push_str(&readfile_split[i + j + 2]);
                macr_vars.push_str(" ");
                j += 1;
            }

            let mut k = 0;
            let mut macr_contents : String = String::new();
            while readfile_split[i + j + k + 3] != macr_ending_text.to_string() && i + j + k + 3 < readfile_split.len() - 1 {
                macr_contents.push_str(&readfile_split[i + j + k + 3]);
                macr_contents.push_str(" ");
                k += 1;
            }

            macrs.push(Macr::from(macr_name, &macr_vars, &macr_contents));
            
            if verbose {
                println!("now the macrs are:");
                for macr in &macrs {
                    println!("{}, vars are: ", macr.name);
                    for var in &macr.vars {
                        print!("{} ", var);
                    }
                    println!(", contents are: ");
                    for content in &macr.contents {
                        print!("{} ", content);
                    }
                    println!("\nend of this fun");
                }
            }
        }
    }

    loop {

        let mut input = String::new();
        let mut expression : Vec<String> = Vec::new();
        std::io::stdin().read_line(&mut input).unwrap();
        for i in input.split_whitespace() {
            expression.push(String::from(i));
        };

    
        
        let mut i : usize = 0;
        while i < expression.len() {
            let mut do_not_increment : bool = false;
            for mac in &macrs {
                if mac.name == expression[i] {
                    if mac.vars.len() > 0 {
                        if  i > mac.vars.len() - 1 {
                            for content in &mac.contents {
                                expression.insert(i + 1, content.to_string());
                            }
                            if verbose {
                                println!("after insertion");
                                for item in &expression {
                                    print!("{} ", item);
                                }
                                println!();
                            }
                            for j in 0..mac.vars.len() {
                                //actually replacing moment
                                for item_num in 0..expression.len() {
                                    if expression[item_num] == mac.vars[j] {
                                        expression[item_num] = expression[i - mac.vars.len()].clone();
                                    }
                                }
                                //delete it
                                expression.remove(i - mac.vars.len());
                            }
                            expression.remove(i - mac.vars.len());
                            if verbose { 
                                println!("after deletion");
                                for item in &expression {
                                    print!("{} ", item);
                                }
                                println!();
                            }
                            i = 0; //reset to start in case there's more macs in the spread out mac
                            do_not_increment = true;
                        }
                    } else {
                        for content in &mac.contents {
                            expression.insert(i + 1, content.to_string());
                        }
                        if verbose { 
                            println!("after insertion");
                            for item in &expression {
                                print!("{} ", item);
                            }
                            println!();
                        }
                        expression.remove(i - mac.vars.len());
                        if verbose { 
                            println!("after deletion");
                            for item in &expression {
                                print!("{} ", item);
                            }
                            println!();
                        }
                        i = 0; //reset to start in case there's more macs in the spread out mac
                        do_not_increment = true;
                    }
                }
            }
            if !do_not_increment {
                i += 1;
            }
        }
        

        for item in &expression {
            print!("{} ", item);
        }
        println!();

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
        println!();
        println!();
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
        } else if expr[inp].numer == "sin" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: ((evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).sin()).to_string(),
                    denom: "1".to_string()
                })
            }
        } else if expr[inp].numer == "cos" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: ((evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).cos()).to_string(),
                    denom: "1".to_string()
                })
            }
        } else if expr[inp].numer == "tan" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: ((evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).tan()).to_string(),
                    denom: "1".to_string()
                })
            }
        } else if expr[inp].numer == "asin" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: ((evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).asin()).to_string(),
                    denom: "1".to_string()
                })
            }
        } else if expr[inp].numer == "acos" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: ((evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).acos()).to_string(),
                    denom: "1".to_string()
                })
            }
        } else if expr[inp].numer == "atan" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: ((evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).atan()).to_string(),
                    denom: "1".to_string()
                })
            }
        } else if expr[inp].numer == "atan2" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: ((evaluate(inp - 2, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 2, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).atan2(evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap())).to_string(),
                    denom: "1".to_string()
                })
            }
        } else if expr[inp].numer == "csc" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: (1. / ((evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).sin())).to_string(),
                    denom: "1".to_string()
                })
            }
        } else if expr[inp].numer == "sec" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: (1. / ((evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).cos())).to_string(),
                    denom: "1".to_string()
                })
            }
        } else if expr[inp].numer == "cot" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: (1. / ((evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).tan())).to_string(),
                    denom: "1".to_string()
                })
            }
        } else if expr[inp].numer == "pow" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: evaluate(inp - 2, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap().powf(evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).to_string(),
                    denom: evaluate(inp - 2, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap().powf(evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()).to_string()
                })
            }
        } else if expr[inp].numer == "root" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: evaluate(inp - 2, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap().powf(evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap()).to_string(),
                    denom: evaluate(inp - 2, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap().powf(evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap()).to_string()
                })
            }
        } else if expr[inp].numer == "log" {
            *inputs_amount = 2;
            if inp <= 1 {
                *inputs_amount = 0;
                None
            } else {
                Some(Fraction {
                    numer: (evaluate(inp - 2, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 2, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap())
                            .log((evaluate(inp - 1, expr, inputs_amount, verbose)?.numer.parse::<f64>().unwrap() / evaluate(inp - 1, expr, inputs_amount, verbose)?.denom.parse::<f64>().unwrap()))
                            .to_string(),
                    denom: "1".to_string()
                })
            }
        } else if expr[inp].numer == "simplify" {
            *inputs_amount = 1;
            if inp <= 0 {
                *inputs_amount = 0;
                None
            } else {
                Some(evaluate(inp - 1, expr, inputs_amount, verbose)?.simplify())
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
