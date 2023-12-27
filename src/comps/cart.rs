use std::{fs::File, os::unix::fs::MetadataExt, io::Read, sync::RwLock};

pub struct ROMHeader {
    _entry: [u8; 4],
    _logo: [u8; 0x30],

    title: [char; 16],
    _new_lic_code: u16,
    _sgb_flag: u8,
    type_: u8,
    rom_size: u8,
    ram_size: u8,
    _dest_code: u8,
    lic_code: u8,
    version: u8,
    checksum: u8,
    _global_checksum: u16,
}

impl ROMHeader {
    pub fn from(rom_data: &Vec<u8>) -> Self {
        ROMHeader {
            _entry: rom_data[0x100..=0x103].try_into().unwrap(),
            _logo: rom_data[0x104..=0x133].try_into().unwrap(),
            title: rom_data[0x134..=0x143].into_iter().map(|val| *val as char).collect::<Vec<char>>().try_into().unwrap(),
            _new_lic_code: (rom_data[0x144] as u16) << 8 | (rom_data[0x145] as u16),
            _sgb_flag: rom_data[0x146],
            type_: rom_data[0x147],
            rom_size: rom_data[0x148],
            ram_size: rom_data[0x149],
            _dest_code: rom_data[0x14a],
            lic_code: rom_data[0x14b],
            version: rom_data[0x14c],
            checksum: rom_data[0x14d],
            _global_checksum: (rom_data[0x14e] as u16) << 8 | (rom_data[0x14f] as u16),
        }
    }
}

pub struct CartContext {
    _filename: [char; 1024],
    rom_size: u32,
    pub rom_data: Vec<u8>,
    _header: ROMHeader
}

pub static CART: RwLock<CartContext> = RwLock::new(CartContext {
    _filename: [' '; 1024],
    rom_size: 0,
    rom_data: vec![],
    _header: ROMHeader {
        _entry: [0; 4],
        _logo: [0; 48],
        title: [' '; 16],
        _new_lic_code: 0,
        _sgb_flag: 0,
        type_: 0,
        rom_size: 0,
        ram_size: 0,
        _dest_code: 0,
        lic_code: 0,
        version: 0,
        checksum: 0,
        _global_checksum: 0
    }
});

impl CartContext {
    pub fn load(&mut self, args: Vec<String>) {
        if args.len() < 2 {
            panic!("Usage: cargo run <rom_file> \n");
        }

        let filename = &args[1];

        // Open the file
        println!("Filename: {filename}");
        let mut file = File::open(format!("/Users/lassegrosbol-rais/Desktop/gbemu/roms/{filename}")).unwrap();
        println!("Opened: {filename}");

        // Extract the data
        self.rom_size = file.metadata().unwrap().size() as u32;
        self.rom_data = Vec::with_capacity(self.rom_size as usize);
        file.read_to_end(&mut self.rom_data).unwrap();

        // Create ROMHeader from data
        let header = ROMHeader::from(&self.rom_data);

        // Display data
        println!("Cartridge Loaded:");
        println!("\t Title    : {}", header.title.iter().collect::<String>());
        println!("\t Type     : {:02X} ({})", header.type_, ROM_TYPES[header.type_ as usize]);
        println!("\t ROM Size : {} KB", 32 << header.rom_size);
        println!("\t RAM Size : {:02X}", header.ram_size);
        println!("\t LIC Code : {:02X} ({})", header.lic_code, lic_code(header.lic_code));
        println!("\t ROM Vers : {:02X}", header.version);

        // Validate checksum
        let mut x: u16 = 0;

        for i in 0x134..=0x14C {
            x = x.wrapping_sub(self.rom_data[i] as u16).wrapping_sub(1);
        }

        println!("\t Checksum : {:02X} ({})", header.checksum, if x as u8 != 0 {"PASSED"} else {"FAILED"});

        // Convert filename from &str to [char; 1024]
        let chars = filename.chars();
        let mut filename = [' '; 1024];

        for (i, c) in chars.enumerate() {
            filename[i] = c;
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.rom_data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.rom_data[address as usize] = value;
    }
}

const ROM_TYPES: [&str; 35] = [
    "ROM ONLY",
    "MBC1",
    "MBC1+RAM",
    "MBC1+RAM+BATTERY",
    "0x04 ???",
    "MBC2",
    "MBC2+BATTERY",
    "0x07 ???",
    "ROM+RAM 1",
    "ROM+RAM+BATTERY 1",
    "0x0A ???",
    "MMM01",
    "MMM01+RAM",
    "MMM01+RAM+BATTERY",
    "0x0E ???",
    "MBC3+TIMER+BATTERY",
    "MBC3+TIMER+RAM+BATTERY 2",
    "MBC3",
    "MBC3+RAM 2",
    "MBC3+RAM+BATTERY 2",
    "0x14 ???",
    "0x15 ???",
    "0x16 ???",
    "0x17 ???",
    "0x18 ???",
    "MBC5",
    "MBC5+RAM",
    "MBC5+RAM+BATTERY",
    "MBC5+RUMBLE",
    "MBC5+RUMBLE+RAM",
    "MBC5+RUMBLE+RAM+BATTERY",
    "0x1F ???",
    "MBC6",
    "0x21 ???",
    "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
];

fn lic_code(code: u8) -> &'static str {
    return match code {
        0x00 => "None",
        0x01 => "Nintendo R&D1",
        0x08 => "Capcom",
        0x13 => "Electronic Arts",
        0x18 => "Hudson Soft",
        0x19 => "b-ai",
        0x20 => "kss",
        0x22 => "pow",
        0x24 => "PCM Complete",
        0x25 => "san-x",
        0x28 => "Kemco Japan",
        0x29 => "seta",
        0x30 => "Viacom",
        0x31 => "Nintendo",
        0x32 => "Bandai",
        0x33 => "Ocean/Acclaim",
        0x34 => "Konami",
        0x35 => "Hector",
        0x37 => "Taito",
        0x38 => "Hudson",
        0x39 => "Banpresto",
        0x41 => "Ubi Soft",
        0x42 => "Atlus",
        0x44 => "Malibu",
        0x46 => "angel",
        0x47 => "Bullet-Proof",
        0x49 => "irem",
        0x50 => "Absolute",
        0x51 => "Acclaim",
        0x52 => "Activision",
        0x53 => "American sammy",
        0x54 => "Konami",
        0x55 => "Hi tech entertainment",
        0x56 => "LJN",
        0x57 => "Matchbox",
        0x58 => "Mattel",
        0x59 => "Milton Bradley",
        0x60 => "Titus",
        0x61 => "Virgin",
        0x64 => "LucasArts",
        0x67 => "Ocean",
        0x69 => "Electronic Arts",
        0x70 => "Infogrames",
        0x71 => "Interplay",
        0x72 => "Broderbund",
        0x73 => "sculptured",
        0x75 => "sci",
        0x78 => "THQ",
        0x79 => "Accolade",
        0x80 => "misawa",
        0x83 => "lozc",
        0x86 => "Tokuma Shoten Intermedia",
        0x87 => "Tsukuda Original",
        0x91 => "Chunsoft",
        0x92 => "Video system",
        0x93 => "Ocean/Acclaim",
        0x95 => "Varie",
        0x96 => "Yonezawa/s'pal",
        0x97 => "Kaneko",
        0x99 => "Pack in soft",
        0xA4 => "Konami (Yu-Gi-Oh!)",
        _ => "UNKNOWN"
    }
}
