use crate::comps::{instructions::AddrMode, emu::EMULATOR, bus::bus_read};

use super::{instructions::Instruction, common::*, cpu_proc::proc_by_inst, interrupts::*};

pub struct CPUContext {
    pub registers: Registers,
    pub fetched_data: u16,
    pub mem_dest: u16,
    pub dest_is_mem: bool,
    pub cur_opcode: u8,
    pub cur_inst: &'static Instruction,
    pub halted: bool,
    pub stepping: bool,
    pub int_master_enabled: bool,
    pub enabling_ime: bool,
    pub int_flags: u8,
    pub ie_register: u8,
}

impl CPUContext {
    pub fn step(&mut self) {
        if !self.halted {
            let pc = self.registers.pc;

            // let log = format!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            //     self.registers.a,
            //     self.registers.f,
            //     self.registers.b,
            //     self.registers.c,
            //     self.registers.d,
            //     self.registers.e,
            //     self.registers.h,
            //     self.registers.l,
            //     self.registers.sp,
            //     pc,
            //     bus_read(self, pc),
            //     bus_read(self, pc + 1),
            //     bus_read(self, pc + 2),
            //     bus_read(self, pc + 3),
            // );
            
            self.fetch_instruction();
            EMULATOR.lock().unwrap().cycles(self, 1);
            self.fetch_data();

            let flags = format!("{}{}{}{}",
                if self.registers.f & (1 << 7) != 0 {"Z"} else {"-"},
                if self.registers.f & (1 << 6) != 0 {"N"} else {"-"},
                if self.registers.f & (1 << 5) != 0 {"H"} else {"-"},
                if self.registers.f & (1 << 4) != 0 {"C"} else {"-"},
            );

            println!("{:08X} - ${:04X}: {:10} ({:02X}, {:02X}, {:02X}), A: {:02X}, F: {} BC: {:02X}{:02X}, DE: {:02X}{:02X}, HL: {:02X}{:02X}",
                EMULATOR.lock().unwrap().ticks,
                pc,
                self.inst_string(),
                self.cur_opcode.to_owned(),
                bus_read(self, pc + 1),
                bus_read(self, pc + 2),
                self.registers.a,
                flags,
                self.registers.b,
                self.registers.c,
                self.registers.d,
                self.registers.e,
                self.registers.h,
                self.registers.l,
            );

            // dbg_update(self);
            // dbg_print();

            self.execute();
        } else {
            EMULATOR.lock().unwrap().cycles(self, 1);
            if self.int_flags != 0 {
                self.halted = false;
            }
        }

        if self.int_master_enabled {
            handle_interrupts(self);
            self.enabling_ime = false;
        }

        if self.enabling_ime {
            self.int_master_enabled = true;
        }
    }

    fn execute(&mut self) {
        let proc = proc_by_inst(self.cur_inst.inst_type);

        proc(self);
    }

    pub fn set_flags(&mut self, z: Option<bool>, n: Option<bool>, h: Option<bool>, c: Option<bool>) {
        let flags = &mut self.registers.f;

        if let Some(z) = z {
            bit_set(flags, 7, z);
        }

        if let Some(n) = n {
            bit_set(flags, 6, n);
        }

        if let Some(h) = h {
            bit_set(flags, 5, h);
        }

        if let Some(c) = c {
            bit_set(flags, 4, c);
        }
    }

    pub fn flag_z(&self) -> u8 {
        bit(self.registers.f, 7)
    }

    pub fn flag_n(&self) -> u8 {
        bit(self.registers.f, 6)
    }

    pub fn flag_h(&self) -> u8 {
        bit(self.registers.f, 5)
    }

    pub fn flag_c(&self) -> u8 {
        bit(self.registers.f, 4)
    }

    pub fn get_ie_reg(&self) -> u8 {
        self.ie_register
    }

    pub fn set_ie_reg(&mut self, value: u8) {
        self.ie_register = value;
    }

    fn inst_string(&mut self) -> String {
        type AM = AddrMode;
        let inst = self.cur_inst;

        format!("{} {}",
            self.cur_inst.inst_type,
            match self.cur_inst.mode {
                AM::IMP => format!(""),
                AM::RxD16 | AM::RxA16 => format!("{},${:04X}", inst.reg1.unwrap(), self.fetched_data),
                AM::RxR => format!("{},{}", inst.reg1.unwrap(), inst.reg2.unwrap()),
                AM::MRxR => format!("({}),{}", inst.reg1.unwrap(), inst.reg2.unwrap()),
                AM::R => format!("{}", inst.reg1.unwrap()),
                AM::RxD8 | AM::RxA8 => format!("{},${:02X}", inst.reg1.unwrap(), self.fetched_data as u8),
                AM::RxMR => format!("{},({})", inst.reg1.unwrap(), inst.reg2.unwrap()),
                AM::RxHLI => format!("{},({}+)", inst.reg1.unwrap(), inst.reg2.unwrap()),
                AM::RxHLD => format!("{},({}-)", inst.reg1.unwrap(), inst.reg2.unwrap()),
                AM::HLIxR => format!("({}+),{}", inst.reg1.unwrap(), inst.reg2.unwrap()),
                AM::HLDxR => format!("({}-),{}", inst.reg1.unwrap(), inst.reg2.unwrap()),
                AM::A8xR => format!("{},{}", bus_read(self, self.registers.pc - 1), inst.reg2.unwrap()),
                AM::HLxSPR => format!("({}),SP+${:02X}", inst.reg1.unwrap(), self.fetched_data as u8),
                AM::D16 => format!("${:04X}", self.fetched_data),
                AM::D8 => format!("${:02X}", self.fetched_data as u8),
                AM::D16xR => format!(""),
                AM::MRxD8 => format!("({}),${:02X}", inst.reg1.unwrap(), self.fetched_data as u8),
                AM::MR => format!("({})", inst.reg1.unwrap()),
                AM::A16xR => format!("(${:04X}),{}", self.fetched_data, inst.reg2.unwrap()),
            }
        )
    }

    pub fn request_interrupt(&mut self, int_type: InterruptType) {
        self.int_flags |= int_type as u8;
    }
}

pub struct Registers {
    pub a: u8,
    pub f: u8, 
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16
}
