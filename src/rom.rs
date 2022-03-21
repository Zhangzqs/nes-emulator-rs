use crate::addressable::Addressable;

#[derive(Debug, PartialEq)]
pub enum Mirror {
    Vertical,
    Horizontal,
    FourScreen,
}
pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub mirror: Mirror,
    pub has_battery_backed: bool,
}

impl Addressable for Rom {
    fn read(&self, mut addr: u16) -> u8 {
        if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr = addr % 0x4000;
        }
        let data = self.prg_rom[addr as usize];
        data
    }

    fn write(&mut self, addr: u16, data: u8) {
        println!("Rom is a read-only device")
    }
}

const PRG_ROM_PAGE_SIZE: usize = 0x4000;
const CHR_ROM_PAGE_SIZE: usize = 0x2000;
impl Rom {
    pub fn new(data: &[u8]) -> Result<Rom, String> {
        if let Some(error) = check_rom(data) {
            return Err(error);
        }
        let prg_rom_size = data[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = data[5] as usize * CHR_ROM_PAGE_SIZE;

        let mapper_l = data[6] >> 4;
        let mapper_h = data[7] >> 4;
        let mapper = mapper_h << 4 | mapper_l;

        let is_vertical_mirror = data[6] & 0b0001 != 0;
        let has_battery_backed = data[6] & 0b0010 != 0;
        let has_trainer = data[6] & 0b0100 != 0;
        let four_screen = data[6] & 0b1000 != 0;
        let mirror = if four_screen {
            Mirror::FourScreen
        } else {
            if is_vertical_mirror {
                Mirror::Vertical
            } else {
                Mirror::Horizontal
            }
        };
        let prg_rom_start = 16 + if has_trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;
        Ok(Self {
            prg_rom: data[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
            chr_rom: data[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            mapper,
            mirror,
            has_battery_backed,
        })
    }
}

const NES_HEADER: [u8; 4] = [78, 69, 83, 26];
fn check_rom(data: &[u8]) -> Option<String> {
    let header: [u8; 4] = NES_HEADER;
    if &data[0..4] != &header {
        return Some("Invalid nes file".to_string());
    }
    if data[7] & 0b1100 == 0b1000 {
        return Some("NES2.0 format is not supported".to_string());
    }
    None
}

pub mod test {

    use super::*;

    struct TestRom {
        header: Vec<u8>,
        trainer: Option<Vec<u8>>,
        pgp_rom: Vec<u8>,
        chr_rom: Vec<u8>,
    }

    fn create_rom(rom: TestRom) -> Vec<u8> {
        let mut result = Vec::with_capacity(
            rom.header.len()
                + rom.trainer.as_ref().map_or(0, |t| t.len())
                + rom.pgp_rom.len()
                + rom.chr_rom.len(),
        );

        result.extend(&rom.header);
        if let Some(t) = rom.trainer {
            result.extend(t);
        }
        result.extend(&rom.pgp_rom);
        result.extend(&rom.chr_rom);

        result
    }

    pub fn test_rom() -> Rom {
        let test_rom = create_rom(TestRom {
            header: vec![
                0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 00, 00, 00, 00, 00, 00, 00, 00, 00,
            ],
            trainer: None,
            pgp_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
            chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
        });

        Rom::new(&test_rom).unwrap()
    }

    #[test]
    fn test() {
        let test_rom = create_rom(TestRom {
            header: vec![
                0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 00, 00, 00, 00, 00, 00, 00, 00, 00,
            ],
            trainer: None,
            pgp_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
            chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
        });

        let rom: Rom = Rom::new(&test_rom).unwrap();

        assert_eq!(rom.chr_rom, vec!(2; 1 * CHR_ROM_PAGE_SIZE));
        assert_eq!(rom.prg_rom, vec!(1; 2 * PRG_ROM_PAGE_SIZE));
        assert_eq!(rom.mapper, 3);
        assert_eq!(rom.mirror, Mirror::Vertical);
    }

    #[test]
    fn test_with_trainer() {
        let test_rom = create_rom(TestRom {
            header: vec![
                0x4E,
                0x45,
                0x53,
                0x1A,
                0x02,
                0x01,
                0x31 | 0b100,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
            ],
            trainer: Some(vec![0; 512]),
            pgp_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
            chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
        });

        let rom: Rom = Rom::new(&test_rom).unwrap();

        assert_eq!(rom.chr_rom, vec!(2; 1 * CHR_ROM_PAGE_SIZE));
        assert_eq!(rom.prg_rom, vec!(1; 2 * PRG_ROM_PAGE_SIZE));
        assert_eq!(rom.mapper, 3);
        assert_eq!(rom.mirror, Mirror::Vertical);
    }

    #[test]
    fn test_nes2_is_not_supported() {
        let test_rom = create_rom(TestRom {
            header: vec![
                0x4E, 0x45, 0x53, 0x1A, 0x01, 0x01, 0x31, 0x8, 00, 00, 00, 00, 00, 00, 00, 00,
            ],
            trainer: None,
            pgp_rom: vec![1; 1 * PRG_ROM_PAGE_SIZE],
            chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
        });
        let rom = Rom::new(&test_rom);
        match rom {
            Result::Ok(_) => assert!(false, "should not load rom"),
            Result::Err(str) => assert_eq!(str, "NES2.0 format is not supported"),
        }
    }
}
