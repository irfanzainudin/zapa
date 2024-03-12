use clap::Parser;
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Fail {
    /// Name of Zapa file
    #[arg(short, long)]
    zapa_file: String,
}

fn read_lines(filename: &Path) -> Vec<String> {
    let result = read_to_string(filename);
    match result {
        Ok(res) => {
            return res.lines().map(String::from).collect();
        }
        Err(_) => {
            return Vec::new();
        }
    }
}

fn main() {
    let args = Fail::parse();
    let filename = Path::new(&args.zapa_file);

    match filename.extension() {
        Some(ext) => {
            if ext.to_str().unwrap() == "zapa" {
                println!("Fail {:?} berjaya diterima", filename);
                let vec_str = read_lines(filename);
                println!("Fail {:?} berjaya dibaca", filename);
                println!("{:?}", vec_str);
                // println!("Sedang dikompil...");
            } else {
                println!(
                    "Fail {:?} tidak menggunakan format yang betul: .zapa",
                    filename
                );
            }
        }
        None => {
            eprintln!("Fail {:?} tidak berjaya diterima", filename);
            exit(1);
        }
    }
}
