use crate::device::SharedMemory;

pub struct PPU {
    pub memory: SharedMemory,
    pub mode: u8,
    pub mode_clock: u32,
    pub line: u8,
    pub framebuffer: [u32; 160 * 144],
    pub frame_ready: bool,
}

impl PPU {
    const LCDC_ADDR: u16 = 0xFF40;
    const STAT_ADDR: u16 = 0xFF41;
    const SCY_ADDR: u16 = 0xFF42;
    const SCX_ADDR: u16 = 0xFF43;
    const LY_ADDR: u16 = 0xFF44;
    const LYC_ADDR: u16 = 0xFF45;
    const BGP_ADDR: u16 = 0xFF47;
    const OBP0_ADDR: u16 = 0xFF48;
    const OBP1_ADDR: u16 = 0xFF49;
    const WY_ADDR: u16 = 0xFF4A;
    const WX_ADDR: u16 = 0xFF4B;

    pub fn new(memory: SharedMemory) -> PPU {
        PPU {
            memory,
            mode: 2,
            mode_clock: 0,
            line: 0,
            framebuffer: [0; 160 * 144],
            frame_ready: false,
        }
    }

    fn read_byte(&self, addr: u16) -> u8 {
        let byte = self.memory.borrow().read_byte(addr);
        byte
    }

    fn write_byte(&self, addr: u16, value: u8) {
        self.memory.borrow_mut().write_byte(addr, value);
    }

    fn add_interrupt(&mut self, interrupt_bit: u8) {
        let interrupt_flags = self.read_byte(0xFF0F);
        self.write_byte(0xFF0F, interrupt_bit | interrupt_flags);
    }

    pub fn step(&mut self, cycles: u32) {
        self.mode_clock += cycles;

        match self.mode {
            0 => {
                if self.mode_clock >= 204 {
                    self.mode_clock -= 204;
                    self.line += 1;
                    self.write_byte(Self::LY_ADDR, self.line);
                    if self.line == 144 {
                        self.mode = 1;
                        self.frame_ready = true;
                        self.add_interrupt(0x01);
                    } else {
                        self.mode = 2;
                    }
                }
            }
            1 => {
                if self.mode_clock >= 456 {
                    self.mode_clock -= 456;
                    self.line += 1;
                    self.write_byte(Self::LY_ADDR, self.line);
                    if self.line > 153 {
                        self.line = 0;
                        self.write_byte(Self::LY_ADDR, self.line);
                        self.mode = 2;
                    }
                }
            }
            2 => {
                if self.mode_clock >= 80 {
                    self.mode_clock -= 80;
                    self.mode = 3;
                }
            }
            3 => {
                if self.mode_clock >= 172 {
                    self.mode_clock -= 172;
                    self.mode = 0;
                    self.render_scanline();
                }
            }
            _ => (),
        }

        self.update_stat();
    }

    fn update_stat(&mut self) {
        let mut stat = self.read_byte(Self::STAT_ADDR);

        stat = (stat & 0xFC) | (self.mode & 0x03);

        let lyc = self.read_byte(Self::LYC_ADDR);
        if self.line == lyc {
            stat |= 0x04;
            if stat & 0x40 != 0 {
                self.add_interrupt(0x02);
            }
        } else {
            stat &= !0x04;
        }

        self.write_byte(0xFF41, stat);
    }

    fn render_scanline(&mut self) {
        let lcdc = self.read_byte(Self::LCDC_ADDR);

        if lcdc & 0x01 != 0 {
            self.render_background();
        }
        if lcdc & 0x20 != 0 {
            self.render_window();
        }
        if lcdc & 0x02 != 0 {
            self.render_sprites();
        }
    }

    fn render_window(&mut self) {
        let lcdc = self.read_byte(Self::LCDC_ADDR);
        let wx = self.read_byte(Self::WX_ADDR).wrapping_sub(7);
        let wy = self.read_byte(Self::WY_ADDR);
        let ly = self.line;
        let bgp = self.read_byte(Self::BGP_ADDR);

        if ly >= wy {
            let window_map_addr = if lcdc & 0x40 != 0 { 0x9C00 } else { 0x9800 };
            let tile_data_addr = if lcdc & 0x10 != 0 { 0x8000 } else { 0x8800 };

            let y_pos = ly.wrapping_sub(wy);
            let tile_row = (y_pos as u16 / 8) * 32;

            for x in 0..160 {
                if x < wx {
                    continue;
                }
                let x_pos = x.wrapping_sub(wx);
                let tile_col = (x_pos / 8) as u16;
                let tile_index_addr = window_map_addr + tile_row + tile_col;
                let tile_number = self.read_byte(tile_index_addr);

                let tile_addr = if lcdc & 0x10 != 0 {
                    tile_data_addr + (tile_number as u16 * 16)
                } else {
                    tile_data_addr + ((tile_number as i8 as i16 + 128) as u16 * 16)
                };

                let line = (y_pos % 8) as u16 * 2;
                let data1 = self.read_byte(tile_addr + line);
                let data2 = self.read_byte(tile_addr + line + 1);

                let color_bit = 7 - (x_pos % 8);
                let color_num = (((data2 >> color_bit) & 1) << 1) | ((data1 >> color_bit) & 1);

                let color = self.get_color(color_num, bgp);

                let index = ly as usize * 160 + x as usize;
                self.framebuffer[index] = color;
            }
        }
    }

    fn render_sprites(&mut self) {
        let lcdc = self.read_byte(Self::LCDC_ADDR);
        let sprite_height = if lcdc & 0x04 != 0 { 16 } else { 8 };
        let obp0 = self.read_byte(Self::OBP0_ADDR);
        let obp1 = self.read_byte(Self::OBP1_ADDR);
        let ly = self.line;

        let mut sprites = Vec::new();

        for i in 0..40 {
            let sprite_addr = 0xFE00 + i * 4;
            let y = self.read_byte(sprite_addr) as i16 - 16;
            let x = self.read_byte(sprite_addr + 1) as i16 - 8;
            let tile_number = self.read_byte(sprite_addr + 2);
            let attributes = self.read_byte(sprite_addr + 3);

            if ly as i16 >= y && (ly as i16) < (y + sprite_height) {
                sprites.push((x, y, tile_number, attributes));
                if sprites.len() >= 10 {
                    break;
                }
            }
        }

        sprites.sort_by_key(|&(x, _, _, _)| x);

        for (x, y, mut tile_number, attributes) in sprites {
            let palette = if attributes & 0x10 != 0 { obp1 } else { obp0 };
            let flip_x = attributes & 0x20 != 0;
            let flip_y = attributes & 0x40 != 0;
            let priority = attributes & 0x80 != 0;

            let sprite_line = ly as i16 - y;

            let line = if flip_y {
                sprite_height - 1 - sprite_line
            } else {
                sprite_line
            } as u16;

            if sprite_height == 16 {
                tile_number &= 0xFE;
            }

            let base_tile_addr = 0x8000 + (tile_number as u16 * 16);

            let (tile_addr, line) = if sprite_height == 16 && line >= 8 {
                (base_tile_addr + 16, line - 8)
            } else {
                (base_tile_addr, line)
            };

            let data1 = self.read_byte(tile_addr + line * 2);
            let data2 = self.read_byte(tile_addr + line * 2 + 1);

            for pixel in 0..8 {
                let pixel_i16 = pixel as i16;

                let x_pos = if flip_x {
                    x + 7 - pixel_i16
                } else {
                    x + pixel_i16
                };

                if x_pos < 0 || x_pos >= 160 {
                    continue;
                }

                let color_bit = if flip_x { pixel } else { 7 - pixel } as u8;

                let color_num = (((data2 >> color_bit) & 1) << 1) | ((data1 >> color_bit) & 1);
                if color_num == 0 {
                    continue;
                }

                let color = self.get_color(color_num, palette);

                let index = ly as usize * 160 + x_pos as usize;

                if priority && self.framebuffer[index] != 0xFFFFFFFF {
                    continue;
                }

                self.framebuffer[index] = color;
            }
        }
    }

    fn render_background(&mut self) {
        let scx = self.read_byte(Self::SCX_ADDR);
        let scy = self.read_byte(Self::SCY_ADDR);
        let ly = self.line;
        let lcdc = self.read_byte(Self::LCDC_ADDR);
        let bgp = self.read_byte(Self::BGP_ADDR);

        let bg_map_addr = if lcdc & 0x08 != 0 { 0x9C00 } else { 0x9800 };
        let tile_data_addr = if lcdc & 0x10 != 0 { 0x8000 } else { 0x8800 };

        let y_pos = scy.wrapping_add(ly);
        let tile_row = (y_pos as u16 / 8) * 32;

        for x in 0..160 {
            let x_pos = scx.wrapping_add(x as u8);
            let tile_col = (x_pos / 8) as u16;
            let tile_index_addr = bg_map_addr + tile_row + tile_col;
            let tile_number = self.read_byte(tile_index_addr);

            let tile_addr = if lcdc & 0x10 != 0 {
                tile_data_addr + (tile_number as u16 * 16)
            } else {
                tile_data_addr + ((tile_number as i8 as i16 + 128) as u16 * 16)
            };

            let line = (y_pos % 8) as u16 * 2;
            let data1 = self.read_byte(tile_addr + line);
            let data2 = self.read_byte(tile_addr + line + 1);

            let color_bit = 7 - (x_pos % 8);
            let color_num = (((data2 >> color_bit) & 1) << 1) | ((data1 >> color_bit) & 1);

            let color = self.get_color(color_num, bgp);

            let index = ly as usize * 160 + x;
            self.framebuffer[index] = color;
        }
    }
}
