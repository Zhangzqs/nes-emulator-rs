#[derive(Debug)]
pub struct Asm {
    code: Vec<u8>,
}

impl Asm {
    fn new() -> Self {
        Self { code: vec![] }
    }

    fn dd(&mut self, data: &[u8]) -> u16 {
        let start_addr = self.code.len();
        for byte in data {
            self.code.push(*byte);
        }

        start_addr as u16
    }
}
#[test]
fn test_dd() {
    let mut asm = Asm::new();
    let addr = asm.dd(&[0x12, 0x21, 0x32]);
    assert_eq!(addr, 0);
    assert_eq!(asm.code, vec![0x12, 0x21, 0x32]);
    let addr = asm.dd(&[0x13, 0x22, 0x33]);
    assert_eq!(asm.code, vec![0x12, 0x21, 0x32, 0x13, 0x22, 0x33]);
}
