use std::io;

fn main() {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let mut x = 0; 
    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => x = i, 
        Err(..) => println!("error")
    };
    let mut baum_sweet_sequence = String::from("");
    for i in 0..x+1 {
//        println!("{}", i);
        let odd = baum_sweet(i);
//        println!("{}", odd);
        if odd {
            baum_sweet_sequence.push('0');
        } else {
            baum_sweet_sequence.push('1');
        }
    }
    println!("{}", baum_sweet_sequence);
}

fn baum_sweet(x: u32) -> bool {
    // x is 0 by default.
    let b=format!("{:b}",x);
    let mut state = 0; 
    let mut odd = true;
 //   println!("binary is: {}", b);
    for chars in b.chars() {
        match chars {
            '0' => {
                match state {
                    0 => {
                        state = 2;
                        odd = true;
                    },
                    1 => {
                        state = 2;
                        odd = true;
                    },
                    2 => {
                        state = 1;
                        odd = false;
                    },
                    3 => {
                        state = 3;
                        odd = true;
                    },
                    _ => println!("entered impossible FSM node"),
                }
            },
            '1' => {
                match state {
                    0 => {
                        state = 1;
                        odd = false;
                    },
                    1 => {
                        state = 1;
                        odd = false;
                    },
                    2 => {
                        state = 3;
                        odd = true;
                    },
                    3 => {
                        state = 3;
                        odd = true;
                    },
                    _ => println!("entered impossible FSM node"),
                }
            },
            _ => println!("error!"),
        }
    };
    odd
}
