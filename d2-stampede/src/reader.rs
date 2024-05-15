use bitter::{BitReader, LittleEndianReader};

pub(crate) struct Reader<'a> {
    pub(crate) le_reader: LittleEndianReader<'a>,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Reader {
            le_reader: LittleEndianReader::new(buf),
        }
    }

    #[inline]
    pub fn empty(&mut self) -> bool {
        self.le_reader.bytes_remaining() == 0
    }

    #[inline]
    pub fn refill(&mut self) {
        #[cfg(feature = "unsafe_reader")]
        unsafe {
            self.le_reader.refill_lookahead_unchecked();
        }
        #[cfg(not(feature = "unsafe_reader"))]
        self.le_reader.refill_lookahead();
    }

    #[inline]
    pub fn read_bits(&mut self, amount: u32) -> u32 {
        self.refill();
        let x = self.le_reader.peek(amount);
        self.le_reader.consume(amount);
        x as u32
    }

    #[inline]
    pub fn read_bits_no_refill(&mut self, amount: u32) -> u32 {
        let x = self.le_reader.peek(amount);
        self.le_reader.consume(amount);
        x as u32
    }

    #[inline]
    pub fn read_bytes(&mut self, amount: u32) -> Vec<u8> {
        let mut bytes = vec![0; amount as usize];
        self.le_reader.read_bytes(&mut bytes);
        bytes
    }

    #[inline]
    pub fn read_bool(&mut self) -> bool {
        self.refill();
        let x = self.le_reader.peek(1);
        self.le_reader.consume(1);
        x == 1
    }

    #[inline]
    pub fn read_f32(&mut self) -> f32 {
        self.refill();
        f32::from_bits(self.read_bits_no_refill(32))
    }

    #[inline]
    pub fn read_var_u32(&mut self) -> u32 {
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        self.refill();
        loop {
            let byte = self.read_bits_no_refill(8);

            x |= (byte & 0x7F) << y;
            y += 7;

            if (byte & 0x80) == 0 || y == 35 {
                return x;
            }
        }
    }

    #[inline]
    pub fn read_var_u64(&mut self) -> u64 {
        let mut x: u64 = 0;
        let mut y: u8 = 0;
        self.refill();
        loop {
            let byte = self.read_bits_no_refill(8);

            x |= (byte as u64 & 0x7F) << y;
            y += 7;

            if y <= 21 {
                self.refill();
            }

            if (byte & 0x80) == 0 {
                return x;
            }
        }
    }

    #[inline]
    pub fn read_var_i32(&mut self) -> i32 {
        let ux: u32 = self.read_var_u32();
        if ux & 1 != 0 {
            return !((ux >> 1) as i32);
        }
        (ux >> 1) as i32
    }

    #[inline]
    pub fn read_ubit_var(&mut self) -> u32 {
        self.refill();
        let bits = self.read_bits_no_refill(6);
        match bits & 0x30 {
            0x10 => (bits & 0xF) | (self.read_bits_no_refill(4) << 4),
            0x20 => (bits & 0xF) | (self.read_bits_no_refill(8) << 4),
            0x30 => (bits & 0xF) | (self.read_bits_no_refill(28) << 4),
            _ => bits,
        }
    }

    #[inline]
    pub fn read_ubit_var_fp(&mut self) -> i32 {
        self.refill();
        if self.read_bits_no_refill(1) == 1 {
            return self.read_bits_no_refill(2) as i32;
        }
        if self.read_bits_no_refill(1) == 1 {
            return self.read_bits_no_refill(4) as i32;
        }
        if self.read_bits_no_refill(1) == 1 {
            return self.read_bits_no_refill(10) as i32;
        }
        if self.read_bits_no_refill(1) == 1 {
            return self.read_bits_no_refill(17) as i32;
        }
        self.read_bits_no_refill(31) as i32
    }

    #[inline]
    pub fn read_normal(&mut self) -> f32 {
        let is_neg = self.read_bits_no_refill(1) == 1;
        let len = self.read_bits_no_refill(11) as f32;
        let normal = len * (1.0 / (1 << 11) as f32 - 1.0);
        match is_neg {
            true => -normal,
            false => normal,
        }
    }

    #[inline]
    pub fn read_3bit_normal(&mut self) -> [f32; 3] {
        self.refill();
        let mut vec = [0.0f32; 3];
        vec[0] = match self.read_bits_no_refill(1) == 1 {
            true => self.read_normal(),
            false => vec[0],
        };
        vec[1] = match self.read_bits_no_refill(1) == 1 {
            true => self.read_normal(),
            false => vec[1],
        };
        vec[2] = match (vec[0] * vec[0] + vec[1] * vec[1]) as f64 {
            x if x < 1.0 => (1.0 - x).sqrt() as f32,
            _ => vec[2],
        };
        vec[2] = match self.read_bits_no_refill(1) == 1 {
            true => -vec[2],
            false => vec[2],
        };
        vec
    }

    #[inline]
    pub fn read_le_u64(&mut self) -> u64 {
        #[cfg(unsafe_reader)]
        unsafe {
            u64::from_le_bytes((&self.read_bytes(8)[..8]).try_into().unwrap_unchecked())
        }
        #[cfg(not(unsafe_reader))]
        {
            u64::from_le_bytes((&self.read_bytes(8)[..8]).try_into().unwrap())
        }
    }

    #[inline]
    pub fn read_string(&mut self) -> String {
        let mut buf: Vec<u8> = vec![];
        loop {
            let b = self.read_bits(8) as u8;
            if b == 0 {
                return unsafe { String::from_utf8_unchecked(buf) };
            }
            buf.push(b);
        }
    }

    #[inline]
    pub fn read_coordinate(&mut self) -> f32 {
        let mut value = 0f32;

        let mut int_val = self.read_bits_no_refill(1);
        let mut fract_val = self.read_bits_no_refill(1);

        if int_val != 0 || fract_val != 0 {
            let signbit = self.read_bits_no_refill(1);

            if int_val != 0 {
                int_val = self.read_bits_no_refill(14) + 1;
            }

            if fract_val != 0 {
                fract_val = self.read_bits_no_refill(5);
            }

            value = (int_val as f32) + (fract_val as f32) * (1.0 / (1 << 5) as f32);

            if signbit == 1 {
                value = -value;
            }
        }
        value
    }

    #[inline]
    pub fn read_angle(&mut self, n: u32) -> f32 {
        (self.read_bits_no_refill(n) as f32) * 360.0 / (1 << n) as f32
    }

    #[inline]
    pub fn read_bits_as_bytes(&mut self, n: u32) -> Vec<u8> {
        let bytes = n / 8;
        let bits = n % 8;
        let mut tmp = vec![0; bytes as usize];
        self.le_reader.read_bytes(&mut tmp);
        if bits > 0 {
            self.refill();
            tmp.push(self.read_bits_no_refill(bits) as u8);
        }
        tmp
    }
}
