use std::io;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::env;

// Bytecode commands
// 0x00: IDK (think of something for this)
// 0x01: LIT (consume next bytes as literal)
// 0x02: SWAP (swap n-1 and n)
// 0x03: DEL (remove n)
// 0x04: COPY (copy n to n+1)
// 0x05: DEF (function definition start)
// 0x06: END (function definition end)
// 0x10: ADD (n-1 + n)
// 0x11: SUB (n-1 - n)
// 0x12: MUL (n-1 * n)
// 0x13: DIV (n-1 / n)
// 0x14: POW (n-1 ^ n)
// 0x15: SQRT (square root of n)
// 0x16: ONEMIN (1.0 - n)
// 0x17: ROUND
// 0x18: CEIL
// 0x19: FLOOR
// 0x1a: MOD (n-1 % n)
// 0x1b: FRACT (fractional part of n)
// 0x1c: COMP (1.0 / n)
// 0x1d: LERP (lerp n-2 to n-1 by n alpha)
// 0x1e: MIN
// 0x1f: MAX
  
// ./slipcode Q:/Code/slipcompiler/Test.slb

fn main() {
    println!("Slipcode | Skye Terran, 2021\n");

    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let mut instructions: Vec<u8> = get_file_as_byte_vec(file_path);
    let mut values: Vec<f32> = vec![];

    execute(&mut instructions, &mut values);
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

fn execute(instructions: &mut Vec<u8>, values: &mut Vec<f32>) {
    println!("Executing bytecode instructions...\n");
    let mut iter = instructions.iter();
    
    // Handle literals
    let mut is_literal = false;
    let mut literal = [0u8; 4];
    let mut lit_digit = 0;
    loop {
        let byte_opt = iter.next();

        if is_literal {
            if byte_opt.is_some() {
                let byte = byte_opt.unwrap();
                // Record another of the literal's bytes
                literal[lit_digit] = *byte;

                // Continue consuming the literal
                if lit_digit >= 3 {
                    let num = f32::from_bits(as_u32_be(&literal));
                    values.push(num);
                    println!("LIT {:?}", num);
                    println!("Values: {:?}\n", values);
                    is_literal = false;
                    lit_digit = 0;
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
                    0x01 => is_literal = true,
                    0x02 => swap(values),
                    0x03 => del(values),
                    0x04 => copy(values),
                    0x10 => add(values),
                    0x11 => sub(values),
                    0x12 => mul(values),
                    0x19 => floor(values),
                    _ => break
                }
                if !is_literal {
                    println!("Values: {:?}\n", values);
                }
            } else {
                break;
            }
        }
    }
    instructions.clear();
}

fn add(values: &mut Vec<f32>) {
    println!("ADD");
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

fn sub(values: &mut Vec<f32>) {
    println!("SUB");
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

fn mul(values: &mut Vec<f32>) {
    println!("MUL");
    let b_opt = values.pop();
    let a_opt = values.pop();
    if a_opt.is_some() && b_opt.is_some() {
        let a = a_opt.unwrap();
        let b = b_opt.unwrap();
        values.push(a * b);
    } else {
        println!("Not enough values.");
    }
}

fn del(values: &mut Vec<f32>) {
    println!("DEL");
    values.pop();
}

fn copy(values: &mut Vec<f32>) {
    println!("COPY");
    let a_opt = values.pop();
    if a_opt.is_some() {
        let a = a_opt.unwrap();
        values.push(a);
        values.push(a);
    } else {
        println!("Not enough values.");
    }
}

fn swap(values: &mut Vec<f32>) {
    println!("SWAP");
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

fn floor(values: &mut Vec<f32>) {
    println!("FLOOR");
    let a_opt = values.pop();
    if a_opt.is_some() {
        let a = a_opt.unwrap();
        let a = a.floor();
        values.push(a);
    }
}

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24)
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}