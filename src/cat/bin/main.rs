use std::io::{BufReader, Read, self};
use std::{env, process};
use std::fs::File;

fn usage() {
    println!("Enter file name");
}

fn read_file(file_name: &String) -> File {
    let file_result = File::open(file_name);
    let file = match file_result {
        Ok(f) => f,
        Err(_) => {
            println!("File {} not found", file_name);
            process::exit(1);
        },
    };
    file
}

fn buf_read(file: File) -> Result<String, io::Error> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
   let args: Vec<String> = env::args().collect(); 

    if args.len() == 1 {
        usage();
        process::exit(1);
    }

    let file_name = &args[1];
    let file = read_file(&file_name);

    let contents = buf_read(file).unwrap();
    println!("{contents}");
}
