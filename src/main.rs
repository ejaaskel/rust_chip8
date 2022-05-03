
use std::io::BufReader;
use std::io::Read;
use std::fs::File;

fn read_rom() -> Vec<u8> {
    let rom = File::open("src/pong.ch8").expect("Read failed!");
    let mut rom_reader = BufReader::new(rom);
    let mut rom_buffer = Vec::new();

    // Read file into vector.
    rom_reader.read_to_end(&mut rom_buffer).expect("Read to end failed!");
    return rom_buffer;
}

fn main() {
    let mut opcode:u16 = 0;

    let memory:[u8;4096];

    let V:[u8;16];

    let I:u16;
    let pc:u16;

    let gfx:[u8;64 * 32];

    let delay_timer:u8;
    let sound_timer:u8;

    let stack:u16;
    let sp:u16;

    let key:[u8;16];

    let mut rom_buffer = read_rom();

    let mut rom_reader_i = 0;

    for value in rom_buffer {
        if rom_reader_i % 2 == 0 {
            opcode = 0;
            opcode = (value as u16) << 8;
        }
        else {
            opcode += value as u16;
            println!("SHORT: {}", opcode);
        }
        rom_reader_i = rom_reader_i + 1;
    }
}
