use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;

fn main() {
    // get command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: rotate <left|right> <input file> <output file>");
        process::exit(1);
    }

    let direction = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];

    // read input file into vector of bytes
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

    //create output file, write data to it
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

// &mut [u8] is a mutable reference to slice of bytes
fn rotate(data: &mut [u8], direction: &str) -> Result<(), &'static str> {
    if data.is_empty() {
        return Ok(());
    }

    match direction {
        "left" => {
            // get first bit of first byte (the and operation isolates the first bit)
            let first_bit = (data[0] & 0b1000_0000) >> 7;
            // shift each byte to the left by 1 bit (data[i] << 1 shifts each byte to the left by 1 bit)
            // ((data[i + 1] & 0b1000_0000) >> 7) takes the the MSB of the byte and places it in the LSB of the byte using the or operation
            for i in 0..data.len() - 1 {
                data[i] = (data[i] << 1) | ((data[i + 1] & 0b1000_0000) >> 7);
            }
            // set last bit of last byte to first bit of first byte
            data[data.len() - 1] = (data[data.len() - 1] << 1) | first_bit;
        }

        "right" => {
            // get last bit of last byte
            let last_bit = data[data.len() - 1] & 0b0000_0001;
            // shift each byte to the right by 1 bit
            // This loop iterates over all bytes in the data slice except the first one, but in reverse order
            // ((data[i - 1] & 0b0000_0001) << 7) takes the LSB of the byte and places it in the MSB of the byte using the or operation
            for i in (1..data.len()).rev() {
                data[i] = (data[i] >> 1) | ((data[i - 1] & 0b0000_0001) << 7);
            }
            // set first bit of first byte to last bit of last byte
            data[0] = (data[0] >> 1) | (last_bit << 7);
        }

        _ => return Err("Invalid direction. Use 'left' or 'right'."),
    }

    Ok(())
}
