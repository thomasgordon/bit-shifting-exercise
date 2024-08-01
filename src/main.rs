use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: rotate <left|right> <input file> <output file>");
        process::exit(1);
    }

    let direction = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];

    let mut data = match fs::read(input_file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error reading input file: {}", err);
            process::exit(1);
        }
    };

    if let Err(err) = rotate(&mut data, direction) {
        eprintln!("Error during rotation: {}", err);
        process::exit(1);
    }

    let mut output_file = match File::create(output_file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating output file: {}", err);
            process::exit(1);
        }
    };

    if let Err(err) = output_file.write_all(&data) {
        eprintln!("Error writing to output file: {}", err);
        process::exit(1);
    }

    println!("File rotation completed successfully.");
}

fn rotate(data: &mut [u8], direction: &str) -> Result<(), &'static str> {
    if data.is_empty() {
        return Ok(());
    }

    match direction {
        "left" => {
            let first_bit = (data[0] & 0b1000_0000) >> 7;

            for i in 0..data.len() - 1 {
                data[i] = (data[i] << 1) | ((data[i + 1] & 0b1000_0000) >> 7);
            }

            data[data.len() - 1] = (data[data.len() - 1] << 1) | first_bit;
        }

        "right" => {
            let last_bit = data[data.len() - 1] & 0b0000_0001;

            for i in (1..data.len()).rev() {
                data[i] = (data[i] >> 1) | ((data[i - 1] & 0b0000_0001) << 7);
            }

            data[0] = (data[0] >> 1) | (last_bit << 7);
        }

        _ => return Err("Invalid direction. Use 'left' or 'right'."),
    }

    Ok(())
}
