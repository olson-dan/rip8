use std::fmt;
use std::time::Instant;

#[derive(Copy, Clone)]
struct Address {
    value: u16,
}

impl Address {
    fn new(x: u8, y: u8, z: u8) -> Address {
        Address {
            value: ((x as u16) << 8) | ((y as u16) << 4) | (z as u16),
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:03X}", self.value)
    }
}

#[derive(Copy, Clone)]
struct Constant {
    value: u16,
}

impl Constant {
    fn new(x: u8, y: u8) -> Constant {
        Constant {
            value: ((x as u16) << 4) | (y as u16),
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}", self.value)
    }
}

const NUM_REGISTERS: usize = 16;

#[derive(Copy, Clone)]
enum Register {
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
    V7 = 7,
    V8 = 8,
    V9 = 9,
    VA = 10,
    VB = 11,
    VC = 12,
    VD = 13,
    VE = 14,
    VF = 15,
}

impl Register {
    fn new(x: u8) -> Register {
        match x {
            0 => Register::V0,
            1 => Register::V1,
            2 => Register::V2,
            3 => Register::V3,
            4 => Register::V4,
            5 => Register::V5,
            6 => Register::V6,
            7 => Register::V7,
            8 => Register::V8,
            9 => Register::V9,
            10 => Register::VA,
            11 => Register::VB,
            12 => Register::VC,
            13 => Register::VD,
            14 => Register::VE,
            15 => Register::VF,
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "V{:X}", *self as u32)
    }
}

enum Instruction {
    SysCall(Address),
    ClearScreen,
    Return,
    Jump(Address),
    Call(Address),
    SkipIfEqual(Register, Constant),
    SkipIfNotEqual(Register, Constant),
    SkipIfRegistersEqual(Register, Register),
    SetImmediate(Register, Constant),
    AddImmediate(Register, Constant),
    SetRegister(Register, Register),
    OrRegister(Register, Register),
    AndRegister(Register, Register),
    XorRegister(Register, Register),
    AdcRegister(Register, Register),
    SwbRegister(Register, Register),
    ShrRegister(Register, Register),
    ReverseSwbRegister(Register, Register),
    ShlRegister(Register, Register),
    SkipIfRegistersNotEqual(Register, Register),
    StoreAddress(Address),
    JumpOffset(Address),
    StoreRandom(Register, Constant),
    DrawSprite(Register, Register, Constant),
    SkipIfPressed(Register),
    SkipIfNotPressed(Register),
    SetFromDelay(Register),
    WaitKeyPress(Register),
    SetToDelay(Register),
    SetToSound(Register),
    AddAddress(Register),
    SetAddressToSprite(Register),
    StoreBCD(Register),
    StoreRegisters(Register),
    LoadRegisters(Register),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::ClearScreen => write!(f, "CLS"),
            Instruction::Return => write!(f, "RET"),
            Instruction::SysCall(addr) => write!(f, "SYS {}", addr),
            Instruction::Jump(addr) => write!(f, "JP {}", addr),
            Instruction::Call(addr) => write!(f, "CALL {}", addr),
            Instruction::SkipIfEqual(x, c) => write!(f, "SE {}, {}", x, c),
            Instruction::SkipIfNotEqual(x, c) => write!(f, "SNE {}, {}", x, c),
            Instruction::SkipIfRegistersEqual(x, y) => write!(f, "SE {}, {}", x, y),
            Instruction::SetImmediate(x, c) => write!(f, "LD {}, {}", x, c),
            Instruction::AddImmediate(x, c) => write!(f, "ADD {}, {}", x, c),
            Instruction::SetRegister(x, y) => write!(f, "LD {}, {}", x, y),
            Instruction::OrRegister(x, y) => write!(f, "OR {}, {}", x, y),
            Instruction::AndRegister(x, y) => write!(f, "AND {}, {}", x, y),
            Instruction::XorRegister(x, y) => write!(f, "XOR {}, {}", x, y),
            Instruction::AdcRegister(x, y) => write!(f, "ADD {}, {}", x, y),
            Instruction::SwbRegister(x, y) => write!(f, "SUB {}, {}", x, y),
            Instruction::ShrRegister(x, y) => write!(f, "SHR {}, {}", x, y),
            Instruction::ReverseSwbRegister(x, y) => write!(f, "SUBN {}, {}", x, y),
            Instruction::ShlRegister(x, y) => write!(f, "SHL {}, {}", x, y),
            Instruction::SkipIfRegistersNotEqual(x, y) => write!(f, "SNE {}, {}", x, y),
            Instruction::StoreAddress(addr) => write!(f, "LD I, {}", addr),
            Instruction::JumpOffset(addr) => write!(f, "JP V0, {}", addr),
            Instruction::StoreRandom(x, c) => write!(f, "RND {}, {}", x, c),
            Instruction::DrawSprite(x, y, c) => write!(f, "DRW {}, {}, {}", x, y, c),
            Instruction::SkipIfPressed(x) => write!(f, "SKP {}", x),
            Instruction::SkipIfNotPressed(x) => write!(f, "SKNP {}", x),
            Instruction::SetFromDelay(x) => write!(f, "LD {}, DT", x),
            Instruction::WaitKeyPress(x) => write!(f, "LD {}, K", x),
            Instruction::SetToDelay(x) => write!(f, "LD DT, {}", x),
            Instruction::SetToSound(x) => write!(f, "LD ST, {}", x),
            Instruction::AddAddress(x) => write!(f, "ADD I, {}", x),
            Instruction::SetAddressToSprite(x) => write!(f, "LD F, {}", x),
            Instruction::StoreBCD(x) => write!(f, "LD B, {}", x),
            Instruction::StoreRegisters(x) => write!(f, "LD [I], {}", x),
            Instruction::LoadRegisters(x) => write!(f, "LD {}, [I]", x),
            _ => unimplemented!(),
        }
    }
}

struct MachineState {
    ip: usize,
    sp: usize,
    finished: bool,
    addr: Address,
    registers: [u8; NUM_REGISTERS],
}

impl MachineState {
    fn new() -> MachineState {
        MachineState {
            ip: 0x200,
            sp: 0,
            finished: false,
            addr: Address { value: 0 },
            registers: [0; NUM_REGISTERS],
        }
    }
}

struct Timers {
    delay: u16,
    sound: u16,
    last_update: Instant,
}

impl Timers {
    fn new() -> Timers {
        Timers {
            delay: 0,
            sound: 0,
            last_update: Instant::now(),
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let diff = now.duration_since(self.last_update);
        if diff.subsec_micros() > 1660 {
            self.delay = self.delay.saturating_sub(1);
            self.sound = self.sound.saturating_sub(1);
            self.last_update = now;
        }
    }
}

fn decode_instruction(state: MachineState, memory: &[u8]) -> Instruction {
    let a = (memory[state.ip + 0] & 0xf0) >> 4;
    let b = (memory[state.ip + 0] & 0x0f) >> 0;
    let c = (memory[state.ip + 1] & 0xf0) >> 4;
    let d = (memory[state.ip + 1] & 0x0f) >> 0;
    match a {
        0x0 if b == 0x0 && c == 0xe && d == 0x0 => Instruction::ClearScreen,
        0x0 if b == 0x0 && c == 0xe && d == 0xe => Instruction::Return,
        0x0 => Instruction::SysCall(Address::new(b, c, d)),
        0x1 => Instruction::Jump(Address::new(b, c, d)),
        0x2 => Instruction::Call(Address::new(b, c, d)),
        0x3 => Instruction::SkipIfEqual(Register::new(b), Constant::new(c, d)),
        0x4 => Instruction::SkipIfNotEqual(Register::new(b), Constant::new(c, d)),
        0x5 if d == 0x0 => Instruction::SkipIfRegistersEqual(Register::new(b), Register::new(c)),
        0x6 => Instruction::SetImmediate(Register::new(b), Constant::new(c, d)),
        0x7 => Instruction::AddImmediate(Register::new(b), Constant::new(c, d)),
        0x8 if d == 0x0 => Instruction::SetRegister(Register::new(b), Register::new(c)),
        0x8 if d == 0x1 => Instruction::OrRegister(Register::new(b), Register::new(c)),
        0x8 if d == 0x2 => Instruction::AndRegister(Register::new(b), Register::new(c)),
        0x8 if d == 0x3 => Instruction::XorRegister(Register::new(b), Register::new(c)),
        0x8 if d == 0x4 => Instruction::AdcRegister(Register::new(b), Register::new(c)),
        0x8 if d == 0x5 => Instruction::SwbRegister(Register::new(b), Register::new(c)),
        0x8 if d == 0x6 => Instruction::ShrRegister(Register::new(b), Register::new(c)),
        0x8 if d == 0x7 => Instruction::ReverseSwbRegister(Register::new(b), Register::new(c)),
        0x8 if d == 0xe => Instruction::ShlRegister(Register::new(b), Register::new(c)),
        0x9 if d == 0x0 => Instruction::SkipIfRegistersNotEqual(Register::new(b), Register::new(c)),
        0xa => Instruction::StoreAddress(Address::new(b, c, d)),
        0xb => Instruction::JumpOffset(Address::new(b, c, d)),
        0xc => Instruction::StoreRandom(Register::new(b), Constant::new(c, d)),
        0xd => Instruction::DrawSprite(Register::new(b), Register::new(c), Register::new(d)),
        0xe if c == 0x9 && d == 0xe => Instruction::SkipIfPressed(Register::new(b)),
        _ => panic!(format!(
            "Unknown opcode at {:03X}: Instrcu{:02X}{:02X}{:02X}{:02X}",
            state.ip, a, b, c, d
        )),
    }
}
