/*
0x0000 - 0x3FFF: ROM Bank 00 (fixed; typically contains the boot ROM and initial program code).
0x4000 - 0x7FFF: ROM Bank 01~NN (switchable; used for larger games).
0x8000 - 0x9FFF: Video RAM (VRAM) (used by the GPU).
0xA000 - 0xBFFF: External RAM (provided by the game cartridge).
0xC000 - 0xDFFF: Work RAM (WRAM) (internal RAM for general use).
0xE000 - 0xFDFF: Echo RAM (mirror of C000~DDFF; rarely used).
0xFE00 - 0xFE9F: Sprite Attribute Table (OAM) (used for sprite data).
0xFEA0 - 0xFEFF: Not Usable.
0xFF00 - 0xFF7F: I/O Registers (used for hardware control).
0xFF80 - 0xFFFE: High RAM (HRAM) (fast internal RAM).
0xFFFF: Interrupt Enable Register.
*/

pub struct Memory {
    pub rom: [u8; 0x8000],  // 0x0000 - 0x7FFFF
    pub vram: [u8; 0x2000], // 0x8000 - 0x9FFF
    pub eram: [u8; 0x2000], // 0xA000 - 0xBFFF
    pub wram: [u8; 0x2000], // 0xC000 - 0xDFFF
    // 0xE000 - 0xFDFF (echo RAM)
    pub oam: [u8; 0xA0], // 0xFE00 - 0xFE9F
    // 0xFEA0 - 0xFEFF not usable
    pub io: [u8; 0x80],       // 0xFF00 - 0xFF7F
    pub hram: [u8; 0x7F],     // 0xFF80 - 0xFFFE
    pub interrupt_enable: u8, // 0xFFFF
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            rom: [0; 0x8000],
            vram: [0; 0x2000],
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io: [0; 0x80],
            hram: [0; 0x7F],
            interrupt_enable: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.eram[(addr - 0xA000) as usize],
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize], // Echo RAM mirrors WRAM
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable,
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.rom[address as usize] = value,
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.eram[(address - 0xA000) as usize] = value,
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize] = value, // Echo RAM mirrors WRAM
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFF00..=0xFF7F => self.io[(address - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.interrupt_enable = value,
            _ => {}
        }
    }
}
