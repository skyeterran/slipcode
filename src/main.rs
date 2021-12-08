use std::io;

fn main() {
    println!("Slipcode | Skye Terran, 2021");

    let mut instructions: Vec<u8> = vec![];
    let mut values: Vec<i32> = vec![];

    loop {
        let input = get_input();
        let mut iter = input.iter();
        loop {
            let byte_opt = iter.next();
            if byte_opt.is_some() {
                let byte = byte_opt.unwrap();
                instructions.push(*byte);
            } else {
                break;
            }
        }
        execute(&mut instructions, &mut values);
    }
}

fn get_input() -> Vec<u8> {
    println!("\nEnter instruction:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Couldn't read input!");

    println!("\n");

    input.truncate(input.len() - 2);
    let bytes = input.as_bytes();

    bytes.to_vec()
}

fn execute(instructions: &mut Vec<u8>, values: &mut Vec<i32>) {
    debug_instructions(instructions);
    let mut iter = instructions.iter();
    
    // Handle literals
    let mut is_literal = false;
    let mut literal = [0u8; 8];
    let mut lit_digit = 0;
    loop {
        let byte_opt = iter.next();

        if is_literal {
            if byte_opt.is_some() {
                let byte = byte_opt.unwrap();
                // Record another of the literal's bytes
                literal[lit_digit] = *byte;

                // Continue consuming the literal
                if lit_digit >= 7 {
                    let num = f32::from_bits()
                    is_literal = false;
                } else {
                    lit_digit += 1;
                }
            } else {
                break;
            }
        } else {
            if byte_opt.is_some() {
                let byte = byte_opt.unwrap();
                match byte {
                    65 => add(values),
                    75 => kill(values),
                    76 => is_literal = true,
                    83 => sub(values),
                    88 => swap(values),
                    _ => break
                }
            } else {
                break;
            }
        }
    }
    instructions.clear();
    println!("Values: {:?}", values);
}

fn debug_instructions(instructions: &Vec<u8>) {
    let mut readable_stack: Vec<String> = Vec::new();
    let mut iter = instructions.iter();
    loop {
        let mut name = "";
        let value = iter.next();
        if value.is_some() {
            let byte = value.unwrap();
            let byte_array = [*byte];
            match byte {
                45 => name = "MINUS",
                46 => name = "POINT",
                48..=57 => name = std::str::from_utf8(&byte_array).unwrap(),
                65 => name = "ADD",
                70 => name = "LITERAL_FLOAT",
                73 => name = "LITERAL_INT",
                75 => name = "KILL",
                76 => name = "LITERAL",
                83 => name = "SUB",
                88 => name = "SWAP",
                _ => name = "NULL"
            }
            readable_stack.push(name.to_string());
        } else {
            break;
        }
    }
    println!("Bytes: {:?}", instructions);
    println!("Instructions: {:?}", readable_stack);
}

fn add(values: &mut Vec<i32>) {
    let b_opt = values.pop();
    let a_opt = values.pop();
    if a_opt.is_some() && b_opt.is_some() {
        let a = a_opt.unwrap();
        let b = b_opt.unwrap();
        values.push(a + b);
    } else {
        println!("Not enough values.");
    }
}

fn sub(values: &mut Vec<i32>) {
    let b_opt = values.pop();
    let a_opt = values.pop();
    if a_opt.is_some() && b_opt.is_some() {
        let a = a_opt.unwrap();
        let b = b_opt.unwrap();
        values.push(a - b);
    } else {
        println!("Not enough values.");
    }
}

fn kill(values: &mut Vec<i32>) {
    values.pop();
}

fn swap(values: &mut Vec<i32>) {
    let b_opt = values.pop();
    let a_opt = values.pop();
    if a_opt.is_some() && b_opt.is_some() {
        let a = a_opt.unwrap();
        let b = b_opt.unwrap();
        values.push(b);
        values.push(a);
    } else {
        println!("Not enough values.");
    }
}

// Instruction codes
// null (0) = NULL
// - (45) = MINUS
// . (46) = POINT
// 0..9 (48..57) = numbers 0..9
// F (70) = LIT_FLOAT
// I (73) = LIT_INT
// A (65) = ADD
// K (75) = KILL (removes top value from stack)
// L (76) = LITERAL
// S (83) = SUB
// X (88) = SWAP (switches the order of the last two values in the stack)