use bitter::{BitReader, LittleEndianReader};

pub(crate) struct Reader<'a> {
    pub(crate) le_reader: LittleEndianReader<'a>,
    string_buf: [u8; 4096],
}

impl<'a> Reader<'a> {
    pub(crate) fn new(buf: &'a [u8]) -> Self {
        Reader {
            le_reader: LittleEndianReader::new(buf),
            string_buf: [0; 4096],
        }
    }

    #[inline(always)]
    pub(crate) fn empty(&mut self) -> bool {
        self.le_reader.bytes_remaining() == 0
    }

    #[inline(always)]
    pub(crate) fn refill(&mut self) {
        #[cfg(not(debug_assertions))]
        unsafe {
            self.le_reader.refill_lookahead_unchecked();
        }
        #[cfg(debug_assertions)]
        self.le_reader.refill_lookahead();
    }

    #[inline(always)]
    pub(crate) fn read_bits(&mut self, amount: u32) -> u32 {
        self.refill();
        self.read_bits_no_refill(amount)
    }

    #[inline(always)]
    pub(crate) fn read_bits_no_refill(&mut self, amount: u32) -> u32 {
        debug_assert!(amount <= 32);
        debug_assert!(self.le_reader.has_bits_remaining(amount as usize));
        let x = self.le_reader.peek(amount);
        self.le_reader.consume(amount);
        x as u32
    }

    #[inline]
    pub(crate) fn read_bytes(&mut self, amount: u32) -> Vec<u8> {
        let mut bytes = vec![0; amount as usize];
        self.le_reader.read_bytes(&mut bytes);
        bytes
    }

    #[inline(always)]
    pub(crate) fn read_bool(&mut self) -> bool {
        // self.refill();
        let x = self.le_reader.peek(1);
        self.le_reader.consume(1);
        x == 1
    }

    #[inline(always)]
    pub(crate) fn read_f32(&mut self) -> f32 {
        f32::from_bits(self.read_bits(32))
    }

    #[inline(always)]
    pub(crate) fn read_var_u32(&mut self) -> u32 {
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

    #[inline(always)]
    pub(crate) fn read_var_u64(&mut self) -> u64 {
        let mut x: u64 = 0;
        let mut y: u8 = 0;
        self.refill();
        loop {
            let byte = self.read_bits_no_refill(8);

            x |= (byte as u64 & 0x7F) << y;
            y += 7;

            if (byte & 0x80) == 0 {
                return x;
            }

            if y == 49 {
                self.refill();
            }
        }
    }

    #[inline(always)]
    pub(crate) fn read_var_i32(&mut self) -> i32 {
        let ux: u32 = self.read_var_u32();
        if ux & 1 != 0 {
            return !((ux >> 1) as i32);
        }
        (ux >> 1) as i32
    }

    const UBV_COUNT: [u8; 4] = [0, 4, 8, 28];
    #[inline(always)]
    pub(crate) fn read_ubit_var(&mut self) -> u32 {
        self.refill();
        let a = self.read_bits_no_refill(6);
        let b = a >> 4;
        if a == 0 {
            return b;
        }
        (a & 15) | (self.read_bits_no_refill(Self::UBV_COUNT[b as usize] as u32) << 4)
    }

    const UBVFP_COUNT: [u8; 5] = [2, 4, 10, 17, 31];
    #[inline(always)]
    pub(crate) fn read_ubit_var_fp(&mut self) -> i32 {
        let mut i: u8 = 0;
        self.refill();
        while i < 4 && !self.read_bool() {
            i += 1
        }
        self.read_bits_no_refill(Self::UBVFP_COUNT[i as usize] as u32) as i32
    }

    #[inline(always)]
    pub(crate) fn read_ubit_var_fp_no_refill(&mut self) -> i32 {
        let mut i: u8 = 0;
        while i < 4 && !self.read_bool() {
            i += 1
        }
        self.read_bits_no_refill(Self::UBVFP_COUNT[i as usize] as u32) as i32
    }

    const NORMAL_FACTOR: f32 = (1.0 / (1 << 11) as f32) - 1.0;
    #[inline(always)]
    pub(crate) fn read_normal(&mut self) -> f32 {
        let is_neg = self.read_bool();
        let len = self.read_bits_no_refill(11) as f32;
        let normal = len * Self::NORMAL_FACTOR;
        match is_neg {
            true => -normal,
            false => normal,
        }
    }

    #[inline(always)]
    pub(crate) fn read_3bit_normal(&mut self) -> [f32; 3] {
        self.refill();
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
            _ => vec[2],
        };
        vec[2] = match self.read_bool() {
            true => -vec[2],
            false => vec[2],
        };
        vec
    }

    #[inline(always)]
    pub(crate) fn read_le_u64(&mut self) -> u64 {
        self.le_reader.read_u64().unwrap()
    }

    #[inline(always)]
    pub(crate) fn read_string(&mut self) -> String {
        let mut i = 0;
        loop {
            let b = self.read_bits(8) as u8;
            if b == 0 {
                return String::from_utf8_lossy(&self.string_buf[..i]).into();
            }
            self.string_buf[i] = b;
            i += 1;
        }
    }

    const FRACTION_FACTOR: f32 = (1.0 / (1 << 5) as f32);
    #[inline(always)]
    pub(crate) fn read_coordinate(&mut self) -> f32 {
        self.refill();

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

            value = (int_val as f32) + (fract_val as f32) * Self::FRACTION_FACTOR;

            if signbit == 1 {
                value = -value;
            }
        }
        value
    }

    #[inline(always)]
    pub(crate) fn read_angle(&mut self, n: u32) -> f32 {
        (self.read_bits_no_refill(n) as f32) * 360.0 / (1 << n) as f32
    }

    #[inline(always)]
    pub(crate) fn read_bits_as_bytes(&mut self, n: u32) -> Vec<u8> {
        let bits = n % 8;
        let mut tmp = vec![0; (n >> 3) as usize];
        self.le_reader.read_bytes(&mut tmp);
        if bits > 0 {
            tmp.push(self.read_bits(bits) as u8);
        }
        tmp
    }
}
