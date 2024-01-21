use std::{str::FromStr, fmt::{Debug, Display}, collections::HashMap};

use crate::op_code::*;

pub struct Assembler{
    content:Vec<char>,
    i:usize,
    current_instruction:String,
    instructions:Vec<u8>,
    functions:HashMap<String,u64>, // maps functions to the index of first instruction inside it
    goto_statements:HashMap<usize,String>,
}

impl Assembler{
    pub fn new(content:String)->Self{
        Assembler{
            content:content.chars().collect(),
            i: 0,
            current_instruction: String::from(""),
            instructions: vec![],
            functions: HashMap::new(),
            goto_statements: HashMap::new(),
        }
    }

    pub fn assemble(&mut self)->&mut Vec<u8>{

        while self.i<self.content.len() {

            self.parse_next_instruction();

            if !self.current_instruction.is_empty()
            {
                self.map_current_instruction();
            }

        }

        self.map_goto_statements();

        for f in &self.functions{
            println!("{}, {}",f.0,f.1);
        }
        
        return &mut self.instructions;
    }

    fn map_current_instruction(&mut self){
    
        match self.current_instruction.as_str() {
    
            op_code_name::NOP=>{
                self.instructions.push(op_code::NOP)
            }
            op_code_name::HALT=>{
                self.instructions.push(op_code::HALT)
            }
            op_code_name::PRINT=>{
                self.instructions.push(op_code::PRINT)
            }
            op_code_name::PRINTLN=>{
                self.instructions.push(op_code::PRINTLN)
            }
            op_code_name::GOTO=>{
                self.parse_next_instruction();
                self.goto_statements.insert(self.instructions.len(), self.current_instruction.to_owned()); // push the index of goto instruction to replace its function name operand later
                self.instructions.push(op_code::GOTO);
                self.instructions.append(&mut vec![0;8]); // null 8 bytes will be replaced later with the first ip index of the function
            }
            op_code_name::PUSH_I32=>{
                let mut int=self.get_next_num_lit::<i32>(i32::MIN,i32::MAX).to_be_bytes().to_vec();
                self.instructions.push(op_code::PUSH_I32);
                self.instructions.append(&mut int);
            }
            op_code_name::PUSH_U32=>{
                let mut uint=self.get_next_num_lit::<u32>(u32::MIN,u32::MAX).to_be_bytes().to_vec();
                self.instructions.push(op_code::PUSH_U32);
                self.instructions.append(&mut uint);
            }
            op_code_name::PUSH_F32=>{
                let mut float=self.get_next_num_lit::<f32>(f32::MIN,f32::MAX).to_be_bytes().to_vec();
                self.instructions.push(op_code::PUSH_F32);
                self.instructions.append(&mut float);
            }
            op_code_name::PUSH_I64=>{
                let mut int=self.get_next_num_lit::<i64>(i64::MIN,i64::MAX).to_be_bytes().to_vec();
                self.instructions.push(op_code::PUSH_I64);
                self.instructions.append(&mut int);
            }
            op_code_name::PUSH_U64=>{
                let mut uint=self.get_next_num_lit::<u64>(u64::MIN,u64::MAX).to_be_bytes().to_vec();
                self.instructions.push(op_code::PUSH_U64);
                self.instructions.append(&mut uint);
            }
            op_code_name::PUSH_F64=>{
                // This doesn't care about the floating point, so you may store a value bigger than u64 here.
                let mut float=self.get_next_num_lit::<f64>(f64::MIN,f64::MAX).to_be_bytes().to_vec();
                self.instructions.push(op_code::PUSH_F64);
                self.instructions.append(&mut float);
            }
            op_code_name::PUSH_CHAR=>{
                let mut c=self.get_next_char_lit('\'')
                    .encode_utf16()
                    .flat_map(|b16|b16.to_be_bytes())
                    .collect();
                self.instructions.push(op_code::PUSH_CHAR);
                self.instructions.append(&mut c);
            }
            op_code_name::PUSH_STR=>{
                let mut str:Vec<_>=self.get_next_char_lit('\"')
                    .encode_utf16()
                    .flat_map(|b16|b16.to_be_bytes())
                    .collect();
                let mut size=((str.len()/2) as u64).to_be_bytes().to_vec();
                self.instructions.push(op_code::PUSH_STR);
                self.instructions.append(&mut size);
                self.instructions.append(&mut str);
                /*
                 * let mut tbb:Vec<u16>=tb.chunks(2)
                        .map(
                            |b|
                            ((b[0] as u16) << 8) | b[1] as u16
                        )
                        .collect();
                 */
            }
            // Registers don't care about the floating point, so you may store a value bigger than u64 inside them.
            op_code_name::PUSH_TO_RBP=>{
                let mut reg_val=self.get_next_reg_num_lit();
                self.instructions.push(op_code::PUSH_TO_RBP);
                self.instructions.append(&mut reg_val);
            }
            op_code_name::PUSH_TO_RSP=>{
                let mut reg_val=self.get_next_reg_num_lit();
                self.instructions.push(op_code::PUSH_TO_RSP);
                self.instructions.append(&mut reg_val);
            }
            op_code_name::PUSH_TO_RAX=>{
                let mut reg_val=self.get_next_reg_num_lit();
                self.instructions.push(op_code::PUSH_TO_RAX);
                self.instructions.append(&mut reg_val);
            }
            op_code_name::PUSH_TO_RBX=>{
                let mut reg_val=self.get_next_reg_num_lit();
                self.instructions.push(op_code::PUSH_TO_RBX);
                self.instructions.append(&mut reg_val);
            }
            op_code_name::PUSH_TO_RCX=>{
                let mut reg_val=self.get_next_reg_num_lit();
                self.instructions.push(op_code::PUSH_TO_RCX);
                self.instructions.append(&mut reg_val);
            }
            op_code_name::PUSH_TO_RDX=>{
                let mut reg_val=self.get_next_reg_num_lit();
                self.instructions.push(op_code::PUSH_TO_RDX);
                self.instructions.append(&mut reg_val);
            }
            op_code_name::PUSH_TO_RDI=>{
                let mut reg_val=self.get_next_reg_num_lit();
                self.instructions.push(op_code::PUSH_TO_RDI);
                self.instructions.append(&mut reg_val);
            }
            op_code_name::PUSH_TO_RSI=>{
                let mut reg_val=self.get_next_reg_num_lit();
                self.instructions.push(op_code::PUSH_TO_RSI);
                self.instructions.append(&mut reg_val);
            }
            op_code_name::PUSH_TRUE=>{
                self.instructions.push(op_code::PUSH_TRUE)
            }
            op_code_name::PUSH_FALSE=>{
                self.instructions.push(op_code::PUSH_FALSE)
            }
            op_code_name::POP=>{
                self.instructions.push(op_code::POP)
            }
            op_code_name::POP_TO_RBP=>{
                self.instructions.push(op_code::POP_TO_RBP);
            }
            op_code_name::POP_TO_RSP=>{
                self.instructions.push(op_code::POP_TO_RSP);
            }
            op_code_name::POP_TO_RAX=>{
                self.instructions.push(op_code::POP_TO_RAX);
            }
            op_code_name::POP_TO_RBX=>{
                self.instructions.push(op_code::POP_TO_RBX);
            }
            op_code_name::POP_TO_RCX=>{
                self.instructions.push(op_code::POP_TO_RCX);
            }
            op_code_name::POP_TO_RDX=>{
                self.instructions.push(op_code::POP_TO_RDX);
            }
            op_code_name::POP_TO_RDI=>{
                self.instructions.push(op_code::POP_TO_RDI);
            }
            op_code_name::POP_TO_RSI=>{
                self.instructions.push(op_code::POP_TO_RSI);
            }
            op_code_name::DUP=>{
                self.instructions.push(op_code::DUP)
            }
            op_code_name::DUP_RBP=>{
                self.instructions.push(op_code::DUP_RBP);
            }
            op_code_name::DUP_RSP=>{
                self.instructions.push(op_code::DUP_RSP);
            }
            op_code_name::DUP_RAX=>{
                self.instructions.push(op_code::DUP_RAX);
            }
            op_code_name::DUP_RBX=>{
                self.instructions.push(op_code::DUP_RBX);
            }
            op_code_name::DUP_RCX=>{
                self.instructions.push(op_code::DUP_RCX);
            }
            op_code_name::DUP_RDX=>{
                self.instructions.push(op_code::DUP_RDX);
            }
            op_code_name::DUP_RDI=>{
                self.instructions.push(op_code::DUP_RDI);
            }
            op_code_name::DUP_RSI=>{
                self.instructions.push(op_code::DUP_RSI);
            }
            op_code_name::ADD_I32=>{
                self.instructions.push(op_code::ADD_I32)
            }
            op_code_name::ADD_U32=>{
                self.instructions.push(op_code::ADD_U32)
            }
            op_code_name::ADD_F32=>{
                self.instructions.push(op_code::ADD_F32)
            }
            op_code_name::ADD_I64=>{
                self.instructions.push(op_code::ADD_I64)
            }
            op_code_name::ADD_U64=>{
                self.instructions.push(op_code::ADD_U64)
            }
            op_code_name::ADD_F64=>{
                self.instructions.push(op_code::ADD_F64)
            }
            op_code_name::SUB_I32=>{
                self.instructions.push(op_code::SUB_I32)
            }
            op_code_name::SUB_U32=>{
                self.instructions.push(op_code::SUB_U32)
            }
            op_code_name::SUB_F32=>{
                self.instructions.push(op_code::SUB_F32)
            }
            op_code_name::SUB_I64=>{
                self.instructions.push(op_code::SUB_I64)
            }
            op_code_name::SUB_U64=>{
                self.instructions.push(op_code::SUB_U64)
            }
            op_code_name::SUB_F64=>{
                self.instructions.push(op_code::SUB_F64)
            }
            op_code_name::MUL_I32=>{
                self.instructions.push(op_code::MUL_I32)
            }
            op_code_name::MUL_U32=>{
                self.instructions.push(op_code::MUL_U32)
            }
            op_code_name::MUL_F32=>{
                self.instructions.push(op_code::MUL_F32)
            }
            op_code_name::MUL_I64=>{
                self.instructions.push(op_code::MUL_I64)
            }
            op_code_name::MUL_U64=>{
                self.instructions.push(op_code::MUL_U64)
            }
            op_code_name::MUL_F64=>{
                self.instructions.push(op_code::MUL_F64)
            }
            op_code_name::DIV_I32=>{
                self.instructions.push(op_code::DIV_I32)
            }
            op_code_name::DIV_U32=>{
                self.instructions.push(op_code::DIV_U32)
            }
            op_code_name::DIV_F32=>{
                self.instructions.push(op_code::DIV_F32)
            }
            op_code_name::DIV_I64=>{
                self.instructions.push(op_code::DIV_I64)
            }
            op_code_name::DIV_U64=>{
                self.instructions.push(op_code::DIV_U64)
            }
            op_code_name::DIV_F64=>{
                self.instructions.push(op_code::DIV_F64)
            }
            op_code_name::REM_I32=>{
                self.instructions.push(op_code::REM_I32)
            }
            op_code_name::REM_U32=>{
                self.instructions.push(op_code::REM_U32)
            }
            op_code_name::REM_F32=>{
                self.instructions.push(op_code::REM_F32)
            }
            op_code_name::REM_I64=>{
                self.instructions.push(op_code::REM_I64)
            }
            op_code_name::REM_U64=>{
                self.instructions.push(op_code::REM_U64)
            }
            op_code_name::REM_F64=>{
                self.instructions.push(op_code::REM_F64)
            }
            op_code_name::SHR_I32=>{
                self.instructions.push(op_code::SHR_I32)
            }
            op_code_name::SHR_U32=>{
                self.instructions.push(op_code::SHR_U32)
            }
            op_code_name::SHR_I64=>{
                self.instructions.push(op_code::SHR_I64)
            }
            op_code_name::SHR_U64=>{
                self.instructions.push(op_code::SHR_U64)
            }
            op_code_name::SHL_I32=>{
                self.instructions.push(op_code::SHL_I32)
            }
            op_code_name::SHL_U32=>{
                self.instructions.push(op_code::SHL_U32)
            }
            op_code_name::SHL_I64=>{
                self.instructions.push(op_code::SHL_I64)
            }
            op_code_name::SHL_U64=>{
                self.instructions.push(op_code::SHL_U64)
            }
            op_code_name::XOR_I32=>{
                self.instructions.push(op_code::XOR_I32)
            }
            op_code_name::XOR_U32=>{
                self.instructions.push(op_code::XOR_U32)
            }
            op_code_name::XOR_I64=>{
                self.instructions.push(op_code::XOR_I64)
            }
            op_code_name::XOR_U64=>{
                self.instructions.push(op_code::XOR_U64)
            }
            op_code_name::AND_I32=>{
                self.instructions.push(op_code::AND_I32)
            }
            op_code_name::AND_U32=>{
                self.instructions.push(op_code::AND_U32)
            }
            op_code_name::AND_I64=>{
                self.instructions.push(op_code::AND_I64)
            }
            op_code_name::AND_U64=>{
                self.instructions.push(op_code::AND_U64)
            }
            op_code_name::OR_I32=>{
                self.instructions.push(op_code::OR_I32)
            }
            op_code_name::OR_U32=>{
                self.instructions.push(op_code::OR_U32)
            }
            op_code_name::OR_I64=>{
                self.instructions.push(op_code::OR_I64)
            }
            op_code_name::OR_U64=>{
                self.instructions.push(op_code::OR_U64)
            }
            op_code_name::NOT_I32=>{
                self.instructions.push(op_code::NOT_I32)
            }
            op_code_name::NOT_U32=>{
                self.instructions.push(op_code::NOT_U32)
            }
            op_code_name::NOT_I64=>{
                self.instructions.push(op_code::NOT_I64)
            }
            op_code_name::NOT_U64=>{
                self.instructions.push(op_code::NOT_U64)
            }
            op_code_name::NEG_I32=>{
                self.instructions.push(op_code::NEG_I32)
            }
            op_code_name::NEG_F32=>{
                self.instructions.push(op_code::NEG_F32)
            }
            op_code_name::NEG_I64=>{
                self.instructions.push(op_code::NEG_I64)
            }
            op_code_name::NEG_F64=>{
                self.instructions.push(op_code::NEG_F64)
            }
            op_code_name::I32_TO_U32=>{
                self.instructions.push(op_code::I32_TO_U32)
            }
            op_code_name::I32_TO_F32=>{
                self.instructions.push(op_code::I32_TO_F32)
            }
            op_code_name::I32_TO_I64=>{
                self.instructions.push(op_code::I32_TO_I64)
            }
            op_code_name::I32_TO_U64=>{
                self.instructions.push(op_code::I32_TO_U64)
            }
            op_code_name::I32_TO_F64=>{
                self.instructions.push(op_code::I32_TO_F64)
            }
            op_code_name::U32_TO_I32=>{
                self.instructions.push(op_code::U32_TO_I32)
            }
            op_code_name::U32_TO_F32=>{
                self.instructions.push(op_code::U32_TO_F32)
            }
            op_code_name::U32_TO_I64=>{
                self.instructions.push(op_code::U32_TO_I64)
            }
            op_code_name::U32_TO_U64=>{
                self.instructions.push(op_code::U32_TO_U64)
            }
            op_code_name::U32_TO_F64=>{
                self.instructions.push(op_code::U32_TO_F64)
            }
            op_code_name::F32_TO_I32=>{
                self.instructions.push(op_code::F32_TO_I32)
            }
            op_code_name::F32_TO_U32=>{
                self.instructions.push(op_code::F32_TO_U32)
            }
            op_code_name::F32_TO_I64=>{
                self.instructions.push(op_code::F32_TO_I64)
            }
            op_code_name::F32_TO_U64=>{
                self.instructions.push(op_code::F32_TO_U64)
            }
            op_code_name::F32_TO_F64=>{
                self.instructions.push(op_code::F32_TO_F64)
            }
            op_code_name::I64_TO_I32=>{
                self.instructions.push(op_code::I64_TO_I32)
            }
            op_code_name::I64_TO_U32=>{
                self.instructions.push(op_code::I64_TO_U32)
            }
            op_code_name::I64_TO_F32=>{
                self.instructions.push(op_code::I64_TO_F32)
            }
            op_code_name::I64_TO_U64=>{
                self.instructions.push(op_code::I64_TO_U64)
            }
            op_code_name::I64_TO_F64=>{
                self.instructions.push(op_code::I64_TO_F64)
            }
            op_code_name::U64_TO_I32=>{
                self.instructions.push(op_code::U64_TO_I32)
            }
            op_code_name::U64_TO_U32=>{
                self.instructions.push(op_code::U64_TO_U32)
            }
            op_code_name::U64_TO_F32=>{
                self.instructions.push(op_code::U64_TO_F32)
            }
            op_code_name::U64_TO_I64=>{
                self.instructions.push(op_code::U64_TO_I64)
            }
            op_code_name::U64_TO_F64=>{
                self.instructions.push(op_code::U64_TO_F64)
            }
            op_code_name::F64_TO_I32=>{
                self.instructions.push(op_code::F64_TO_I32)
            }
            op_code_name::F64_TO_U32=>{
                self.instructions.push(op_code::F64_TO_U32)
            }
            op_code_name::F64_TO_F32=>{
                self.instructions.push(op_code::F64_TO_F32)
            }
            op_code_name::F64_TO_I64=>{
                self.instructions.push(op_code::F64_TO_I64)
            }
            op_code_name::F64_TO_U64=>{
                self.instructions.push(op_code::F64_TO_U64)
            }
            _=>{
                if !self.current_instruction.ends_with(":") {
                    panic!("أمر غير متوقع \"{}\"",self.current_instruction)   
                }
                self.current_instruction.remove(self.current_instruction.len()-1);
                self.functions.insert(self.current_instruction.to_owned(), self.instructions.len() as u64);
            }
        }
    
    }

    fn parse_next_instruction(&mut self){
        self.current_instruction.clear();
        while self.i<self.content.len(){
            let c=self.current_char();
            self.next();
            if c.is_ascii_whitespace(){
                if self.current_instruction.is_empty(){
                    continue
                }
                break
            }
            self.current_instruction+=&c.to_string();
        }
    }

    fn get_next_reg_num_lit<>(&mut self)->Vec<u8>{
        
        let mut num_lit=String::from("");

        while self.i<self.content.len(){
            let c=self.current_char();
            self.next();

            if !c.is_ascii_whitespace(){
                num_lit+=&c.to_string();
                continue
            }
            else if num_lit.is_empty(){
                panic!("يُتوقع عدد.")
            }
            break

        }

        let num_i64=num_lit.parse::<i64>();

        match num_i64 {
            Ok(val) => return val.to_be_bytes().to_vec(),
            _ =>{}
        }
        let num_u64=num_lit.parse::<u64>();

        match num_u64 {
            Ok(val) => return val.to_be_bytes().to_vec(),
            _ =>{}
        }
        let num_f64=num_lit.parse::<f64>();

        match num_f64 {
            Ok(val) => return val.to_be_bytes().to_vec(),
            Err(_) =>
                panic!("القيمة \'{}\' قد تعدت القيمة المسموح بها في المسجلات ذات 64 بت.",num_lit)
        }

    }

    fn get_next_num_lit<NUM:FromStr+PartialOrd+Display>(&mut self,min:NUM,max:NUM)->NUM
        where NUM::Err:Debug{
        
        let mut num_lit=String::from("");

        while self.i<self.content.len(){
            let c=self.current_char();
            self.next();

            if !c.is_ascii_whitespace(){
                num_lit+=&c.to_string();
                continue
            }
            else if num_lit.is_empty(){
                panic!("يُتوقع عدد.")
            }
            break

        }

        let num=num_lit.parse::<NUM>();

        match num {
            Ok(val) => return val,
            Err(_) =>
                panic!("القيمة \'{}\' قد تعدت القيمة المسموح بها من \'{}\' إلى \'{}\'.",num_lit,min,max)
        }
    }

    fn get_next_char_lit(&mut self,quote:char)->String{

        let mut char_lit=String::from("");

        let quote_ch=self.current_char();
        self.next();

        let is_char=quote_ch=='\'';
        let is_str=quote_ch=='\"';

        if !is_char && quote=='\''{
            panic!("يُتوقع حرف.")
        }
        if !is_str && quote=='\"'{
            panic!("يُتوقع نص.")
        }

        while self.i<self.content.len(){
            let current_char=self.current_char();
            self.next();

            /*If it's quote (" or ') return the token*/
            if current_char==quote{
                if is_char&&char_lit.chars().count()!=1{
                    panic!("\'{}\' يجب أن يكون حرفاً واحداً فقط.",char_lit)
                }
                return char_lit;
            }

            /*Append if it is not a special character*/
            if current_char!='\\'{
                Assembler::check_is_kufr_or_unsupported_character(current_char);
                char_lit+=&current_char.to_string();
                continue;
            }

            let ctrl_char=self.current_char();
            self.next();

            /*Unicode characters parsing*/
            if ctrl_char=='ي'{
                let c=self.get_next_unicode_char_from_code_point();
                Assembler::check_is_kufr_or_unsupported_character(c);
                char_lit+=&c.to_string();
                continue;
            }

            let es=Assembler::get_escape_sequence_form_char(ctrl_char);
            char_lit+=&es.to_string();
        }


        let ch=char_lit.parse::<char>();

        match ch {
            Ok(_) => return char_lit,
            Err(_) =>
                panic!("القيمة \'{}\' قد تعدت القيمة المسموح بها.",char_lit)
        }
    }

    fn current_char(&self)->char{
        self.content[self.i]
    }

    fn next(&mut self){
        self.i+=1
    }

    fn get_next_unicode_char_from_code_point(&mut self)->char{
        let mut code_point=String::from("");
        for _ in 0..4{
            let c=self.current_char();
            self.next();
            if !c.is_ascii_hexdigit(){
                panic!("يُتوقع رقم سداسي عشر.")
            }
            code_point+=&c.to_string();
        }
        let c=char::from_u32(
            u32::from_str_radix(&code_point,16).unwrap()
        ).unwrap();
        return c;
    }

    fn get_escape_sequence_form_char(c:char)->char{
        match c {
            'خ'=>return '\x08', // مسافة للخلف
            'ف'=>return '\t', // مسافة أفقية
            'س'=>return '\n', // سطر جديد
            'ر'=>return '\x0b', // مسافة رأسية
            'ص'=>return '\x0c', // الصفحة التالية
            'ج'=>return '\r', // إرجاع المؤشر إلى بداية السطر، وبدء الكتابة منه
            '\\'=>return '\\',
            '\''=>return '\'',
            '\"'=>return '\"',
            _=>{
                panic!(" \'\\{}\' حرف خاص غير صالح.",c)
            }
        }
    }

    fn check_is_kufr_or_unsupported_character(c:char){
        if Assembler::is_kufr_or_unsupported_character(c){
            panic!("يحتوي على رمز للكُفار أو رمز غير مدعوم.")
        }
    }

    fn is_kufr_or_unsupported_character(c:char)->bool{
        let chars=[
            '\u{03EE}','\u{03EF}','\u{058d}','\u{058e}',
            '\u{05EF}', // yod triangle
            '\u{07D9}','\u{093B}','\u{13D0}','\u{16BE}','\u{165C}','\u{16ED}',
            '\u{17D2}','\u{1D7B}','\u{2020}','\u{2021}','\u{256A}','\u{256B}',
            '\u{256C}','\u{2616}','\u{2617}','\u{269C}','\u{269E}','\u{269F}',
            '\u{26AF}','\u{26B0}','\u{26B1}','\u{26F3}','\u{26F9}','\u{26FB}',
            '\u{26FF}','\u{27CA}','\u{29FE}','\u{2CFE}',
        ];

        if chars.contains(&c){
            return true
        }

        let ranges=[
            /*  from  ,    to  */
            ('\u{0900}','\u{109F}'),//HinduEurope
            ('\u{1100}','\u{1C7F}'),//HinduEurope
            ('\u{253C}','\u{254B}'),
            ('\u{2624}','\u{2638}'),//Kufr
            ('\u{263D}','\u{2653}'),//Kufr
            ('\u{2654}','\u{2667}'),
            ('\u{2669}','\u{2671}'),//Music and kufr crosses
            ('\u{2680}','\u{268F}'),
            ('\u{2680}','\u{268F}'),
            ('\u{26A2}','\u{26A9}'),// Pride
            ('\u{26B3}','\u{26BC}'),// Kufr
            ('\u{26BF}','\u{26EC}'),
            ('\u{2719}','\u{2725}'),// Kufr crosses
            ('\u{2BF0}','\u{2C5F}'),// Includes astrology
            ('\u{2D80}','\u{AB2F}'),
            ('\u{AB70}','\u{FAFF}'),
        ];

        for (r1,r2) in ranges{
            if c>=r1 && c<=r2{
                return true
            }
        }

        return false
    }

    fn map_goto_statements(&mut self){
        for (goto_ip,func_name) in &self.goto_statements{
            let found_func=self.functions.get(func_name);
            match found_func {
                Some(func_ip) => {
                    let func_ip_bytes=func_ip.to_be_bytes();
                    let mut i=*goto_ip+1;
                    for byte in func_ip_bytes{
                        self.instructions[i]=byte;
                        i+=1;
                    }
                },
                None => {
                    panic!("لم يتم العثور على دالة {}.",func_name)
                },
            }
        }
    }

}