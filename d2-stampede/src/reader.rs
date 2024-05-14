use anyhow::Result;
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
    pub fn remain_bytes(&mut self) -> u32 {
        self.le_reader.bytes_remaining() as u32
    }

    #[inline]
    pub fn read_bits(&mut self, amount: u32) -> u32 {
        #[cfg(unsafe_reader)]
        unsafe {
            self.le_reader.read_bits(amount).unwrap_unchecked() as u32
        }
        #[cfg(not(unsafe_reader))]
        {
            self.le_reader.read_bits(amount).unwrap() as u32
        }
    }

    #[inline]
    pub fn read_next_byte(&mut self) -> u8 {
        #[cfg(unsafe_reader)]
        unsafe {
            self.le_reader.read_u8().unwrap_unchecked()
        }
        #[cfg(not(unsafe_reader))]
        {
            self.le_reader.read_u8().unwrap()
        }
    }

    #[inline]
    pub fn read_byte(&mut self) -> u8 {
        self.read_next_byte()
    }

    #[inline]
    pub fn read_bytes(&mut self, amount: u32) -> Vec<u8> {
        let mut bytes = vec![0; amount as usize];
        self.le_reader.read_bytes(&mut bytes);
        bytes
    }

    #[inline]
    pub fn read_bool(&mut self) -> bool {
        #[cfg(unsafe_reader)]
        unsafe {
            self.le_reader.read_bit().unwrap_unchecked()
        }
        #[cfg(not(unsafe_reader))]
        {
            self.le_reader.read_bit().unwrap()
        }
    }

    #[inline]
    pub fn read_f32(&mut self) -> f32 {
        #[cfg(unsafe_reader)]
        unsafe {
            self.le_reader.read_f32().unwrap_unchecked()
        }
        #[cfg(not(unsafe_reader))]
        {
            self.le_reader.read_f32().unwrap()
        }
    }

    #[inline]
    pub fn read_var_u32(&mut self) -> u32 {
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        loop {
            let byte = self.read_byte() as u32;

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
        let mut byte: u8;
        loop {
            byte = self.read_byte();

            x |= (byte as u64 & 0x7F) << y;
            y += 7;

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
        let bits = self.read_bits(6);
        match bits & 0x30 {
            0x10 => (bits & 0xF) | (self.read_bits(4) << 4),
            0x20 => (bits & 0xF) | (self.read_bits(8) << 4),
            0x30 => (bits & 0xF) | (self.read_bits(28) << 4),
            _ => bits,
        }
    }

    #[inline]
    pub fn read_ubit_var_fp(&mut self) -> i32 {
        if self.read_bool() {
            return self.read_bits(2) as i32;
        }
        if self.read_bool() {
            return self.read_bits(4) as i32;
        }
        if self.read_bool() {
            return self.read_bits(10) as i32;
        }
        if self.read_bool() {
            return self.read_bits(17) as i32;
        }
        self.read_bits(31) as i32
    }

    #[inline]
    pub fn read_normal(&mut self) -> f32 {
        let is_neg = self.read_bool();
        let len = self.read_bits(11) as f32;
        let normal = len * (1.0 / (1 << 11) as f32 - 1.0);
        match is_neg {
            true => -normal,
            false => normal,
        }
    }

    #[inline]
    pub fn read_3bit_normal(&mut self) -> [f32; 3] {
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
    pub fn read_string(&mut self) -> Result<String> {
        let mut buf: Vec<u8> = vec![];
        loop {
            if self.le_reader.bytes_remaining() == 0 {
                return Ok(String::new());
            }
            let b = self.read_byte();
            if b == 0 {
                return Ok(String::from_utf8_lossy(&buf).to_string());
            }
            buf.push(b);
        }
    }

    #[inline]
    pub fn read_coordinate(&mut self) -> f32 {
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

    #[inline]
    pub fn read_angle(&mut self, n: u32) -> f32 {
        (self.read_bits(n) as f32) * 360.0 / (1 << n) as f32
    }

    #[inline]
    pub fn read_bits_as_bytes(&mut self, n: u32) -> Vec<u8> {
        let bytes = n / 8;
        let bits = n % 8;
        let mut tmp = vec![0; bytes as usize];
        self.le_reader.read_bytes(&mut tmp);
        if bits > 0 {
            #[cfg(unsafe_reader)]
            unsafe {
                tmp.push(self.le_reader.read_bits(bits).unwrap_unchecked() as u8);
            }
            #[cfg(not(unsafe_reader))]
            {
                tmp.push(self.le_reader.read_bits(bits).unwrap() as u8);
            }
        }
        tmp
    }
}
