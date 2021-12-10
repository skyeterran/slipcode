use std::io;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::env;
use simple_logger::SimpleLogger;

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

// Contextual: literal types
// 0x00: BOOL (boolean) # Example: LIT BOOL 0
// 0x01: INT (int) # Example: LIT INT 69
// 0x02: FLT (float) # Example: LIT FLT -90.0
// 0x03: VEC (vector) # Example: LIT VEC 1.0,-7.01,2.7

// ./slipcode Q:/Code/slipcompiler/Test.vcr

// Opcode kind enum
// TODO: put this in a "VCROperation" struct which implements a .execute() trait or smth
enum VCROperationKind {
    Generic,
    Type,
    Value
}

fn main() {
    // Log levels are [error, warn, info, debug, trace] in descending order of priority
    SimpleLogger::new().init().unwrap();
    
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let mut instructions: Vec<u8> = get_file_as_byte_vec(file_path);
    let mut values: Vec<f32> = vec![];

    execute(&mut instructions, &mut values);
}

fn execute(instructions: &mut Vec<u8>, values: &mut Vec<f32>) {
    log::info!("Executing bytecode instructions...\n");
    let mut iter = instructions.iter();
    
    // Handle literals
    let mut op_type = VCROperationKind::Generic;
    let mut literal = [0u8; 4];
    let mut lit_digit = 0;

    // Handle each opcode in order
    loop {
        let byte_opt = iter.next();

        // Contextually handle opcodes depending on their type
        match op_type {
            // Treat the opcode as a literal type declaration
            VCROperationKind::Type => {
                // Consume the literal type
                let byte = byte_opt.unwrap();

                // The following opcodes are expected to be a literal value
                op_type = VCROperationKind::Value;
            },
            // Treat the opcode as a literal value
            VCROperationKind::Value => {
                if byte_opt.is_some() {
                    let byte = byte_opt.unwrap();
                    // Record another of the literal's bytes
                    literal[lit_digit] = *byte;
    
                    // Continue consuming the literal
                    if lit_digit >= 3 {
                        let num = f32::from_bits(as_u32_be(&literal));
                        values.push(num);
                        log::debug!("LIT {:?}", num);
                        log::debug!("Values: {:?}\n", values);
                        op_type = VCROperationKind::Generic;
                        lit_digit = 0;
                    } else {
                        lit_digit += 1;
                    }
                } else {
                    break;
                }
            },
            // Treat the opcode as a generic command
            _ => {
                if byte_opt.is_some() {
                    let byte = byte_opt.unwrap();
                    match byte {
                        0x01 => op_type = VCROperationKind::Type,
                        0x02 => swap(values),
                        0x03 => del(values),
                        0x04 => copy(values),
                        0x10 => add(values),
                        0x11 => sub(values),
                        0x12 => mul(values),
                        0x19 => floor(values),
                        _ => break
                    }
                    // DEBUG - show the value stack upon every generic command
                    if let VCROperationKind::Generic = op_type {
                        log::debug!("Values: {:?}\n", values);
                    }
                } else {
                    break;
                }
            }
        }
    }
    instructions.clear();
}

fn add(values: &mut Vec<f32>) {
    log::debug!("ADD");
    let b_opt = values.pop();
    let a_opt = values.pop();
    if a_opt.is_some() && b_opt.is_some() {
        let a = a_opt.unwrap();
        let b = b_opt.unwrap();
        values.push(a + b);
    } else {
        log::error!("Not enough values.");
    }
}

fn sub(values: &mut Vec<f32>) {
    log::debug!("SUB");
    let b_opt = values.pop();
    let a_opt = values.pop();
    if a_opt.is_some() && b_opt.is_some() {
        let a = a_opt.unwrap();
        let b = b_opt.unwrap();
        values.push(a - b);
    } else {
        log::error!("Not enough values.");
    }
}

fn mul(values: &mut Vec<f32>) {
    log::debug!("MUL");
    let b_opt = values.pop();
    let a_opt = values.pop();
    if a_opt.is_some() && b_opt.is_some() {
        let a = a_opt.unwrap();
        let b = b_opt.unwrap();
        values.push(a * b);
    } else {
        log::error!("Not enough values.");
    }
}

fn del(values: &mut Vec<f32>) {
    log::debug!("DEL");
    values.pop();
}

fn copy(values: &mut Vec<f32>) {
    log::debug!("COPY");
    let a_opt = values.pop();
    if a_opt.is_some() {
        let a = a_opt.unwrap();
        values.push(a);
        values.push(a);
    } else {
        log::error!("Not enough values.");
    }
}

fn swap(values: &mut Vec<f32>) {
    log::debug!("SWAP");
    let b_opt = values.pop();
    let a_opt = values.pop();
    if a_opt.is_some() && b_opt.is_some() {
        let a = a_opt.unwrap();
        let b = b_opt.unwrap();
        values.push(b);
        values.push(a);
    } else {
        log::error!("Not enough values.");
    }
}

fn floor(values: &mut Vec<f32>) {
    log::debug!("FLOOR");
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