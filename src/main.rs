use clap::Parser;
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;
mod token;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct FailZapa {
    /// Name of Zapa file
    #[arg(short, long)]
    zapa_file: String,
}

fn read_lines(filename: &Path) -> Vec<String> {
    let result = read_to_string(filename);
    match result {
        Ok(res) => {
            return res.lines().map(str::trim).map(String::from).collect();
        }
        Err(_) => {
            return Vec::new();
        }
    }
}

fn main() {
    let args: FailZapa = FailZapa::parse();
    let filename: &Path = Path::new(&args.zapa_file);

    // let tok = token::Token {
    //     token_type: token::TokenType::LeftParen,
    //     lexeme: String::from("("),
    //     literal: String::from("lp"),
    //     line: 0,
    // };

    // println!("{}", tok);

    match filename.extension() {
        Some(ext) => {
            if ext.to_str().unwrap() == "zapa" {
                println!("Fail Zapa {:?} berjaya diterima", filename);
                let vec_str: Vec<String> = read_lines(filename);
                for vs in &vec_str {
                    let lines: Vec<String> = vs.as_str().split(' ').map(String::from).collect();
                    println!("{:?}", lines);
                }
                println!("Fail Zapa {:?} berjaya dibaca", filename);
                println!("{:?}", vec_str);
                println!("Sedang dikompil...");
            } else {
                println!(
                    "Fail Zapa {:?} tidak menggunakan format yang betul: <NAMA_FAIL>.zapa",
                    filename
                );
            }
        }
        None => {
            eprintln!("Fail Zapa {:?} tidak berjaya diterima", filename);
            exit(1);
        }
    }
}
