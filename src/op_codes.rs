#![recursion_limit = "256"]

use iota::iota;

pub mod op_code_name{

    pub const NOP:&str="لا_شيء";

    pub const HALT:&str="توقف";

    pub const PRINT:&str="اطبع";
    pub const PRINTLN:&str="اطبع_";

    pub const CALL:&str="استدع";

    pub const GOTO:&str="انتقل";
    pub const GOTO_IF_0:&str="انتقل0";
    pub const GOTO_IF_NOT_0:&str="انتقل!0";

    pub const PUSH_I32:&str="أدرج_ص32";
    pub const PUSH_U32:&str="أدرج_م32";
    pub const PUSH_F32:&str="أدرج_ع32";
    pub const PUSH_I64:&str="أدرج_ص64";
    pub const PUSH_U64:&str="أدرج_م64";
    pub const PUSH_F64:&str="أدرج_ع64";
    pub const PUSH_CHAR:&str="أدرج_حرف";
    pub const PUSH_STR:&str="أدرج_نص";
    pub const PUSH_TRUE:&str="أدرج_صواب";
    pub const PUSH_FALSE:&str="أدرج_خطأ";

    pub const PUSH_TO_RBP:&str="أدرج_إلى_س";
    pub const PUSH_TO_RSP:&str="أدرج_إلى_ف";
    pub const PUSH_TO_RAX:&str="أدرج_إلى_أ";
    pub const PUSH_TO_RBX:&str="أدرج_إلى_ب";
    pub const PUSH_TO_RCX:&str="أدرج_إلى_ج";
    pub const PUSH_TO_RDX:&str="أدرج_إلى_د";
    pub const PUSH_TO_RDI:&str="أدرج_إلى_م";
    pub const PUSH_TO_RSI:&str="أدرج_إلى_ن";

    pub const POP:&str="استخرج";
    pub const POP_TO_RBP:&str="استخرج_إلى_س";
    pub const POP_TO_RSP:&str="استخرج_إلى_ف";
    pub const POP_TO_RAX:&str="استخرج_إلى_أ";
    pub const POP_TO_RBX:&str="استخرج_إلى_ب";
    pub const POP_TO_RCX:&str="استخرج_إلى_ج";
    pub const POP_TO_RDX:&str="استخرج_إلى_د";
    pub const POP_TO_RDI:&str="استخرج_إلى_م";
    pub const POP_TO_RSI:&str="استخرج_إلى_ن";

    pub const DUP:&str="انسخ";
    pub const DUP_RBP:&str="انسخ_س";
    pub const DUP_RSP:&str="انسخ_ف";
    pub const DUP_RAX:&str="انسخ_أ";
    pub const DUP_RBX:&str="انسخ_ب";
    pub const DUP_RCX:&str="انسخ_ج";
    pub const DUP_RDX:&str="انسخ_د";
    pub const DUP_RDI:&str="انسخ_م";
    pub const DUP_RSI:&str="انسخ_ن";

    pub const ADD_I32:&str="اجمع_ص32";
    pub const ADD_U32:&str="اجمع_م32";
    pub const ADD_F32:&str="اجمع_ع32";
    pub const ADD_I64:&str="اجمع_ص64";
    pub const ADD_U64:&str="اجمع_م64";
    pub const ADD_F64:&str="اجمع_ع64";

    pub const SUB_I32:&str="اطرح_ص32";
    pub const SUB_U32:&str="اطرح_م32";
    pub const SUB_F32:&str="اطرح_ع32";
    pub const SUB_I64:&str="اطرح_ص64";
    pub const SUB_U64:&str="اطرح_م64";
    pub const SUB_F64:&str="اطرح_ع64";

    pub const MUL_I32:&str="اضرب_ص32";
    pub const MUL_U32:&str="اضرب_م32";
    pub const MUL_F32:&str="اضرب_ع32";
    pub const MUL_I64:&str="اضرب_ص64";
    pub const MUL_U64:&str="اضرب_م64";
    pub const MUL_F64:&str="اضرب_ع64";

    pub const DIV_I32:&str="اقسم_ص32";
    pub const DIV_U32:&str="اقسم_م32";
    pub const DIV_F32:&str="اقسم_ع32";
    pub const DIV_I64:&str="اقسم_ص64";
    pub const DIV_U64:&str="اقسم_م64";
    pub const DIV_F64:&str="اقسم_ع64";

    pub const REM_I32:&str="باقي_قسمة_ص32";
    pub const REM_U32:&str="باقي_قسمة_م32";
    pub const REM_F32:&str="باقي_قسمة_ع32";
    pub const REM_I64:&str="باقي_قسمة_ص64";
    pub const REM_U64:&str="باقي_قسمة_م64";
    pub const REM_F64:&str="باقي_قسمة_ع64";

    pub const SHR_I32:&str="زح_يمين_ص32";
    pub const SHR_U32:&str="زح_يمين_م32";
    pub const SHR_I64:&str="زح_يمين_ص64";
    pub const SHR_U64:&str="زح_يمين_م64";

    pub const SHL_I32:&str="زح_يسار_ص32";
    pub const SHL_U32:&str="زح_يسار_م32";
    pub const SHL_I64:&str="زح_يسار_ص64";
    pub const SHL_U64:&str="زح_يسار_م64";

    pub const XOR_I32:&str="عدم_تكافؤ_ص32";
    pub const XOR_U32:&str="عدم_تكافؤ_م32";
    pub const XOR_I64:&str="عدم_تكافؤ_ص64";
    pub const XOR_U64:&str="عدم_تكافؤ_م64";

    pub const AND_I32:&str="مع_ص32";
    pub const AND_U32:&str="مع_م32";
    pub const AND_I64:&str="مع_ص64";
    pub const AND_U64:&str="مع_م64";

    pub const OR_I32:&str="أو_ص32";
    pub const OR_U32:&str="أو_م32";
    pub const OR_I64:&str="أو_ص64";
    pub const OR_U64:&str="أو_م64";

    pub const NOT_I32:&str="عكس_ص32";
    pub const NOT_U32:&str="عكس_م32";
    pub const NOT_I64:&str="عكس_ص64";
    pub const NOT_U64:&str="عكس_م64";

    pub const NEG_I32:&str="سالب_ص32";
    pub const NEG_F32:&str="سالب_ع32";
    pub const NEG_I64:&str="سالب_ص64";
    pub const NEG_F64:&str="سالب_ع64";

    pub const I32_TO_U32:&str="ص32_إلى_م32";
    pub const I32_TO_F32:&str="ص32_إلى_ع32";
    pub const I32_TO_I64:&str="ص32_إلى_ص64";
    pub const I32_TO_U64:&str="ص32_إلى_م64";
    pub const I32_TO_F64:&str="ص32_إلى_ع64";

    pub const U32_TO_I32:&str="م32_إلى_ص32";
    pub const U32_TO_F32:&str="م32_إلى_ع32";
    pub const U32_TO_I64:&str="م32_إلى_ص64";
    pub const U32_TO_U64:&str="م32_إلى_م64";
    pub const U32_TO_F64:&str="م32_إلى_ع64";

    pub const F32_TO_I32:&str="ع32_إلى_ص32";
    pub const F32_TO_U32:&str="ع32_إلى_م32";
    pub const F32_TO_I64:&str="ع32_إلى_ص64";
    pub const F32_TO_U64:&str="ع32_إلى_م64";
    pub const F32_TO_F64:&str="ع32_إلى_ع64";

    pub const I64_TO_I32:&str="ص64_إلى_ص32";
    pub const I64_TO_U32:&str="ص64_إلى_م32";
    pub const I64_TO_F32:&str="ص64_إلى_ع32";
    pub const I64_TO_U64:&str="ص64_إلى_م64";
    pub const I64_TO_F64:&str="ص64_إلى_ع64";

    pub const U64_TO_I32:&str="م64_إلى_ص32";
    pub const U64_TO_U32:&str="م64_إلى_م32";
    pub const U64_TO_F32:&str="م64_إلى_ع32";
    pub const U64_TO_I64:&str="م64_إلى_ص64";
    pub const U64_TO_F64:&str="م64_إلى_ع64";

    pub const F64_TO_I32:&str="ع64_إلى_ص32";
    pub const F64_TO_U32:&str="ع64_إلى_م32";
    pub const F64_TO_F32:&str="ع64_إلى_ع32";
    pub const F64_TO_I64:&str="ع64_إلى_ص64";
    pub const F64_TO_U64:&str="ع64_إلى_م64";

}

pub mod op_code{
    
    super::iota!{
        pub const NOP:u8=iota;
                , HALT
                , PRINT
                , PRINTLN
                , CALL
                , GOTO
                , GOTO_IF_0
                , GOTO_IF_NOT_0
                , PUSH_I32
                , PUSH_U32
                , PUSH_F32
                , PUSH_I64
                , PUSH_U64
                , PUSH_F64
                , PUSH_CHAR
                , PUSH_STR
                , PUSH_TRUE
                , PUSH_FALSE
                , PUSH_TO_RBP
                , PUSH_TO_RSP
                , PUSH_TO_RAX
                , PUSH_TO_RBX
                , PUSH_TO_RCX
                , PUSH_TO_RDX
                , PUSH_TO_RDI
                , PUSH_TO_RSI
                , POP
                , POP_TO_RBP
                , POP_TO_RSP
                , POP_TO_RAX
                , POP_TO_RBX
                , POP_TO_RCX
                , POP_TO_RDX
                , POP_TO_RDI
                , POP_TO_RSI
                , DUP
                , DUP_RBP
                , DUP_RSP
                , DUP_RAX
                , DUP_RBX
                , DUP_RCX
                , DUP_RDX
                , DUP_RDI
                , DUP_RSI
                , ADD_I32
                , ADD_U32
                , ADD_F32
                , ADD_I64
                , ADD_U64
                , ADD_F64
                , SUB_I32
                , SUB_U32
                , SUB_F32
                , SUB_I64
                , SUB_U64
                , SUB_F64
                , MUL_I32
                , MUL_U32
                , MUL_F32
                , MUL_I64
                , MUL_U64
                , MUL_F64
                , DIV_I32
                , DIV_U32
                , DIV_F32
                , DIV_I64
                , DIV_U64
                , DIV_F64
                , REM_I32
                , REM_U32
                , REM_F32
                , REM_I64
                , REM_U64
                , REM_F64
                , SHR_I32
                , SHR_U32
                , SHR_I64
                , SHR_U64
                , SHL_I32
                , SHL_U32
                , SHL_I64
                , SHL_U64
                , XOR_I32
                , XOR_U32
                , XOR_I64
                , XOR_U64
                , AND_I32
                , AND_U32
                , AND_I64
                , AND_U64
                , OR_I32
                , OR_U32
                , OR_I64
                , OR_U64
                , NOT_I32
                , NOT_U32
                , NOT_I64
                , NOT_U64
                , NEG_I32
                , NEG_F32
                , NEG_I64
                , NEG_F64
                , I32_TO_U32
                , I32_TO_F32
                , I32_TO_I64
                , I32_TO_U64
                , I32_TO_F64
                , U32_TO_I32
                , U32_TO_F32
                , U32_TO_I64
                , U32_TO_U64
                , U32_TO_F64
                , F32_TO_I32
                , F32_TO_U32
                , F32_TO_I64
                , F32_TO_U64
                , F32_TO_F64
                , I64_TO_I32
                , I64_TO_U32
                , I64_TO_F32
                , I64_TO_U64
                , I64_TO_F64
                , U64_TO_I32
                , U64_TO_U32
                , U64_TO_F32
                , U64_TO_I64
                , U64_TO_F64
                , F64_TO_I32
                , F64_TO_U32
                , F64_TO_F32
                , F64_TO_I64
                , F64_TO_U64
    }

}