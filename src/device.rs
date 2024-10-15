use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use crate::{cpu::CPU, memory::Memory, ppu::PPU};

pub type SharedMemory = Rc<RefCell<Memory>>;

pub struct Device {
    pub cpu: CPU,
    pub ppu: Rc<RefCell<PPU>>,
    pub memory: SharedMemory,
}

impl Device {
    pub fn new() -> Device {
        let memory = Rc::new(RefCell::new(Memory::new()));
        let ppu = Rc::new(RefCell::new(PPU::new(memory.clone())));
        let cpu = CPU::new(memory.clone(), ppu.clone());

        Device {
            memory: memory.clone(),
            cpu,
            ppu,
        }
    }

    pub fn load_instructions(&mut self, filename: &str) {
        let mut file = File::open(filename).expect("error trying to open the file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("error trying to read");

        let start_address = 0x0000;
        for (i, &byte) in buffer.iter().enumerate() {
            self.memory
                .borrow_mut()
                .write_byte((start_address + i) as u16, byte);
        }
    }
}
