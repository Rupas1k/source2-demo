use bitter::{BitReader, LittleEndianReader};

pub struct Reader<'a> {
    pub buf:        LittleEndianReader<'a>,
}

pub trait ReaderMethods<'a> {
    fn new(buf: &'a [u8]) -> Self;
    fn remain_bytes(&mut self) -> u32;
    fn remain_bits(&mut self) -> u32;
    fn read_bits(&mut self, amount: u32) -> u32;
    fn read_next_byte(&mut self) -> u8;
    fn read_byte(&mut self) -> u8;
    fn read_bytes(&mut self, amount: u32) -> Vec<u8>;
    fn read_bool(&mut self) -> bool;
    fn read_var_u32(&mut self) -> u32;
    fn read_var_u64(&mut self) -> u64;
    fn read_var_i32(&mut self) -> i32;
    fn read_ubit_var(&mut self) -> u32;
    fn read_ubit_var_fp(&mut self) -> u32;
    fn read_ubit_var_fieldpath(&mut self) -> i32;
    fn read_normal(&mut self) -> f32;
    fn read_3bit_normal(&mut self) -> [f32; 3];
    fn read_le_u64(&mut self) -> u64;
    fn read_string(&mut self) -> Option<String>;
    fn read_coordinate(&mut self) -> f32;
    fn read_angle(&mut self, n: u32) -> f32;
    fn read_string_n(&mut self, n: u32) -> String;
    fn read_bits_as_bytes(&mut self, n: u32) -> Vec<u8>;
}


impl<'a> ReaderMethods<'a> for Reader<'a> {
    fn new(buf: &'a [u8]) -> Self {
        let buf = LittleEndianReader::new(buf);
        Reader {
            buf,
        }
    }

    fn remain_bytes(&mut self) -> u32 {
        self.buf.bytes_remaining() as u32
    }

    fn remain_bits(&mut self) -> u32 {
        self.buf.bits_remaining().unwrap() as u32
    }

    fn read_bits(&mut self, amount: u32) -> u32 {
        self.buf.read_bits(amount).unwrap() as u32
    }

    fn read_next_byte(&mut self) -> u8 {
        self.buf.read_u8().unwrap()
    }

    fn read_byte(&mut self) -> u8 {
        self.read_next_byte()
    }

    fn read_bytes(&mut self, amount: u32) -> Vec<u8> {
        let mut bytes = vec![0; amount as usize];
        self.buf.read_bytes(&mut bytes);
        bytes
    }

    fn read_bool(&mut self) -> bool {
        self.read_bits(1) == 1
    }

    fn read_var_u32(&mut self) -> u32 {
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        loop {
            let byte = self.read_byte() as u32;

            x |= (byte & 0x7F) << y;
            y += 7;

            if (byte & 0x80) == 0 || y == 35 {
                break;
            }
        }
        x
    }

    fn read_var_u64(&mut self) -> u64 {
        let mut x: u64 = 0;
        let mut y: u64 = 0;

        loop {
            let b = self.read_byte();
            if b < 0x80 {
                return x | ((b as u64) << y)
            }
            x |= ((b & 0x7f) as u64) << y;
            y += 7;
        }
    }

    fn read_var_i32(&mut self) -> i32 {
        let ux = self.read_var_u32();
        let mut x = (ux >> 1) as i32;
        if ux & 1 != 0 {
            x = !x;
        }
        x
    }

    fn read_ubit_var(&mut self) -> u32 {
        let mut bits = self.read_bits(6) as u32;
        bits = match bits & 0x30 {
            0x10 => {
                (bits & 0xF) | (self.read_bits(4) << 4)
            }
            0x20 => {
                (bits & 0xF) | (self.read_bits(8) << 4)
            }
            0x30 => {
                (bits & 0xF) | (self.read_bits(28) << 4)
            }
            _ => bits
        };
        bits
    }

    fn read_ubit_var_fp (&mut self) -> u32 {
        if self.read_bool() {
            return self.read_bits(2);
        }
        if self.read_bool() {
            return self.read_bits(4);
        }
        if self.read_bool() {
            return self.read_bits(10);
        }
        if self.read_bool() {
            return self.read_bits(17);
        }
        self.read_bits(31)
    }

    fn read_ubit_var_fieldpath(&mut self) -> i32 {
        self.read_ubit_var_fp() as i32
    }

    fn read_normal(&mut self) -> f32 {
        let is_neg = self.read_bool();
        let len = self.read_bits(11) as f32;
        let normal = len * (1.0 / (1<<11) as f32 - 1.0);
        match is_neg {
            true => -normal,
            false => normal
        }
    }

    fn read_3bit_normal(&mut self) -> [f32; 3] {
        let mut vec = [0.0f32; 3];
        vec[0] = match self.read_bool() {
            true => self.read_normal(),
            false => vec[0],
        };
        vec[1] = match self.read_bool() {
            true => self.read_normal(),
            false => vec[1],
        };
        vec[2] = match (vec[0] * vec[0] + vec[1] * vec[1]) as f64 {
            x if x < 1.0 => (1.0 - x).sqrt() as f32,
            _ => vec[2]
        };
        vec[2] = match self.read_bool() {
            true => -vec[2],
            false => vec[2]
        };
        vec
    }

    fn read_le_u64(&mut self) -> u64 {
        u64::from_le_bytes((&self.read_bytes(8)[..8]).try_into().unwrap())
    }

    fn read_string(&mut self) -> Option<String> {
        let mut buf: Vec<u8> = vec![];
        loop {
            if self.buf.bytes_remaining() == 0 {
                return None
            }
            let b = self.read_byte();
            if b == 0 {
                break;
            }
            buf.push(b);
        }
        Some(String::from_utf8_lossy(&buf).to_string())
    }

    fn read_coordinate(&mut self) -> f32 {
        let mut value = 0f32;

        let mut int_val = self.read_bits(1);
        let mut fract_val = self.read_bits(1);

        if int_val != 0 || fract_val != 0 {
            let signbit = self.read_bool();

            if int_val != 0 {
                int_val = self.read_bits(14) + 1;
            }

            if fract_val != 0 {
                fract_val = self.read_bits(5);
            }

            value = (int_val as f32) + (fract_val as f32) * (1.0 / (1 << 5) as f32);

            if signbit {
                value = -value;
            }
        }
        value
    }

    fn read_angle(&mut self, n: u32) -> f32 {
        (self.read_bits(n) as f32) * 360.0 / (1 << n) as f32
    }

    fn read_string_n(&mut self, n: u32) -> String {
        String::from_utf8_lossy(self.read_bytes(n).as_slice()).parse().unwrap()
    }

    fn read_bits_as_bytes(&mut self, n: u32) -> Vec<u8> {
        // let mut tmp = Vec::<u8>::new();
        // while n >= 8 {
        //     tmp.push(self.read_byte());
        //     n -= 8;
        // }
        // if n > 0 {
        //     tmp.push(self.read_bits(n) as u8);
        // }
        // tmp

        let bytes = n / 8;
        let bits = n % 8;
        let mut tmp = vec![0; bytes as usize];
        self.buf.read_bytes(&mut tmp);
        if bits > 0 {
            tmp.push(self.buf.read_bits(bits).unwrap() as u8);
        }
        tmp
    }
}