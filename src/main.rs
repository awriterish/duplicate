use std::env;
use std::fs;

fn main() {
    let arg = env::args()
        .skip(1)
        .collect::<Vec<String>>();
    if arg.iter().count() == 2 {
        let result = fs::copy(arg[0].clone(), arg[1].clone());
        match result {
            Ok(_) => {println!("File copied")}
            Err(_) => {println!("Error copying file")}
        }
    } else {
        println!("Incorrect number of arguments passed")
    }
}