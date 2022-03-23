use crate::addressable::*;

pub struct Joypad {
    button_a: bool,
    button_b: bool,
    select: bool,
    start: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Readable for Joypad {}
impl Writable for Joypad {}
impl Addressable for Joypad {}
