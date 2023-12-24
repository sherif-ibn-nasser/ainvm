use std::fs;
use std::env;

fn read_instructions_file_from_args()->Vec<String>{
    let args: Vec<String> = env::args().collect();

    if args.len()!=2{
        println!("يجب تمرير ملف واحد فقط.");
        std::process::exit(1);
    }

    let file_path: &String=&args[1];

    let contents=fs::read_to_string(file_path)
        .expect("لا يمكن قراءة الملف.");

    let instructions=contents.split_whitespace()
        .map(String::from).collect();

    return instructions;
}

fn assemble_instructions(instructions:Vec<String>):Vec<u8>{
    
}

fn main() {
    let instructions_str=read_instructions_file_from_args();

    let instructions=assemble_instructions(instructions_str);

}
