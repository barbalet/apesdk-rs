#![allow(non_camel_case_types)]

//! C-compatible toolkit foundation for the ApeSDK Rust port.

use std::os::raw::{c_char, c_int, c_long, c_ulong};

pub type n_double = f64;
pub type n_char = c_char;
pub type n_string = *mut n_char;
pub type n_constant_string = *const n_char;
pub type n_byte = u8;
pub type n_byte2 = u16;
pub type n_byte4 = u32;
pub type n_c_int = c_int;
pub type n_int = c_long;
pub type n_uint = c_ulong;
pub type n_audio = i16;

pub const CHAR_SPACE: n_byte = 32;
pub const STRING_BLOCK_SIZE: usize = 4096;
pub const PACKED_DATA_BLOCK: usize = 32 * 32 * 32 * 2;
pub const TWO_PI: n_double = 6.2831853071795864769252867665590057683943;
pub const SINE_MAXIMUM: n_int = 26880;
pub const BIG_INTEGER: n_int = 2_147_483_647;
pub const BIG_NEGATIVE_INTEGER: n_int = -2_147_483_648;

pub type n_string_block = [n_char; STRING_BLOCK_SIZE];

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum file_element_type {
    FILE_TYPE_BYTE = 0x01,
    FILE_TYPE_BYTE2 = 0x02,
    FILE_TYPE_BYTE_EXT = 0x03,
    FILE_TYPE_PACKED = 0x05,
    FILE_TYPE_BYTE4 = 0x06,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct n_vect2 {
    pub x: n_int,
    pub y: n_int,
}

impl n_vect2 {
    pub const fn new(x: n_int, y: n_int) -> Self {
        Self { x, y }
    }

    pub const fn data(&self) -> [n_int; 2] {
        [self.x, self.y]
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct n_area2 {
    pub top_left: n_vect2,
    pub bottom_right: n_vect2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct n_quad {
    pub four: [n_vect2; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct n_line {
    pub two: [n_vect2; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct n_vect3 {
    pub x: n_double,
    pub y: n_double,
    pub z: n_double,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct n_rgba {
    pub a: n_byte,
    pub r: n_byte,
    pub g: n_byte,
    pub b: n_byte,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union n_rgba32 {
    pub rgba: n_rgba,
    pub thirtytwo: n_byte4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct n_spacetime {
    pub date: n_byte4,
    pub location: [n_byte2; 2],
    pub time: n_byte4,
}

#[repr(C)]
#[derive(Debug)]
pub struct n_file {
    pub size: n_uint,
    pub location: n_uint,
    pub data: *mut n_byte,
}

pub type n_console_input = extern "C" fn(value: n_string, length: n_int) -> n_string;
pub type n_console_output = extern "C" fn(value: n_constant_string);
pub type n_console = extern "C" fn(
    ptr: *mut std::ffi::c_void,
    response: n_string,
    output_function: n_console_output,
) -> n_int;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct simulated_console_command {
    pub function: Option<n_console>,
    pub command: n_string,
    pub addition: n_string,
    pub help_information: n_string,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryList {
    unit_size: usize,
    max: usize,
    count: usize,
    data: Vec<n_byte>,
}

impl MemoryList {
    pub fn new(unit_size: usize, number: usize) -> Option<Self> {
        if unit_size == 0 {
            return None;
        }

        Some(Self {
            unit_size,
            max: number,
            count: 0,
            data: vec![0; unit_size.saturating_mul(number)],
        })
    }

    pub const fn count(&self) -> usize {
        self.count
    }

    pub const fn max(&self) -> usize {
        self.max
    }

    pub const fn unit_size(&self) -> usize {
        self.unit_size
    }

    pub fn data(&self) -> &[n_byte] {
        &self.data[..self.count.saturating_mul(self.unit_size)]
    }

    pub fn copy_units(&mut self, data: &[n_byte]) -> Result<(), &'static str> {
        if data.len() % self.unit_size != 0 {
            return Err("wrong base unit size");
        }

        let units = data.len() / self.unit_size;
        let new_count = self.count + units;
        while new_count > self.max {
            self.grow();
        }

        let start = self.count * self.unit_size;
        let end = start + data.len();
        if end > self.data.len() {
            self.data.resize(end, 0);
        }
        self.data[start..end].copy_from_slice(data);
        self.count = new_count;

        if self.count >= self.max {
            self.grow();
        }

        Ok(())
    }

    fn grow(&mut self) {
        let new_max = if self.max == 0 { 1 } else { self.max * 2 };
        self.max = new_max;
        self.data.resize(new_max * self.unit_size, 0);
    }
}

pub fn memory_copy(from: &[n_byte], to: &mut [n_byte], number: n_uint) {
    let number = number as usize;
    to[..number].copy_from_slice(&from[..number]);
}

pub fn memory_erase(buffer: &mut [n_byte]) {
    buffer.fill(0);
}

pub fn io_lower_bytes(value: &mut [n_byte]) {
    for character in value {
        if character.is_ascii_uppercase() {
            *character += b'a' - b'A';
        }
    }
}

pub fn io_length_bytes(value: &[n_byte], max: n_int) -> n_int {
    if max < 1 {
        return -1;
    }

    let max = max as usize;
    for index in 0..=max {
        if index >= value.len() || value[index] == 0 {
            return index as n_int;
        }
    }

    max as n_int
}

pub fn io_find_bytes(
    check: &[n_byte],
    from: n_int,
    max: n_int,
    value_find: &[n_byte],
    value_find_length: n_int,
) -> n_int {
    if from < 0 || max < from || value_find_length < 1 {
        return -1;
    }

    let mut loop_index = from as usize;
    let max = max as usize;
    let value_find_length = value_find_length as usize;
    let mut check_length = 0usize;

    while loop_index < max && loop_index < check.len() {
        if check[loop_index] == value_find[check_length] {
            check_length += 1;
            if check_length == value_find_length {
                return (loop_index + 1) as n_int;
            }
        } else {
            check_length = 0;
        }
        loop_index += 1;
    }

    -1
}

pub fn io_string_write_bytes(dest: &mut [n_byte], insert: &[n_byte], pos: &mut n_int) {
    for character in insert.iter().copied() {
        if character == 0 {
            break;
        }
        dest[*pos as usize] = character;
        *pos += 1;
    }
    dest[*pos as usize] = 0;
}

const NEW_SD: [n_int; 256] = [
    0, 659, 1318, 1977, 2634, 3290, 3944, 4595, 5244, 5889, 6531, 7169, 7802, 8431, 9055, 9673,
    10286, 10892, 11492, 12085, 12671, 13249, 13819, 14380, 14933, 15477, 16012, 16537, 17052,
    17557, 18051, 18534, 19007, 19467, 19916, 20353, 20778, 21190, 21590, 21976, 22349, 22709,
    23055, 23387, 23706, 24009, 24299, 24573, 24833, 25078, 25308, 25523, 25722, 25906, 26074,
    26226, 26363, 26484, 26589, 26677, 26750, 26807, 26847, 26871, 26880, 26871, 26847, 26807,
    26750, 26677, 26589, 26484, 26363, 26226, 26074, 25906, 25722, 25523, 25308, 25078, 24833,
    24573, 24299, 24009, 23706, 23387, 23055, 22709, 22349, 21976, 21590, 21190, 20778, 20353,
    19916, 19467, 19007, 18534, 18051, 17557, 17052, 16537, 16012, 15477, 14933, 14380, 13819,
    13249, 12671, 12085, 11492, 10892, 10286, 9673, 9055, 8431, 7802, 7169, 6531, 5889, 5244, 4595,
    3944, 3290, 2634, 1977, 1318, 659, 0, -659, -1318, -1977, -2634, -3290, -3944, -4595, -5244,
    -5889, -6531, -7169, -7802, -8431, -9055, -9673, -10286, -10892, -11492, -12085, -12671,
    -13249, -13819, -14380, -14933, -15477, -16012, -16537, -17052, -17557, -18051, -18534, -19007,
    -19467, -19916, -20353, -20778, -21190, -21590, -21976, -22349, -22709, -23055, -23387, -23706,
    -24009, -24299, -24573, -24833, -25078, -25308, -25523, -25722, -25906, -26074, -26226, -26363,
    -26484, -26589, -26677, -26750, -26807, -26847, -26871, -26880, -26871, -26847, -26807, -26750,
    -26677, -26589, -26484, -26363, -26226, -26074, -25906, -25722, -25523, -25308, -25078, -24833,
    -24573, -24299, -24009, -23706, -23387, -23055, -22709, -22349, -21976, -21590, -21190, -20778,
    -20353, -19916, -19467, -19007, -18534, -18051, -17557, -17052, -16537, -16012, -15477, -14933,
    -14380, -13819, -13249, -12671, -12085, -11492, -10892, -10286, -9673, -9055, -8431, -7802,
    -7169, -6531, -5889, -5244, -4595, -3944, -3290, -2634, -1977, -1318, -659,
];

pub fn area2_add(area: &mut n_area2, vect: n_vect2, first: bool) {
    if first {
        area.top_left = vect;
        area.bottom_right = vect;
        return;
    }

    if vect.x < area.top_left.x {
        area.top_left.x = vect.x;
    }
    if vect.y < area.top_left.y {
        area.top_left.y = vect.y;
    }
    if vect.x > area.bottom_right.x {
        area.bottom_right.x = vect.x;
    }
    if vect.y > area.bottom_right.y {
        area.bottom_right.y = vect.y;
    }
}

pub fn vect2_byte2(input: [n_byte2; 2]) -> n_vect2 {
    n_vect2::new(input[0] as n_int, input[1] as n_int)
}

pub fn vect2_add(initial: n_vect2, second: n_vect2) -> n_vect2 {
    n_vect2::new(initial.x + second.x, initial.y + second.y)
}

pub fn vect2_center(initial: n_vect2, second: n_vect2) -> n_vect2 {
    let sum = vect2_add(initial, second);
    n_vect2::new(sum.x / 2, sum.y / 2)
}

pub fn vect2_subtract(initial: n_vect2, second: n_vect2) -> n_vect2 {
    n_vect2::new(initial.x - second.x, initial.y - second.y)
}

pub fn vect2_divide(initial: n_vect2, second: n_vect2, divisor: n_int) -> Option<n_vect2> {
    if divisor == 0 {
        return None;
    }

    let difference = vect2_subtract(second, initial);
    Some(n_vect2::new(difference.x / divisor, difference.y / divisor))
}

pub fn vect2_multiplier(
    initial: n_vect2,
    second: n_vect2,
    multiplier: n_int,
    divisor: n_int,
) -> Option<n_vect2> {
    if divisor == 0 {
        return None;
    }

    Some(n_vect2::new(
        (multiplier * initial.x * second.x) / divisor,
        (multiplier * initial.y * second.y) / divisor,
    ))
}

pub fn vect2_d(initial: &mut n_vect2, second: n_vect2, multiplier: n_int, divisor: n_int) -> bool {
    if divisor == 0 {
        return false;
    }

    initial.x += (multiplier * second.x) / divisor;
    initial.y += (multiplier * second.y) / divisor;
    true
}

pub fn vect2_dot(initial: n_vect2, second: n_vect2, multiplier: n_int, divisor: n_int) -> n_int {
    (multiplier * ((initial.x * second.x) + (initial.y * second.y))) / divisor
}

pub fn vect2_distance_under(first: n_vect2, second: n_vect2, distance: n_int) -> bool {
    let difference = vect2_subtract(first, second);
    let distance_squ = (difference.x * difference.x) + (difference.y * difference.y);
    (distance * distance) > distance_squ
}

pub fn math_sine(direction: n_int, divisor: n_int) -> n_int {
    NEW_SD[(direction as usize) & 255] / divisor
}

pub fn vect2_rotate90(rotation: &mut n_vect2) {
    let temp = rotation.y;
    rotation.y = -rotation.x;
    rotation.x = temp;
}

pub fn vect2_direction(direction: n_int, divisor: n_int) -> n_vect2 {
    n_vect2::new(
        NEW_SD[((direction + 64) as usize) & 255] / divisor,
        NEW_SD[(direction as usize) & 255] / divisor,
    )
}

pub fn vect2_delta(initial: &mut n_vect2, delta: n_vect2) {
    initial.x += delta.x;
    initial.y += delta.y;
}

pub fn vect2_offset(initial: &mut n_vect2, dx: n_int, dy: n_int) {
    initial.x += dx;
    initial.y += dy;
}

pub fn vect2_back_byte2(converter: &mut n_vect2) -> [n_byte2; 2] {
    converter.x = converter.x.clamp(0, u16::MAX as n_int);
    converter.y = converter.y.clamp(0, u16::MAX as n_int);
    [converter.x as n_byte2, converter.y as n_byte2]
}

pub fn vect2_copy(from: n_vect2) -> n_vect2 {
    from
}

pub fn vect2_populate(x: n_int, y: n_int) -> n_vect2 {
    n_vect2::new(x, y)
}

pub fn vect2_rotation(location: &mut n_vect2, rotation: n_vect2) {
    let temp = n_vect2::new(
        ((location.x * rotation.x) + (location.y * rotation.y)) / SINE_MAXIMUM,
        ((location.x * rotation.y) - (location.y * rotation.x)) / SINE_MAXIMUM,
    );
    *location = temp;
}

pub fn vect2_rotation_bitshift(location: &mut n_vect2, rotation: n_vect2) {
    let temp = n_vect2::new(
        ((location.x * rotation.x) + (location.y * rotation.y)) >> 15,
        ((location.x * rotation.y) - (location.y * rotation.x)) >> 15,
    );
    *location = temp;
}

pub fn vect2_nonzero(nonzero: n_vect2) -> bool {
    nonzero.x != 0 || nonzero.y != 0
}

pub fn vect2_min_max(points: &[n_vect2], maxmin: &mut [n_vect2; 2]) {
    for point in points {
        if point.x < maxmin[0].x {
            maxmin[0].x = point.x;
        }
        if point.y < maxmin[0].y {
            maxmin[0].y = point.y;
        }
        if point.x > maxmin[1].x {
            maxmin[1].x = point.x;
        }
        if point.y > maxmin[1].y {
            maxmin[1].y = point.y;
        }
    }
}

pub fn math_hash_fnv1(key: &[n_byte]) -> n_byte4 {
    let mut hash: n_byte4 = 2_166_136_261;
    for byte in key.iter().copied().take_while(|byte| *byte != 0) {
        hash = hash.wrapping_mul(16_777_619) ^ n_byte4::from(byte);
    }
    hash
}

pub fn math_random(seed: &mut [n_byte2; 2]) -> n_byte2 {
    let tmp0 = seed[0];
    let tmp1 = seed[1];

    seed[0] = tmp1;
    match tmp0 & 7 {
        0 => {
            seed[0] = tmp1 ^ (tmp0 >> 3) ^ 23_141;
        }
        3 => {
            seed[1] = tmp0 ^ (tmp1 >> 1) ^ 53_289;
        }
        5 => {
            seed[1] = tmp1 ^ (tmp0 >> 2) ^ 44_550;
        }
        _ => {
            seed[1] = tmp0 ^ (tmp1 >> 1);
        }
    }

    tmp1
}

pub fn math_random3(seed: &mut [n_byte2; 2]) {
    math_random(seed);
    math_random(seed);
    math_random(seed);
}

fn math_random3_at(seed: &mut [n_byte2; 5], offset: usize) {
    for _ in 0..3 {
        let mut pair = [seed[offset], seed[offset + 1]];
        math_random(&mut pair);
        seed[offset] = pair[0];
        seed[offset + 1] = pair[1];
    }
}

pub fn math_hash(values: &[n_byte]) -> n_uint {
    let mut round: [n_byte2; 5] = [0xfa78, 0xfad7, 0x53e7, 0xa728, 0x2c81];

    if std::mem::size_of::<n_uint>() == 8 {
        for value in values {
            round[0] ^= round[4];
            round[1] ^= n_byte2::from(*value);
            math_random3_at(&mut round, 0);
            math_random3_at(&mut round, 1);
            math_random3_at(&mut round, 2);
            math_random3_at(&mut round, 3);
        }

        return ((round[0] as u64)
            | ((round[1] as u64) << 16)
            | ((round[2] as u64) << 32)
            | ((round[3] as u64) << 48)) as n_uint;
    }

    for value in values {
        round[1] ^= n_byte2::from(*value);
        math_random3_at(&mut round, 0);
    }

    n_uint::from(round[0]) | (n_uint::from(round[1]) << 16)
}

pub fn math_root(input: n_uint) -> n_uint {
    let mut op = input;
    let mut res: n_uint = 0;
    let mut one: n_uint = 1 << ((std::mem::size_of::<n_uint>() * 8) - 2);

    while one > op {
        one >>= 2;
    }

    while one != 0 {
        if op >= res + one {
            op -= res + one;
            res += 2 * one;
        }
        res >>= 1;
        one >>= 2;
    }

    res
}

pub fn math_tan(point: n_vect2) -> n_int {
    let mut return_value = 0;
    let mut check_switch = 128;
    let mut vector_facing = vect2_direction(0, 8);
    let mut best_p = vect2_dot(point, vector_facing, 1, 1);

    while check_switch != 0 {
        vector_facing = vect2_direction(return_value + check_switch, 8);
        let temp_p = vect2_dot(point, vector_facing, 1, 1);
        if temp_p > best_p {
            best_p = temp_p;
            return_value += check_switch;
        }

        vector_facing = vect2_direction(return_value - check_switch + 256, 8);
        let temp_p = vect2_dot(point, vector_facing, 1, 1);
        if temp_p > best_p {
            best_p = temp_p;
            return_value -= check_switch;
        }

        check_switch >>= 1;
    }

    (return_value + 256) & 255
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IoNumber {
    pub characters_read: n_int,
    pub actual_value: n_int,
    pub decimal_divisor: n_int,
}

pub fn io_number_bytes(number_string: &[n_byte]) -> Result<IoNumber, &'static str> {
    if number_string.is_empty() || number_string[0] == 0 {
        return Err("empty number");
    }

    let mut temp: u128 = 0;
    let mut divisor: n_int = 0;
    let mut ten_power_place: n_int = 0;
    let mut string_point = 0usize;
    let mut negative = false;

    if number_string[0] == b'-' {
        negative = true;
        string_point += 1;
    }

    loop {
        let value = number_string.get(string_point).copied().unwrap_or(0);
        string_point += 1;

        if value == 0 {
            let actual_value = if negative {
                if temp == (n_int::MAX as u128) + 1 {
                    n_int::MIN
                } else {
                    -(temp as n_int)
                }
            } else {
                temp as n_int
            };

            if divisor > 0 {
                divisor -= 1;
            }

            return Ok(IoNumber {
                characters_read: ten_power_place,
                actual_value,
                decimal_divisor: divisor,
            });
        }

        if value == b'.' {
            if divisor != 0 {
                return Err("double decimal point in number");
            }
            divisor = 1;
            continue;
        }

        if !value.is_ascii_digit() {
            return Err("number contains non-numeric value");
        }

        let mod_ten = u128::from(value - b'0');
        if temp == 922_337_203_685_477_580 {
            if negative {
                if mod_ten > 8 {
                    return Err("number too small");
                }
            } else if mod_ten > 7 {
                return Err("number too large");
            }
        }
        if temp > 922_337_203_685_477_580 {
            return Err("number numerically too large");
        }
        if divisor != 0 {
            divisor += 1;
        }
        temp = (temp * 10) + mod_ten;
        ten_power_place += 1;
    }
}

pub fn io_number_to_string(number: n_uint) -> String {
    number.to_string()
}

pub fn io_string_number(input_string: &[n_byte], number: n_uint) -> Vec<n_byte> {
    let input_length = io_length_bytes(input_string, STRING_BLOCK_SIZE as n_int);
    let mut output = Vec::new();

    if input_length > 0 {
        output.extend_from_slice(&input_string[..input_length as usize]);
    }
    output.extend_from_slice(io_number_to_string(number).as_bytes());
    output.push(0);
    output
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFile {
    size: n_uint,
    location: n_uint,
    data: Vec<n_byte>,
}

impl NFile {
    pub fn new() -> Self {
        Self {
            size: STRING_BLOCK_SIZE as n_uint,
            location: 0,
            data: vec![0; STRING_BLOCK_SIZE],
        }
    }

    pub fn from_string_block(contents: &[n_byte; STRING_BLOCK_SIZE]) -> Self {
        let mut file = Self::new();
        file.data.copy_from_slice(contents);
        file.location = io_length_bytes(contents, STRING_BLOCK_SIZE as n_int) as n_uint;
        file
    }

    pub fn from_string(string: &[n_byte]) -> Self {
        let size = string.len().saturating_mul(2);
        let mut data = vec![0; size];
        data[..string.len()].copy_from_slice(string);

        Self {
            size: size as n_uint,
            location: 0,
            data,
        }
    }

    pub const fn size(&self) -> n_uint {
        self.size
    }

    pub const fn location(&self) -> n_uint {
        self.location
    }

    pub fn data(&self) -> &[n_byte] {
        &self.data
    }

    pub fn written_data(&self) -> &[n_byte] {
        &self.data[..self.location as usize]
    }

    pub fn reused(&mut self) {
        self.data.fill(0);
        self.location = 0;
    }

    pub fn write_byte(&mut self, byte: n_byte) -> Result<(), &'static str> {
        if (self.location + 1) == self.size {
            self.grow_for_write()?;
        }

        let location = self.location as usize;
        self.data[location] = byte;
        self.location += 1;
        Ok(())
    }

    pub fn write(&mut self, text: &[n_byte], new_line: n_byte) -> Result<(), &'static str> {
        for byte in text.iter().copied().take_while(|byte| *byte != 0) {
            self.write_byte(byte)?;
        }
        if (new_line & 1) != 0 {
            self.write_byte(b'\n')?;
        }
        if (new_line & 2) != 0 {
            self.write_byte(b'\t')?;
        }
        Ok(())
    }

    pub fn hash(&self) -> n_uint {
        let mut hash = math_hash(&self.location.to_ne_bytes());
        hash ^= math_hash(&self.size.to_ne_bytes());
        hash ^= math_hash(&self.data[..self.size as usize]);
        hash
    }

    fn grow_for_write(&mut self) -> Result<(), &'static str> {
        let local_size = self.size as usize;
        let temp_size = if local_size <= 256 * 1024 {
            local_size * 4
        } else if local_size <= 512 * 1024 * 1024 {
            local_size * 2
        } else {
            (local_size * 3) >> 1
        };

        if temp_size <= local_size {
            return Err("Attempted file overwrite");
        }

        self.data.resize(temp_size, 0);
        self.size = temp_size as n_uint;
        Ok(())
    }
}

impl Default for NFile {
    fn default() -> Self {
        Self::new()
    }
}

pub fn io_file_new() -> NFile {
    NFile::new()
}

pub fn io_file_new_from_string_block(contents: &[n_byte; STRING_BLOCK_SIZE]) -> NFile {
    NFile::from_string_block(contents)
}

pub fn io_file_new_from_string(string: &[n_byte]) -> NFile {
    NFile::from_string(string)
}

pub fn io_file_duplicate(initial: &NFile) -> NFile {
    initial.clone()
}

pub fn io_file_hash(local_file: &NFile) -> n_uint {
    local_file.hash()
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectType {
    Empty = 0,
    String = 1,
    Number = 2,
    Boolean = 3,
    Object = 4,
    Array = 5,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectValue {
    Empty,
    String(String),
    Number(n_int),
    Boolean(bool),
    Object(Vec<ObjectEntry>),
    Array(Vec<ObjectValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectEntry {
    pub name: String,
    pub name_hash: n_uint,
    pub value: ObjectValue,
}

impl ObjectEntry {
    pub fn new(name: impl Into<String>, value: ObjectValue) -> Self {
        let name = name.into();
        let name_hash = math_hash(name.as_bytes());
        Self {
            name,
            name_hash,
            value,
        }
    }
}

pub fn array_number(set_number: n_int) -> ObjectValue {
    ObjectValue::Number(set_number)
}

pub fn array_boolean(set_boolean: n_int) -> ObjectValue {
    ObjectValue::Boolean(set_boolean != 0)
}

pub fn array_string(set_string: impl Into<String>) -> ObjectValue {
    ObjectValue::String(set_string.into())
}

pub fn array_object(set_object: Vec<ObjectEntry>) -> ObjectValue {
    ObjectValue::Object(set_object)
}

pub fn array_array(set_array: Vec<ObjectValue>) -> ObjectValue {
    ObjectValue::Array(set_array)
}

pub fn array_add(array: &mut Vec<ObjectValue>, element: ObjectValue) {
    array.push(element);
}

pub fn object_number(
    object: &mut Vec<ObjectEntry>,
    name: impl Into<String>,
    number: n_int,
) -> usize {
    object.push(ObjectEntry::new(name, ObjectValue::Number(number)));
    object.len() - 1
}

pub fn object_boolean(
    object: &mut Vec<ObjectEntry>,
    name: impl Into<String>,
    boolean: n_int,
) -> usize {
    object.push(ObjectEntry::new(name, ObjectValue::Boolean(boolean != 0)));
    object.len() - 1
}

pub fn object_string(
    object: &mut Vec<ObjectEntry>,
    name: impl Into<String>,
    string: impl Into<String>,
) -> usize {
    object.push(ObjectEntry::new(name, ObjectValue::String(string.into())));
    object.len() - 1
}

pub fn object_object(
    object: &mut Vec<ObjectEntry>,
    name: impl Into<String>,
    nested: Vec<ObjectEntry>,
) -> usize {
    object.push(ObjectEntry::new(name, ObjectValue::Object(nested)));
    object.len() - 1
}

pub fn object_array(
    object: &mut Vec<ObjectEntry>,
    name: impl Into<String>,
    array: Vec<ObjectValue>,
) -> usize {
    object.push(ObjectEntry::new(name, ObjectValue::Array(array)));
    object.len() - 1
}

pub fn unknown_json(value: &ObjectValue) -> Option<NFile> {
    match value {
        ObjectValue::Object(_) | ObjectValue::Array(_) => {
            let mut file = NFile::new();
            write_object_value(&mut file, value).ok()?;
            Some(file)
        }
        _ => None,
    }
}

pub fn object_top_object(entries: &[ObjectEntry]) -> NFile {
    unknown_json(&ObjectValue::Object(entries.to_vec())).expect("object serializes")
}

pub fn object_parse_json(input: &[n_byte]) -> Result<ObjectValue, &'static str> {
    let mut parser = JsonParser::new(input);
    let value = parser.parse_value()?;
    parser.skip_whitespace();
    if parser.is_finished() {
        Ok(value)
    } else {
        Err("trailing json data")
    }
}

fn write_object_value(file: &mut NFile, value: &ObjectValue) -> Result<(), &'static str> {
    match value {
        ObjectValue::Empty => Err("Object kind not found"),
        ObjectValue::String(string) => {
            file.write(b"\"", 0)?;
            file.write(string.as_bytes(), 0)?;
            file.write(b"\"", 0)
        }
        ObjectValue::Number(number) => file.write(number.to_string().as_bytes(), 0),
        ObjectValue::Boolean(boolean) => {
            if *boolean {
                file.write(b"true", 0)
            } else {
                file.write(b"false", 0)
            }
        }
        ObjectValue::Object(entries) => {
            file.write(b"{", 0)?;
            write_object_entries(file, entries)?;
            file.write(b"}", 0)
        }
        ObjectValue::Array(values) => {
            file.write(b"[", 0)?;
            for (index, entry) in values.iter().enumerate() {
                write_object_value(file, entry)?;
                if index + 1 != values.len() {
                    file.write(b",", 0)?;
                }
            }
            file.write(b"]", 0)
        }
    }
}

fn write_object_entries(file: &mut NFile, entries: &[ObjectEntry]) -> Result<(), &'static str> {
    for (index, entry) in entries.iter().enumerate() {
        file.write(b"\"", 0)?;
        file.write(entry.name.as_bytes(), 0)?;
        file.write(b"\":", 0)?;
        write_object_value(file, &entry.value)?;
        if index + 1 != entries.len() {
            file.write(b",", 0)?;
        }
    }
    Ok(())
}

struct JsonParser<'a> {
    input: &'a [n_byte],
    location: usize,
}

impl<'a> JsonParser<'a> {
    const fn new(input: &'a [n_byte]) -> Self {
        Self { input, location: 0 }
    }

    const fn is_finished(&self) -> bool {
        self.location >= self.input.len()
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.location += 1;
        }
    }

    fn peek(&self) -> Option<n_byte> {
        self.input.get(self.location).copied()
    }

    fn take(&mut self) -> Option<n_byte> {
        let byte = self.peek()?;
        self.location += 1;
        Some(byte)
    }

    fn expect(&mut self, expected: n_byte) -> Result<(), &'static str> {
        match self.take() {
            Some(byte) if byte == expected => Ok(()),
            _ => Err("unexpected json character"),
        }
    }

    fn parse_value(&mut self) -> Result<ObjectValue, &'static str> {
        self.skip_whitespace();
        match self.peek() {
            Some(b'{') => self.parse_object().map(ObjectValue::Object),
            Some(b'[') => self.parse_array().map(ObjectValue::Array),
            Some(b'"') => self.parse_string().map(ObjectValue::String),
            Some(b't') => {
                self.expect_literal(b"true")?;
                Ok(ObjectValue::Boolean(true))
            }
            Some(b'f') => {
                self.expect_literal(b"false")?;
                Ok(ObjectValue::Boolean(false))
            }
            Some(b'n') => {
                self.expect_literal(b"null")?;
                Ok(ObjectValue::Empty)
            }
            Some(b'-' | b'0'..=b'9') => self.parse_number().map(ObjectValue::Number),
            _ => Err("json value expected"),
        }
    }

    fn parse_object(&mut self) -> Result<Vec<ObjectEntry>, &'static str> {
        self.expect(b'{')?;
        self.skip_whitespace();
        let mut entries = Vec::new();
        if matches!(self.peek(), Some(b'}')) {
            self.location += 1;
            return Ok(entries);
        }

        loop {
            self.skip_whitespace();
            let name = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            let value = self.parse_value()?;
            entries.push(ObjectEntry::new(name, value));
            self.skip_whitespace();
            match self.take() {
                Some(b',') => continue,
                Some(b'}') => return Ok(entries),
                _ => return Err("json object delimiter expected"),
            }
        }
    }

    fn parse_array(&mut self) -> Result<Vec<ObjectValue>, &'static str> {
        self.expect(b'[')?;
        self.skip_whitespace();
        let mut values = Vec::new();
        if matches!(self.peek(), Some(b']')) {
            self.location += 1;
            return Ok(values);
        }

        loop {
            values.push(self.parse_value()?);
            self.skip_whitespace();
            match self.take() {
                Some(b',') => continue,
                Some(b']') => return Ok(values),
                _ => return Err("json array delimiter expected"),
            }
        }
    }

    fn parse_string(&mut self) -> Result<String, &'static str> {
        self.expect(b'"')?;
        let mut output = String::new();
        loop {
            match self.take() {
                Some(b'"') => return Ok(output),
                Some(b'\\') => output.push(self.parse_escape()?),
                Some(byte) if byte < 0x20 => return Err("json string contains control byte"),
                Some(byte) => output.push(char::from(byte)),
                None => return Err("unterminated json string"),
            }
        }
    }

    fn parse_escape(&mut self) -> Result<char, &'static str> {
        match self.take() {
            Some(b'"') => Ok('"'),
            Some(b'\\') => Ok('\\'),
            Some(b'/') => Ok('/'),
            Some(b'b') => Ok('\u{0008}'),
            Some(b'f') => Ok('\u{000c}'),
            Some(b'n') => Ok('\n'),
            Some(b'r') => Ok('\r'),
            Some(b't') => Ok('\t'),
            Some(b'u') => Err("unicode json escapes not implemented"),
            _ => Err("unknown json escape"),
        }
    }

    fn parse_number(&mut self) -> Result<n_int, &'static str> {
        let start = self.location;
        if matches!(self.peek(), Some(b'-')) {
            self.location += 1;
        }

        let digits_start = self.location;
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.location += 1;
        }
        if self.location == digits_start {
            return Err("json number expected");
        }
        if matches!(self.peek(), Some(b'.' | b'e' | b'E')) {
            return Err("decimal json numbers not supported");
        }

        let number = std::str::from_utf8(&self.input[start..self.location])
            .map_err(|_| "invalid json number")?;
        number
            .parse::<n_int>()
            .map_err(|_| "json number out of range")
    }

    fn expect_literal(&mut self, literal: &[n_byte]) -> Result<(), &'static str> {
        if self.input.get(self.location..self.location + literal.len()) == Some(literal) {
            self.location += literal.len();
            Ok(())
        } else {
            Err("json literal expected")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    #[test]
    fn primitive_aliases_match_c_boundary_widths() {
        assert_eq!(size_of::<n_double>(), 8);
        assert_eq!(size_of::<n_char>(), size_of::<c_char>());
        assert_eq!(size_of::<n_byte>(), 1);
        assert_eq!(size_of::<n_byte2>(), 2);
        assert_eq!(size_of::<n_byte4>(), 4);
        assert_eq!(size_of::<n_c_int>(), size_of::<c_int>());
        assert_eq!(size_of::<n_int>(), size_of::<c_long>());
        assert_eq!(size_of::<n_uint>(), size_of::<c_ulong>());
        assert_eq!(size_of::<n_audio>(), 2);
    }

    #[test]
    fn string_block_matches_c_buffer_size() {
        assert_eq!(
            size_of::<n_string_block>(),
            STRING_BLOCK_SIZE * size_of::<n_char>()
        );
    }

    #[test]
    fn repr_c_foundation_structs_have_expected_layouts() {
        assert_eq!(size_of::<n_vect2>(), 2 * size_of::<n_int>());
        assert_eq!(align_of::<n_vect2>(), align_of::<n_int>());
        assert_eq!(size_of::<n_area2>(), 4 * size_of::<n_int>());
        assert_eq!(size_of::<n_quad>(), 8 * size_of::<n_int>());
        assert_eq!(size_of::<n_line>(), 4 * size_of::<n_int>());
        assert_eq!(size_of::<n_vect3>(), 3 * size_of::<n_double>());
        assert_eq!(size_of::<n_rgba>(), 4);
        assert_eq!(size_of::<n_rgba32>(), 4);
        assert_eq!(size_of::<n_spacetime>(), 12);
    }

    #[test]
    fn constants_match_c_toolkit_header() {
        assert_eq!(CHAR_SPACE, 32);
        assert_eq!(STRING_BLOCK_SIZE, 4096);
        assert_eq!(PACKED_DATA_BLOCK, 65_536);
        assert_eq!(SINE_MAXIMUM, 26_880);
        assert_eq!(BIG_INTEGER, 2_147_483_647);
        assert_eq!(BIG_NEGATIVE_INTEGER, -2_147_483_648);
    }

    #[test]
    fn memory_copy_and_erase_match_c_helpers() {
        let from = [1, 2, 3, 4, 5];
        let mut to = [0, 0, 0, 0, 0];
        memory_copy(&from, &mut to, 3);
        assert_eq!(to, [1, 2, 3, 0, 0]);

        memory_erase(&mut to);
        assert_eq!(to, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn memory_list_copies_units_and_doubles_when_full() {
        let mut list = MemoryList::new(2, 2).expect("valid unit size");
        list.copy_units(&[1, 2]).unwrap();
        assert_eq!(list.count(), 1);
        assert_eq!(list.max(), 2);

        list.copy_units(&[3, 4]).unwrap();
        assert_eq!(list.count(), 2);
        assert_eq!(list.max(), 4);
        assert_eq!(list.data(), &[1, 2, 3, 4]);

        assert_eq!(list.copy_units(&[5]).unwrap_err(), "wrong base unit size");
    }

    #[test]
    fn io_length_matches_c_max_and_zero_rules() {
        assert_eq!(io_length_bytes(b"ape\0extra", 4096), 3);
        assert_eq!(io_length_bytes(b"ape", 2), 2);
        assert_eq!(io_length_bytes(b"ape", 0), -1);
    }

    #[test]
    fn io_find_returns_position_after_match_like_c() {
        let command = b"help run";
        assert_eq!(
            io_find_bytes(command, 0, command.len() as n_int, b"help", 4),
            4
        );
        assert_eq!(command[4], b' ');
        assert_eq!(
            io_find_bytes(command, 0, command.len() as n_int, b"run", 3),
            8
        );
        assert_eq!(
            io_find_bytes(command, 0, command.len() as n_int, b"missing", 7),
            -1
        );
    }

    #[test]
    fn byte_string_helpers_match_c_behavior() {
        let mut value = *b"ApE_123";
        io_lower_bytes(&mut value);
        assert_eq!(&value, b"ape_123");

        let mut dest = [0u8; 16];
        let mut pos = 0;
        io_string_write_bytes(&mut dest, b"Sim", &mut pos);
        io_string_write_bytes(&mut dest, b"Ape\0ignored", &mut pos);
        assert_eq!(pos, 6);
        assert_eq!(&dest[..7], b"SimApe\0");
    }

    #[test]
    fn vect2_arithmetic_matches_c_helpers() {
        let first = n_vect2::new(10, -4);
        let second = n_vect2::new(2, 8);

        assert_eq!(vect2_add(first, second), n_vect2::new(12, 4));
        assert_eq!(vect2_subtract(first, second), n_vect2::new(8, -12));
        assert_eq!(vect2_center(first, second), n_vect2::new(6, 2));
        assert_eq!(vect2_divide(first, second, 2), Some(n_vect2::new(-4, 6)));
        assert_eq!(
            vect2_multiplier(first, second, 3, 2),
            Some(n_vect2::new(30, -48))
        );
        assert_eq!(vect2_dot(first, second, 3, 2), -18);
    }

    #[test]
    fn vect2_mutating_helpers_match_c_helpers() {
        let mut value = n_vect2::new(4, 5);
        assert!(vect2_d(&mut value, n_vect2::new(6, -3), 2, 3));
        assert_eq!(value, n_vect2::new(8, 3));

        vect2_delta(&mut value, n_vect2::new(1, 2));
        vect2_offset(&mut value, -3, 4);
        assert_eq!(value, n_vect2::new(6, 9));

        vect2_rotate90(&mut value);
        assert_eq!(value, n_vect2::new(9, -6));
    }

    #[test]
    fn vect2_conversion_and_bounds_match_c_helpers() {
        assert_eq!(vect2_byte2([7, 9]), n_vect2::new(7, 9));

        let mut high = n_vect2::new(70_000, -5);
        assert_eq!(vect2_back_byte2(&mut high), [65_535, 0]);
        assert_eq!(high, n_vect2::new(65_535, 0));
    }

    #[test]
    fn trig_and_minmax_helpers_match_c_tables() {
        assert_eq!(math_sine(0, 1), 0);
        assert_eq!(math_sine(64, 1), SINE_MAXIMUM);
        assert_eq!(vect2_direction(0, 1), n_vect2::new(SINE_MAXIMUM, 0));
        assert_eq!(vect2_direction(64, 1), n_vect2::new(0, SINE_MAXIMUM));

        let points = [
            n_vect2::new(5, 8),
            n_vect2::new(-2, 12),
            n_vect2::new(9, -3),
        ];
        let mut maxmin = [
            n_vect2::new(BIG_INTEGER, BIG_INTEGER),
            n_vect2::new(BIG_NEGATIVE_INTEGER, BIG_NEGATIVE_INTEGER),
        ];
        vect2_min_max(&points, &mut maxmin);
        assert_eq!(maxmin[0], n_vect2::new(-2, -3));
        assert_eq!(maxmin[1], n_vect2::new(9, 12));
    }

    #[test]
    fn hash_functions_match_c_reference_outputs() {
        assert_eq!(math_hash_fnv1(b""), 2_166_136_261);
        assert_eq!(math_hash_fnv1(b"ape"), 899_422_587);
        assert_eq!(math_hash_fnv1(b"ape\0ignored"), 899_422_587);
        assert_eq!(math_hash_fnv1(b"Simulated Ape"), 4_262_418_397);

        assert_eq!(math_hash(&[]), 12_044_969_459_213_400_696);
        assert_eq!(math_hash(b"ape"), 12_424_151_887_698_213_358);
        assert_eq!(math_hash(&[1, 2, 3, 4]), 11_848_353_262_066_058_304);
    }

    #[test]
    fn random_generator_matches_c_sequence() {
        let mut seed = [0x1234, 0xabcd];
        assert_eq!(math_random(&mut seed), 43_981);
        assert_eq!(seed, [43_981, 18_386]);

        math_random3(&mut seed);
        assert_eq!(seed, [9_793, 53_255]);
    }

    #[test]
    fn root_and_tan_match_c_behavior() {
        assert_eq!(math_root(0), 0);
        assert_eq!(math_root(1), 1);
        assert_eq!(math_root(2), 1);
        assert_eq!(math_root(15), 3);
        assert_eq!(math_root(16), 4);
        assert_eq!(math_root(17), 4);
        assert_eq!(math_root(262_144), 512);
        assert_eq!(math_root(123_456_789), 11_111);

        assert_eq!(math_tan(n_vect2::new(10, 0)), 0);
        assert_eq!(math_tan(n_vect2::new(0, 10)), 64);
        assert_eq!(math_tan(n_vect2::new(-10, 0)), 128);
        assert_eq!(math_tan(n_vect2::new(0, -10)), 192);
    }

    #[test]
    fn io_number_matches_c_integer_decimal_model() {
        assert_eq!(
            io_number_bytes(b"123\0").unwrap(),
            IoNumber {
                characters_read: 3,
                actual_value: 123,
                decimal_divisor: 0,
            }
        );
        assert_eq!(
            io_number_bytes(b"-12.34\0").unwrap(),
            IoNumber {
                characters_read: 4,
                actual_value: -1234,
                decimal_divisor: 2,
            }
        );
        assert_eq!(
            io_number_bytes(b".5\0").unwrap(),
            IoNumber {
                characters_read: 1,
                actual_value: 5,
                decimal_divisor: 1,
            }
        );
        assert_eq!(
            io_number_bytes(b"-\0").unwrap(),
            IoNumber {
                characters_read: 0,
                actual_value: 0,
                decimal_divisor: 0,
            }
        );
    }

    #[test]
    fn io_number_errors_match_c_parser_cases() {
        assert_eq!(io_number_bytes(b"").unwrap_err(), "empty number");
        assert_eq!(
            io_number_bytes(b"12x\0").unwrap_err(),
            "number contains non-numeric value"
        );
        assert_eq!(
            io_number_bytes(b"1.2.3\0").unwrap_err(),
            "double decimal point in number"
        );
        assert_eq!(
            io_number_bytes(b"9223372036854775808\0").unwrap_err(),
            "number too large"
        );
        assert_eq!(
            io_number_bytes(b"-9223372036854775809\0").unwrap_err(),
            "number too small"
        );
    }

    #[test]
    fn number_string_helpers_match_c_output() {
        assert_eq!(io_number_to_string(0), "0");
        assert_eq!(io_number_to_string(708), "708");
        assert_eq!(
            io_string_number(b"Simulated Ape \0", 708),
            b"Simulated Ape 708\0"
        );
        assert_eq!(io_string_number(b"\0ignored", 708), b"708\0");
    }

    #[test]
    fn nfile_constructors_match_c_buffer_rules() {
        let file = io_file_new();
        assert_eq!(file.size(), STRING_BLOCK_SIZE as n_uint);
        assert_eq!(file.location(), 0);
        assert_eq!(file.data().len(), STRING_BLOCK_SIZE);
        assert!(file.data().iter().all(|byte| *byte == 0));

        let from_string = io_file_new_from_string(b"ape");
        assert_eq!(from_string.size(), 6);
        assert_eq!(from_string.location(), 0);
        assert_eq!(from_string.data(), b"ape\0\0\0");

        let mut block = [0u8; STRING_BLOCK_SIZE];
        block[..3].copy_from_slice(b"ape");
        let from_block = io_file_new_from_string_block(&block);
        assert_eq!(from_block.size(), STRING_BLOCK_SIZE as n_uint);
        assert_eq!(from_block.location(), 3);
        assert_eq!(&from_block.data()[..4], b"ape\0");
    }

    #[test]
    fn nfile_write_growth_and_reuse_match_c_rules() {
        let mut file = NFile {
            size: 4,
            location: 0,
            data: vec![0; 4],
        };

        file.write(b"abc", 0).unwrap();
        assert_eq!(file.size(), 4);
        assert_eq!(file.location(), 3);
        assert_eq!(file.written_data(), b"abc");

        file.write(b"d", 0).unwrap();
        assert_eq!(file.size(), 16);
        assert_eq!(file.location(), 4);
        assert_eq!(file.written_data(), b"abcd");

        file.write(b"", 3).unwrap();
        assert_eq!(file.written_data(), b"abcd\n\t");

        file.reused();
        assert_eq!(file.location(), 0);
        assert!(file.data().iter().all(|byte| *byte == 0));
    }

    #[test]
    fn nfile_duplicate_and_hash_match_c_behavior() {
        let file = io_file_new_from_string(b"ape");
        let duplicate = io_file_duplicate(&file);
        assert_eq!(duplicate, file);
        assert_eq!(io_file_hash(&file), 845_956_104_357_156_201);

        let mut changed = file.clone();
        changed.write(b"x", 0).unwrap();
        assert_ne!(io_file_hash(&changed), io_file_hash(&file));
    }

    #[test]
    fn object_array_and_object_write_compact_json_like_c() {
        let mut genetics = Vec::new();
        array_add(&mut genetics, array_number(32334));
        array_add(&mut genetics, array_number(55586));

        let mut land = Vec::new();
        object_number(&mut land, "date", 0);
        object_array(&mut land, "genetics", genetics);
        object_number(&mut land, "time", 0);

        let mut information = Vec::new();
        object_number(&mut information, "signature", 20033);
        object_number(&mut information, "version number", 708);
        object_string(
            &mut information,
            "copyright",
            "Copyright Tom Barbalet, 1996-2026.",
        );
        object_string(&mut information, "date", "May  1 2026");

        let mut root = Vec::new();
        object_object(&mut root, "information", information);
        object_object(&mut root, "land", land);

        let file = object_top_object(&root);
        assert_eq!(
            file.written_data(),
            b"{\"information\":{\"signature\":20033,\"version number\":708,\"copyright\":\"Copyright Tom Barbalet, 1996-2026.\",\"date\":\"May  1 2026\"},\"land\":{\"date\":0,\"genetics\":[32334,55586],\"time\":0}}"
        );
    }

    #[test]
    fn unknown_json_supports_only_object_and_array_tops() {
        assert!(unknown_json(&ObjectValue::Number(7)).is_none());

        let array = ObjectValue::Array(vec![
            ObjectValue::Number(1),
            ObjectValue::Boolean(true),
            ObjectValue::String("ape".to_string()),
        ]);
        let file = unknown_json(&array).expect("array serializes");
        assert_eq!(file.written_data(), b"[1,true,\"ape\"]");

        let mut object = Vec::new();
        let first_index = object_number(&mut object, "first", 1);
        object_boolean(&mut object, "second", 0);
        assert_eq!(first_index, 0);
        assert_eq!(object[0].name_hash, math_hash(b"first"));

        let file = unknown_json(&ObjectValue::Object(object)).expect("object serializes");
        assert_eq!(file.written_data(), b"{\"first\":1,\"second\":false}");
    }

    #[test]
    fn object_parse_json_reads_compact_transfer_shape() {
        let parsed = object_parse_json(
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":0,\"genetics\":[7633,53305],\"time\":0}}",
        )
        .unwrap();
        let ObjectValue::Object(root) = parsed else {
            panic!("root should parse as object");
        };
        assert_eq!(root.len(), 2);
        assert_eq!(root[0].name, "information");
        assert_eq!(root[1].name, "land");
        assert_eq!(root[0].name_hash, math_hash(b"information"));
    }

    #[test]
    fn object_parse_json_handles_whitespace_booleans_and_escapes() {
        let parsed =
            object_parse_json(br#"{ "text": "ape\none", "ok": true, "values": [1, -2] }"#).unwrap();
        let ObjectValue::Object(root) = parsed else {
            panic!("root should parse as object");
        };
        assert_eq!(root[0].value, ObjectValue::String("ape\none".to_string()));
        assert_eq!(root[1].value, ObjectValue::Boolean(true));
        assert_eq!(
            root[2].value,
            ObjectValue::Array(vec![ObjectValue::Number(1), ObjectValue::Number(-2)])
        );
    }

    #[test]
    fn object_parse_json_rejects_trailing_and_decimal_data() {
        assert_eq!(
            object_parse_json(b"{\"a\":1}x").unwrap_err(),
            "trailing json data"
        );
        assert_eq!(
            object_parse_json(b"{\"a\":1.5}").unwrap_err(),
            "decimal json numbers not supported"
        );
    }
}
