use std::env;
use std::fs;
use std::error::Error;
use std::process;

struct Files {
    pub file1: String,
    pub file2: String,
}

impl Files {
    fn get(args: &[String]) -> Result<Files, &'static str> {
        let file1;
        let file2;
        if args.len() == 2 {
            file1 = args[1].clone();
            file2 = args[1].clone();
        } else if args.len() == 3 {
            file1 = args[1].clone();
            file2 = args[2].clone();
        } else {
            return Err("Usage: file1 [file2]")
        }
        Ok(Files { file1, file2 })
    }
}

pub fn read_file(file: String) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;
    let mut content_v: Vec<String> = [].to_vec();
    for line in content.lines() {
        content_v.push(line.to_owned());
    };
    Ok(content_v)
}

fn append_lines(lis: Vec<String>, mut des: Vec<String>) -> Vec<String> {
    for line in lis {
        if !des.contains(&line) {
            des.push(line);
        }
    }
    des
}

fn get_out(lis: Vec<String>) -> String {
    let mut output = String::new();
    for line in lis {
        output.push_str(&line);
        output.push_str("\n");
    }
    output.truncate(output.len() - 1);
    output
}

fn run() {
    let args: Vec<String> = env::args().collect();
    let files = Files::get(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });
    let c_file1 = read_file(files.file1).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(2);
    });
    let c_file2 = read_file(files.file2).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(2);
    });
    let sum_lines: Vec<String> = [].to_vec();
    let d1 = append_lines(c_file1, sum_lines);
    let d2 = append_lines(c_file2, d1);
    println!("{}", get_out(d2));
}

fn main() {
    run();
}