use std::fs;
use std::env;

fn read_instructions_file_from_args()->Vec<String>{
    let args: Vec<String> = env::args().collect();

    if args.len()!=2{
        println!("يجب تمرير ملف واحد فقط.");
        std::process::exit(1);
    }

    let file_path: &String=&args[1];

    let content=fs::read_to_string(file_path)
        .expect("لا يمكن قراءة الملف.");

    let lines=content.split("\n")
        .map(String::from).collect();

    return lines;
}

fn assemble_instructions(lines:Vec<String>)->Vec<u8>{
    for l in lines{
        
    }
    
}

fn main() {
    let lines=read_instructions_file_from_args();

    let instructions=assemble_instructions(lines);

}
