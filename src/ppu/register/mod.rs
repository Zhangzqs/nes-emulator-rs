use self::{
    address::AddressRegister, control::ControlRegister, mask::MaskRegister, scroll::ScrollRegister,
    status::StatusRegister,
};

pub mod address;
pub mod control;
pub mod mask;
pub mod scroll;
pub mod status;

pub struct PpuRegister {
    pub control: ControlRegister,
    pub mask: MaskRegister,
    pub status: StatusRegister,
    pub scroll: ScrollRegister,
    pub address: AddressRegister,
}

impl PpuRegister {
    pub fn new() -> Self {
        Self {
            control: ControlRegister::new(),
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
            scroll: ScrollRegister::new(),
            address: AddressRegister::new(),
        }
    }
}
