use std::{fs::{self, File}, io::Read};

use fuel_asm::{Imm06, Imm12, Imm18, Imm24, Instruction, RegId};

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn main() {
    let mut instructions: Vec<Instruction> = Vec::new();
    //read binary file and parse instructions
    let bytes = get_file_as_byte_vec(&String::from("test.bt"));

    for i in (0..bytes.len() / 4 * 4).step_by(4) {
        let inst_bytes : [u8;4] = bytes[i..i+4].try_into().unwrap();
        let instruction = Instruction::try_from(inst_bytes);
        if instruction.is_err() {
            println!("{:#?}",inst_bytes);
        } else {
            print_instruction(&instruction.unwrap());
            println!("");
        }

    }
}


// Code to pretty print bytecode
fn print_reg(r: RegId) -> String {
    match r {
        RegId::BAL => "$bal".to_string(),
        RegId::CGAS => "$cgas".to_string(),
        RegId::ERR => "$err".to_string(),
        RegId::FLAG => "$flag".to_string(),
        RegId::FP => "$fp".to_string(),
        RegId::GGAS => "$ggas".to_string(),
        RegId::HP => "$hp".to_string(),
        RegId::IS => "$is".to_string(),
        RegId::OF => "$of".to_string(),
        RegId::ONE => "$one".to_string(),
        RegId::PC => "$pc".to_string(),
        RegId::RET => "$ret".to_string(),
        RegId::RETL => "$retl".to_string(),
        RegId::SP => "$sp".to_string(),
        RegId::SSP => "$ssp".to_string(),
        RegId::WRITABLE => "$writable".to_string(),
        RegId::ZERO => "$zero".to_string(),
        _ => format!("R{:?}", r.to_u8()),
    }
}

trait Args {
    fn print(&self) -> String;
}

impl Args for RegId {
    fn print(&self) -> String {
        print_reg(*self)
    }
}
impl Args for Imm06 {
    fn print(&self) -> String {
        format!("{:#x}", self.to_u8())
    }
}
impl Args for Imm12 {
    fn print(&self) -> String {
        format!("{:#x}", self.to_u16())
    }
}
impl Args for Imm18 {
    fn print(&self) -> String {
        format!("{:#x}", self.to_u32())
    }
}
impl Args for Imm24 {
    fn print(&self) -> String {
        format!("{:#x}", self.to_u32())
    }
}
impl Args for () {
    fn print(&self) -> String {
        String::new()
    }
}
impl<A: Args> Args for (A,) {
    fn print(&self) -> String {
        self.0.print()
    }
}
impl<A: Args, B: Args> Args for (A, B) {
    fn print(&self) -> String {
        format!("{} {}", self.0.print(), self.1.print())
    }
}
impl<A: Args, B: Args, C: Args> Args for (A, B, C) {
    fn print(&self) -> String {
        format!("{} {} {}", self.0.print(), self.1.print(), self.2.print())
    }
}
impl<A: Args, B: Args, C: Args, D: Args> Args for (A, B, C, D) {
    fn print(&self) -> String {
        format!(
            "{} {} {} {}",
            self.0.print(),
            self.1.print(),
            self.2.print(),
            self.3.print()
        )
    }
}

fn f(name: &str, args: impl Args) {
    let mut line = format!("{name} {}", args.print());
    let s = " ".repeat(48 - line.len());
    line.push_str(&s);
    print!("{line}")
}

fn print_instruction(op: &Instruction) {
    match op {
        Instruction::ADD(x) => f("ADD", x.unpack()),
        Instruction::AND(x) => f("AND", x.unpack()),
        Instruction::DIV(x) => f("DIV", x.unpack()),
        Instruction::EQ(x) => f("EQ", x.unpack()),
        Instruction::EXP(x) => f("EXP", x.unpack()),
        Instruction::GT(x) => f("GT", x.unpack()),
        Instruction::LT(x) => f("LT", x.unpack()),
        Instruction::MLOG(x) => f("MLOG", x.unpack()),
        Instruction::MROO(x) => f("MROO", x.unpack()),
        Instruction::MOD(x) => f("MOD", x.unpack()),
        Instruction::MOVE(x) => f("MOVE", x.unpack()),
        Instruction::MUL(x) => f("MUL", x.unpack()),
        Instruction::NOT(x) => f("NOT", x.unpack()),
        Instruction::OR(x) => f("OR", x.unpack()),
        Instruction::SLL(x) => f("SLL", x.unpack()),
        Instruction::SRL(x) => f("SRL", x.unpack()),
        Instruction::SUB(x) => f("SUB", x.unpack()),
        Instruction::XOR(x) => f("XOR", x.unpack()),
        Instruction::MLDV(x) => f("MLDV", x.unpack()),
        Instruction::RET(x) => f("RET", x.unpack()),
        Instruction::RETD(x) => f("RETD", x.unpack()),
        Instruction::ALOC(x) => f("ALOC", x.unpack()),
        Instruction::MCL(x) => f("MCL", x.unpack()),
        Instruction::MCP(x) => f("MCP", x.unpack()),
        Instruction::MEQ(x) => f("MEQ", x.unpack()),
        Instruction::BHSH(x) => f("BHSH", x.unpack()),
        Instruction::BHEI(x) => f("BHEI", x.unpack()),
        Instruction::BURN(x) => f("BURN", x.unpack()),
        Instruction::CALL(x) => f("CALL", x.unpack()),
        Instruction::CCP(x) => f("CCP", x.unpack()),
        Instruction::CROO(x) => f("CROO", x.unpack()),
        Instruction::CSIZ(x) => f("CSIZ", x.unpack()),
        Instruction::CB(x) => f("CB", x.unpack()),
        Instruction::LDC(x) => f("LDC", x.unpack()),
        Instruction::LOG(x) => f("LOG", x.unpack()),
        Instruction::LOGD(x) => f("LOGD", x.unpack()),
        Instruction::MINT(x) => f("MINT", x.unpack()),
        Instruction::RVRT(x) => f("RVRT", x.unpack()),
        Instruction::SCWQ(x) => f("SCWQ", x.unpack()),
        Instruction::SRW(x) => f("SRW", x.unpack()),
        Instruction::SRWQ(x) => f("SRWQ", x.unpack()),
        Instruction::SWW(x) => f("SWW", x.unpack()),
        Instruction::SWWQ(x) => f("SWWQ", x.unpack()),
        Instruction::TR(x) => f("TR", x.unpack()),
        Instruction::TRO(x) => f("TRO", x.unpack()),
        Instruction::ECK1(x) => f("ECK1", x.unpack()),
        Instruction::ECR1(x) => f("ECR1", x.unpack()),
        Instruction::ED19(x) => f("ED19", x.unpack()),
        Instruction::K256(x) => f("K256", x.unpack()),
        Instruction::S256(x) => f("S256", x.unpack()),
        Instruction::TIME(x) => f("TIME", x.unpack()),
        Instruction::NOOP(_) => f("NOOP", ()),
        Instruction::FLAG(x) => f("FLAG", x.unpack()),
        Instruction::BAL(x) => f("BAL", x.unpack()),
        Instruction::JMP(x) => f("JMP", x.unpack()),
        Instruction::JNE(x) => f("JNE", x.unpack()),
        Instruction::SMO(x) => f("SMO", x.unpack()),
        Instruction::ADDI(x) => f("ADDI", x.unpack()),
        Instruction::ANDI(x) => f("ANDI", x.unpack()),
        Instruction::DIVI(x) => f("DIVI", x.unpack()),
        Instruction::EXPI(x) => f("EXPI", x.unpack()),
        Instruction::MODI(x) => f("MODI", x.unpack()),
        Instruction::MULI(x) => f("MULI", x.unpack()),
        Instruction::ORI(x) => f("ORI", x.unpack()),
        Instruction::SLLI(x) => f("SLLI", x.unpack()),
        Instruction::SRLI(x) => f("SRLI", x.unpack()),
        Instruction::SUBI(x) => f("SUBI", x.unpack()),
        Instruction::XORI(x) => f("XORI", x.unpack()),
        Instruction::JNEI(x) => f("JNEI", x.unpack()),
        Instruction::LB(x) => f("LB", x.unpack()),
        Instruction::LW(x) => f("LW", x.unpack()),
        Instruction::SB(x) => f("SB", x.unpack()),
        Instruction::SW(x) => f("SW", x.unpack()),
        Instruction::MCPI(x) => f("MCPI", x.unpack()),
        Instruction::GTF(x) => f("GTF", x.unpack()),
        Instruction::MCLI(x) => f("MCLI", x.unpack()),
        Instruction::GM(x) => f("GM", x.unpack()),
        Instruction::MOVI(x) => f("MOVI", x.unpack()),
        Instruction::JNZI(x) => f("JNZI", x.unpack()),
        Instruction::JMPF(x) => f("JMPF", x.unpack()),
        Instruction::JMPB(x) => f("JMPB", x.unpack()),
        Instruction::JNZF(x) => f("JNZF", x.unpack()),
        Instruction::JNZB(x) => f("JNZB", x.unpack()),
        Instruction::JNEF(x) => f("JNEF", x.unpack()),
        Instruction::JNEB(x) => f("JNEB", x.unpack()),
        Instruction::JI(x) => f("JI", x.unpack()),
        Instruction::CFEI(x) => f("CFEI", x.unpack()),
        Instruction::CFSI(x) => f("CFSI", x.unpack()),
        Instruction::CFE(x) => f("CFE", x.unpack()),
        Instruction::CFS(x) => f("CFS", x.unpack()),
        Instruction::PSHL(x) => f("PSHL", x.unpack()),
        Instruction::PSHH(x) => f("PSHH", x.unpack()),
        Instruction::POPL(x) => f("POPL", x.unpack()),
        Instruction::POPH(x) => f("POPH", x.unpack()),
        Instruction::WDCM(x) => f("WDCM", x.unpack()),
        Instruction::WQCM(x) => f("WQCM", x.unpack()),
        Instruction::WDOP(x) => f("WDOP", x.unpack()),
        Instruction::WQOP(x) => f("WQOP", x.unpack()),
        Instruction::WDML(x) => f("WDML", x.unpack()),
        Instruction::WQML(x) => f("WQML", x.unpack()),
        Instruction::WDDV(x) => f("WDDV", x.unpack()),
        Instruction::WQDV(x) => f("WQDV", x.unpack()),
        Instruction::WDMD(x) => f("WDMD", x.unpack()),
        Instruction::WQMD(x) => f("WQMD", x.unpack()),
        Instruction::WDAM(x) => f("WDAM", x.unpack()),
        Instruction::WQAM(x) => f("WQAM", x.unpack()),
        Instruction::WDMM(x) => f("WDMM", x.unpack()),
        Instruction::WQMM(x) => f("WQMM", x.unpack()),
        Instruction::ECAL(x) => f("ECAL", x.unpack()),
        Instruction::BSIZ(x) => f("BSIZ", x.unpack()),
        Instruction::BLDD(x) => f("BLDD", x.unpack()),
        _ => {}
    }
}

