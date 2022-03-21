def code_gen(struct_name, fields):
    return f"""
pub struct {struct_name} {'{'}
    {fields[0]}: bool,
    {fields[1]}: bool,
    {fields[2]}: bool,
    {fields[3]}: bool,
    {fields[4]}: bool,
    {fields[5]}: bool,
    {fields[6]}: bool,
    {fields[7]}: bool,
{'}'}

impl From<u8> for {struct_name} {'{'}
    fn from(bits: u8) -> Self {'{'}
        Self {'{'}
            {fields[0]}: ((bits >> 0) & 1) != 0,
            {fields[1]}: ((bits >> 1) & 1) != 0,
            {fields[2]}: ((bits >> 2) & 1) != 0,
            {fields[3]}: ((bits >> 3) & 1) != 0,
            {fields[4]}: ((bits >> 4) & 1) != 0,
            {fields[5]}: ((bits >> 5) & 1) != 0,
            {fields[6]}: ((bits >> 6) & 1) != 0,
            {fields[7]}: ((bits >> 7) & 1) != 0,
        {'}'}
    {'}'}
{'}'}

impl Into<u8> for {struct_name} {'{'}
    fn into(self) -> u8 {'{'}
        let bit0 = self.{fields[0]} as u8;
        let bit1 = self.{fields[1]} as u8;
        let bit2 = self.{fields[2]} as u8;
        let bit3 = self.{fields[3]} as u8;
        let bit4 = self.{fields[4]} as u8;
        let bit5 = self.{fields[5]} as u8;
        let bit6 = self.{fields[6]} as u8;
        let bit7 = self.{fields[7]} as u8;
        let bits = [bit0, bit1, bit2, bit3, bit4, bit5, bit6, bit7];
        let mut result: u8 = 0;
        for (i, bit) in bits.iter().enumerate() {'{'}
            result = result | (bit << i);
        {'}'}
        result
    {'}'}
{'}'}

impl FlagRegister for {struct_name} {'{'}
    fn update(&mut self, data: u8) {'{'}
        self.{fields[0]} = ((data >> 0) & 1) != 0;
        self.{fields[1]} = ((data >> 1) & 1) != 0;
        self.{fields[2]} = ((data >> 2) & 1) != 0;
        self.{fields[3]} = ((data >> 3) & 1) != 0;
        self.{fields[4]} = ((data >> 4) & 1) != 0;
        self.{fields[5]} = ((data >> 5) & 1) != 0;
        self.{fields[6]} = ((data >> 6) & 1) != 0;
        self.{fields[7]} = ((data >> 7) & 1) != 0;
    {'}'}
{'}'}

"""


with open('flag/control.rs', 'w') as f:
    f.write(code_gen(
        struct_name="ControlRegister",
        fields=[
            'nametable_1',
            'nametable_2',
            'vram_address_increment',
            'sprite_pattern_address',
            'background_pattern_address',
            'sprite_size',
            'master_slave_select',
            'generate_vblank_nmi',
        ],
    ))

with open('flag/mask.rs', 'w') as f:
    f.write(code_gen(
        struct_name="MaskRegister",
        fields=[
            'is_grey_scale',
            'leftmost_8pxl_background',
            'leftmost_8pxl_sprite',
            'show_background',
            'show_sprite',
            'emphasise_red',
            'emphasise_green',
            'emphasise_blue',
        ],
    ))

with open('flag/status.rs', 'w') as f:
    f.write(code_gen(
        struct_name="StatusRegister",
        fields=[
            'unused_1',
            'unused_2',
            'unused_3',
            'unused_4',
            'unused_5',
            'sprite_overflow',
            'sprite_zero_hit',
            'vblank_started',
        ],
    ))
