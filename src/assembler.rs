use std::{str::FromStr, fmt::{Debug, Display}};

use crate::op_code::*;

pub struct Assembler{
    content:Vec<char>,
    i:usize,
    current_instruction:String,
}

impl Assembler{
    pub fn new(content:String)->Self{
        Assembler{
            content:content.chars().collect(),
            i: 0,
            current_instruction: String::from("")
        }
    }

    pub fn assemble(&mut self)->Vec<u8>{
        let mut instructions:Vec<u8>=vec![];

        while self.i<self.content.len() {

            let c=self.content[self.i];
            self.i+=1;

            if !c.is_ascii_whitespace(){
                self.current_instruction+=&c.to_string();
            }

            if
                (
                    c.is_ascii_whitespace()
                    ||
                    self.i==self.content.len()
                )
                &&
                !self.current_instruction.is_empty()
            {
                let mut new_instructions=self.map_current_instruction();
                instructions.append(&mut new_instructions);
                self.current_instruction=String::from("");
            }


        }
        
        return instructions;
    }

    fn map_current_instruction(&mut self)->Vec<u8>{
        let mut instructions:Vec<u8>=vec![];
    
        match self.current_instruction.as_str() {
    
            op_code_name::NOP=>{
                instructions.push(op_code::NOP)
            }
            op_code_name::PRINT=>{
                instructions.push(op_code::PRINT)
            }
            op_code_name::PRINTLN=>{
                instructions.push(op_code::PRINTLN)
            }
            op_code_name::PUSH_I32=>{
                let mut int=self.get_next_num_lit::<i32>(i32::MIN,i32::MAX).to_be_bytes().to_vec();
                instructions.push(op_code::PUSH_I32);
                instructions.append(&mut int);
            }
            op_code_name::PUSH_U32=>{
                let mut uint=self.get_next_num_lit::<u32>(u32::MIN,u32::MAX).to_be_bytes().to_vec();
                instructions.push(op_code::PUSH_U32);
                instructions.append(&mut uint);
            }
            op_code_name::PUSH_F32=>{
                let mut float=self.get_next_num_lit::<f32>(f32::MIN,f32::MAX).to_be_bytes().to_vec();
                instructions.push(op_code::PUSH_F32);
                instructions.append(&mut float);
            }
            op_code_name::PUSH_I64=>{
                let mut int=self.get_next_num_lit::<i64>(i64::MIN,i64::MAX).to_be_bytes().to_vec();
                instructions.push(op_code::PUSH_I64);
                instructions.append(&mut int);
            }
            op_code_name::PUSH_U64=>{
                let mut uint=self.get_next_num_lit::<u64>(u64::MIN,u64::MAX).to_be_bytes().to_vec();
                instructions.push(op_code::PUSH_U64);
                instructions.append(&mut uint);
            }
            op_code_name::PUSH_F64=>{
                let mut float=self.get_next_num_lit::<f64>(f64::MIN,f64::MAX).to_be_bytes().to_vec();
                instructions.push(op_code::PUSH_F64);
                instructions.append(&mut float);
            }
            op_code_name::PUSH_CHAR=>{
                let mut c=self.get_next_char_lit('\'').as_bytes().to_vec();
                instructions.push(op_code::PUSH_CHAR);
                instructions.append(&mut c);
            }
            op_code_name::PUSH_STR=>{
                let mut str=self.get_next_char_lit('\"').as_bytes().to_vec();
                instructions.push(op_code::PUSH_STR);
                instructions.append(&mut str);
            }
            op_code_name::PUSH_TRUE=>{
                instructions.push(op_code::PUSH_TRUE)
            }
            op_code_name::PUSH_FALSE=>{
                instructions.push(op_code::PUSH_FALSE)
            }
            op_code_name::POP=>{
                instructions.push(op_code::POP)
            }
            op_code_name::DUP=>{
                instructions.push(op_code::DUP)
            }
            op_code_name::ADD_I32=>{
                instructions.push(op_code::ADD_I32)
            }
            op_code_name::ADD_U32=>{
                instructions.push(op_code::ADD_U32)
            }
            op_code_name::ADD_F32=>{
                instructions.push(op_code::ADD_F32)
            }
            op_code_name::ADD_I64=>{
                instructions.push(op_code::ADD_I64)
            }
            op_code_name::ADD_U64=>{
                instructions.push(op_code::ADD_U64)
            }
            op_code_name::ADD_F64=>{
                instructions.push(op_code::ADD_F64)
            }
            op_code_name::SUB_I32=>{
                instructions.push(op_code::SUB_I32)
            }
            op_code_name::SUB_U32=>{
                instructions.push(op_code::SUB_U32)
            }
            op_code_name::SUB_F32=>{
                instructions.push(op_code::SUB_F32)
            }
            op_code_name::SUB_I64=>{
                instructions.push(op_code::SUB_I64)
            }
            op_code_name::SUB_U64=>{
                instructions.push(op_code::SUB_U64)
            }
            op_code_name::SUB_F64=>{
                instructions.push(op_code::SUB_F64)
            }
            op_code_name::MUL_I32=>{
                instructions.push(op_code::MUL_I32)
            }
            op_code_name::MUL_U32=>{
                instructions.push(op_code::MUL_U32)
            }
            op_code_name::MUL_F32=>{
                instructions.push(op_code::MUL_F32)
            }
            op_code_name::MUL_I64=>{
                instructions.push(op_code::MUL_I64)
            }
            op_code_name::MUL_U64=>{
                instructions.push(op_code::MUL_U64)
            }
            op_code_name::MUL_F64=>{
                instructions.push(op_code::MUL_F64)
            }
            op_code_name::DIV_I32=>{
                instructions.push(op_code::DIV_I32)
            }
            op_code_name::DIV_U32=>{
                instructions.push(op_code::DIV_U32)
            }
            op_code_name::DIV_F32=>{
                instructions.push(op_code::DIV_F32)
            }
            op_code_name::DIV_I64=>{
                instructions.push(op_code::DIV_I64)
            }
            op_code_name::DIV_U64=>{
                instructions.push(op_code::DIV_U64)
            }
            op_code_name::DIV_F64=>{
                instructions.push(op_code::DIV_F64)
            }
            op_code_name::REM_I32=>{
                instructions.push(op_code::REM_I32)
            }
            op_code_name::REM_U32=>{
                instructions.push(op_code::REM_U32)
            }
            op_code_name::REM_F32=>{
                instructions.push(op_code::REM_F32)
            }
            op_code_name::REM_I64=>{
                instructions.push(op_code::REM_I64)
            }
            op_code_name::REM_U64=>{
                instructions.push(op_code::REM_U64)
            }
            op_code_name::REM_F64=>{
                instructions.push(op_code::REM_F64)
            }
            op_code_name::SHR_I32=>{
                instructions.push(op_code::SHR_I32)
            }
            op_code_name::SHR_U32=>{
                instructions.push(op_code::SHR_U32)
            }
            op_code_name::SHR_I64=>{
                instructions.push(op_code::SHR_I64)
            }
            op_code_name::SHR_U64=>{
                instructions.push(op_code::SHR_U64)
            }
            op_code_name::SHL_I32=>{
                instructions.push(op_code::SHL_I32)
            }
            op_code_name::SHL_U32=>{
                instructions.push(op_code::SHL_U32)
            }
            op_code_name::SHL_I64=>{
                instructions.push(op_code::SHL_I64)
            }
            op_code_name::SHL_U64=>{
                instructions.push(op_code::SHL_U64)
            }
            op_code_name::XOR_I32=>{
                instructions.push(op_code::XOR_I32)
            }
            op_code_name::XOR_U32=>{
                instructions.push(op_code::XOR_U32)
            }
            op_code_name::XOR_I64=>{
                instructions.push(op_code::XOR_I64)
            }
            op_code_name::XOR_U64=>{
                instructions.push(op_code::XOR_U64)
            }
            op_code_name::AND_I32=>{
                instructions.push(op_code::AND_I32)
            }
            op_code_name::AND_U32=>{
                instructions.push(op_code::AND_U32)
            }
            op_code_name::AND_I64=>{
                instructions.push(op_code::AND_I64)
            }
            op_code_name::AND_U64=>{
                instructions.push(op_code::AND_U64)
            }
            op_code_name::OR_I32=>{
                instructions.push(op_code::OR_I32)
            }
            op_code_name::OR_U32=>{
                instructions.push(op_code::OR_U32)
            }
            op_code_name::OR_I64=>{
                instructions.push(op_code::OR_I64)
            }
            op_code_name::OR_U64=>{
                instructions.push(op_code::OR_U64)
            }
            op_code_name::NOT_I32=>{
                instructions.push(op_code::NOT_I32)
            }
            op_code_name::NOT_U32=>{
                instructions.push(op_code::NOT_U32)
            }
            op_code_name::NOT_I64=>{
                instructions.push(op_code::NOT_I64)
            }
            op_code_name::NOT_U64=>{
                instructions.push(op_code::NOT_U64)
            }
            op_code_name::NEG_I32=>{
                instructions.push(op_code::NEG_I32)
            }
            op_code_name::NEG_F32=>{
                instructions.push(op_code::NEG_F32)
            }
            op_code_name::NEG_I64=>{
                instructions.push(op_code::NEG_I64)
            }
            op_code_name::NEG_F64=>{
                instructions.push(op_code::NEG_F64)
            }
            op_code_name::I32_TO_U32=>{
                instructions.push(op_code::I32_TO_U32)
            }
            op_code_name::I32_TO_F32=>{
                instructions.push(op_code::I32_TO_F32)
            }
            op_code_name::I32_TO_I64=>{
                instructions.push(op_code::I32_TO_I64)
            }
            op_code_name::I32_TO_U64=>{
                instructions.push(op_code::I32_TO_U64)
            }
            op_code_name::I32_TO_F64=>{
                instructions.push(op_code::I32_TO_F64)
            }
            op_code_name::U32_TO_I32=>{
                instructions.push(op_code::U32_TO_I32)
            }
            op_code_name::U32_TO_F32=>{
                instructions.push(op_code::U32_TO_F32)
            }
            op_code_name::U32_TO_I64=>{
                instructions.push(op_code::U32_TO_I64)
            }
            op_code_name::U32_TO_U64=>{
                instructions.push(op_code::U32_TO_U64)
            }
            op_code_name::U32_TO_F64=>{
                instructions.push(op_code::U32_TO_F64)
            }
            op_code_name::F32_TO_I32=>{
                instructions.push(op_code::F32_TO_I32)
            }
            op_code_name::F32_TO_U32=>{
                instructions.push(op_code::F32_TO_U32)
            }
            op_code_name::F32_TO_I64=>{
                instructions.push(op_code::F32_TO_I64)
            }
            op_code_name::F32_TO_U64=>{
                instructions.push(op_code::F32_TO_U64)
            }
            op_code_name::F32_TO_F64=>{
                instructions.push(op_code::F32_TO_F64)
            }
            op_code_name::I64_TO_I32=>{
                instructions.push(op_code::I64_TO_I32)
            }
            op_code_name::I64_TO_U32=>{
                instructions.push(op_code::I64_TO_U32)
            }
            op_code_name::I64_TO_F32=>{
                instructions.push(op_code::I64_TO_F32)
            }
            op_code_name::I64_TO_U64=>{
                instructions.push(op_code::I64_TO_U64)
            }
            op_code_name::I64_TO_F64=>{
                instructions.push(op_code::I64_TO_F64)
            }
            op_code_name::U64_TO_I32=>{
                instructions.push(op_code::U64_TO_I32)
            }
            op_code_name::U64_TO_U32=>{
                instructions.push(op_code::U64_TO_U32)
            }
            op_code_name::U64_TO_F32=>{
                instructions.push(op_code::U64_TO_F32)
            }
            op_code_name::U64_TO_I64=>{
                instructions.push(op_code::U64_TO_I64)
            }
            op_code_name::U64_TO_F64=>{
                instructions.push(op_code::U64_TO_F64)
            }
            op_code_name::F64_TO_I32=>{
                instructions.push(op_code::F64_TO_I32)
            }
            op_code_name::F64_TO_U32=>{
                instructions.push(op_code::F64_TO_U32)
            }
            op_code_name::F64_TO_F32=>{
                instructions.push(op_code::F64_TO_F32)
            }
            op_code_name::F64_TO_I64=>{
                instructions.push(op_code::F64_TO_I64)
            }
            op_code_name::F64_TO_U64=>{
                instructions.push(op_code::F64_TO_U64)
            }
            _=>{
                panic!("أمر غير متوقع \"{}\"",self.current_instruction)
            }
        }
    
        return instructions;
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
            Err(msg) =>
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
                // TODO: checkIsKufrOrUnsupportedCharacter(current_char);
                char_lit+=&current_char.to_string();
                continue;
            }

            let ctrl_char=self.current_char();
            self.next();

            /*Unicode characters parsing*/
            if ctrl_char=='ي'{
                let c=self.get_next_unicode_char_from_code_point();
                // TODO: checkIsKufrOrUnsupportedCharacter(c);
                char_lit+=&c.to_string();
                continue;
            }

            let es=self.get_next_escape_sequence_form_char(ctrl_char);
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

    fn get_next_escape_sequence_form_char(&self, c:char)->char{
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

}