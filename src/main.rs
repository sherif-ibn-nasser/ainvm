#![recursion_limit = "256"]

pub mod assembler;
pub mod op_code;

use std::fs;
use std::env;

fn read_instructions_file_from_args()->String{
    let args: Vec<String> = env::args().collect();

    if args.len()!=2{
        println!("يجب تمرير ملف واحد فقط.");
        std::process::exit(1);
    }

    let file_path: &String=&args[1];

    return fs::read_to_string(file_path)
        .expect("لا يمكن قراءة الملف.");

}

fn main() {
    let content=read_instructions_file_from_args();
    let instructions=assembler::Assembler::new(content).assemble();
    println!("{}",instructions.len());
    for i in instructions{
        println!("{}**",i)
    }
}
