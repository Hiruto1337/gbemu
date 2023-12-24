use crate::comps::{instructions::{inst_by_opcode, AddrMode, RegType}, emu::EMULATOR};

use super::{cpu::CPUContext, bus::bus_read};

impl CPUContext {
    pub fn fetch_instruction(&mut self) {
        self.cur_opcode = bus_read(self, self.registers.pc);
        self.registers.pc += 1;
        self.cur_inst = inst_by_opcode(self.cur_opcode);
    }
    
    pub fn fetch_data(&mut self) {
        self.mem_dest = 0;
        self.dest_is_mem = false;
    
        type AM = AddrMode;
        match self.cur_inst.mode {
            AM::IMP => {},
            AM::R => {
                self.fetched_data = self.read_reg(self.cur_inst.reg1);
            },
            AM::RxR => {
                self.fetched_data = self.read_reg(self.cur_inst.reg2);
            },
            AM::RxD8 => {
                self.fetched_data = bus_read(self, self.registers.pc) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
                self.registers.pc += 1;
            },
            AM::D16 | AM::RxD16 => {
                let lo = bus_read(self, self.registers.pc) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
    
                let hi = bus_read(self, self.registers.pc + 1) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
                
                self.fetched_data = (hi << 8) | lo;
                self.registers.pc += 2;
            },
            AM::MRxR => {
                self.fetched_data = self.read_reg(self.cur_inst.reg2);
                self.mem_dest = self.read_reg(self.cur_inst.reg1);
                self.dest_is_mem = true;
    
                if self.cur_inst.reg1.unwrap() == RegType::C {
                    self.mem_dest |= 0xFF00;
                }
            },
            AM::RxMR => {
                let mut addr = self.read_reg(self.cur_inst.reg2);
    
                if self.cur_inst.reg1.unwrap() == RegType::C {
                    addr |= 0xFF00;
                }
    
                self.fetched_data = bus_read(self, addr) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
            },
            AM::RxHLI | AM::RxHLD => {
                let address = self.read_reg(self.cur_inst.reg2);
                self.fetched_data = bus_read(self, address) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
    
                if self.cur_inst.mode == AM::RxHLI {
                    let val = self.read_reg(Some(RegType::HL)) + 1;
                    self.set_reg(Some(RegType::HL), val);
                } else {
                    let val = self.read_reg(Some(RegType::HL)) - 1;
                    self.set_reg(Some(RegType::HL), val);
                }
            },
            AM::HLIxR | AM::HLDxR => {
                self.fetched_data = self.read_reg(self.cur_inst.reg2);
                self.mem_dest = self.read_reg(self.cur_inst.reg1);
                self.dest_is_mem = true;
    
                if self.cur_inst.mode == AM::HLIxR {
                    let val = self.read_reg(Some(RegType::HL)) + 1;
                    self.set_reg(Some(RegType::HL), val);
                } else {
                    let val = self.read_reg(Some(RegType::HL)) - 1;
                    self.set_reg(Some(RegType::HL), val);
                }
            },
            AM::RxA8 => {
                self.fetched_data = bus_read(self, self.registers.pc) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
                self.registers.pc += 1;
            },
            AM::A8xR => {
                self.mem_dest = bus_read(self, self.registers.pc) as u16 | 0xFF00;
                self.dest_is_mem = true;
                EMULATOR.lock().unwrap().cycles(self, 1);
                self.registers.pc += 1;
            },
            AM::HLxSPR => {
                self.fetched_data = bus_read(self, self.registers.pc) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
                self.registers.pc += 1;
            },
            AM::D8 => {
                self.fetched_data = bus_read(self, self.registers.pc) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
                self.registers.pc += 1;
            },
            AM::D16xR | AM::A16xR => {
                let lo = bus_read(self, self.registers.pc) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
    
                let hi = bus_read(self, self.registers.pc + 1) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
                
                self.mem_dest = (hi << 8) | lo;
                self.dest_is_mem = true;
    
                self.registers.pc += 2;
                self.fetched_data = self.read_reg(self.cur_inst.reg2);
            },
            AM::MRxD8 => {
                self.fetched_data = bus_read(self, self.registers.pc) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
                self.registers.pc += 1;
                self.mem_dest = self.read_reg(self.cur_inst.reg1);
                self.dest_is_mem = true;
            },
            AM::MR => {
                self.mem_dest = self.read_reg(self.cur_inst.reg1) as u16;
                self.dest_is_mem = true;
                let address = self.read_reg(self.cur_inst.reg1);
                self.fetched_data = bus_read(self, address) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
            },
            AM::RxA16 => {
                let lo = bus_read(self, self.registers.pc) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
    
                let hi = bus_read(self, self.registers.pc + 1) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
    
                let addr = (hi << 8) | lo;
    
                self.registers.pc += 2;
                self.fetched_data = bus_read(self, addr) as u16;
                EMULATOR.lock().unwrap().cycles(self, 1);
            }
        }
    }
}
