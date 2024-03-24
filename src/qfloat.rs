use crate::reader::Reader;

enum QFloatFlags {
    RoundDown = 1 << 0,
    RoundUp = 1 << 1,
    EncodeZero = 1 << 2,
    EncodeInteger = 1 << 3,
}

#[derive(Debug)]
pub(crate) struct QFloatDecoder {
    bit_count: u32,
    low: f32,
    high: f32,
    high_low_mul: f32,
    dec_mul: f32,
    offset: f32,
    flags: u32,
}

impl QFloatDecoder {
    pub(crate) fn new(bit_count: i32, flags: i32, low_value: f32, high_value: f32) -> Self {
        if bit_count == 0 || bit_count >= 32 {
            return QFloatDecoder {
                bit_count: 32,
                low: 0.0,
                high: 1.0,
                high_low_mul: 0.0,
                dec_mul: 0.0,
                offset: 0.0,
                flags: 0,
            };
        }

        let mut decoder = QFloatDecoder {
            bit_count: bit_count as u32,
            offset: 0.0,
            low: low_value,
            high: high_value,
            flags: flags as u32,
            high_low_mul: 0.0,
            dec_mul: 0.0,
        };

        decoder.validate_flags();

        let mut steps = (1 << decoder.bit_count) as u32;

        if decoder.flags & (QFloatFlags::RoundDown as u32) != 0 {
            let range = decoder.high - decoder.low;
            decoder.offset = range / (steps as f32);
            decoder.high -= decoder.offset;
        } else if decoder.flags & (QFloatFlags::RoundUp as u32) != 0 {
            let range = decoder.high - decoder.low;
            decoder.offset = range / (steps as f32);
            decoder.low += decoder.offset;
        }

        if decoder.flags & (QFloatFlags::EncodeInteger as u32) != 0 {
            let mut delta = decoder.high + decoder.low;

            if delta < 1.0 {
                delta = 1.0
            }

            let delta_log2 = (delta as f64).log2().ceil();
            let range2 = (1 << (delta_log2 as u32)) as u32;
            let mut bc = decoder.bit_count;

            while (1 << bc) <= range2 {
                bc += 1;
            }

            if bc > decoder.bit_count {
                decoder.bit_count = bc;
                steps = 1 << decoder.bit_count;
            }

            decoder.offset = (range2 as f32) / (steps as f32);
            decoder.high = decoder.low + range2 as f32 - decoder.offset
        }

        decoder.assign_multipliers(steps);

        if decoder.flags & (QFloatFlags::RoundDown as u32) != 0
            && decoder.quantize(decoder.low) == decoder.low
        {
            decoder.flags &= !(QFloatFlags::RoundDown as u32)
        }

        if decoder.flags & (QFloatFlags::RoundUp as u32) != 0
            && decoder.quantize(decoder.high) == decoder.high
        {
            decoder.flags &= !(QFloatFlags::RoundUp as u32)
        }

        if decoder.flags & (QFloatFlags::EncodeZero as u32) != 0 && decoder.quantize(0.0) == 0.0 {
            decoder.flags &= !(QFloatFlags::EncodeZero as u32)
        }

        decoder
    }

    fn validate_flags(&mut self) {
        let mut flags = self.flags;
        let low = self.low;
        let high = self.high;

        if flags == 0 {
            return;
        }

        if low == 0.0 && (flags & QFloatFlags::RoundDown as u32) != 0
            || high == 0.0 && (flags & QFloatFlags::RoundUp as u32) != 0
        {
            flags &= !(QFloatFlags::EncodeZero as u32);
        }

        if low == 0.0 && (flags & QFloatFlags::EncodeZero as u32) != 0 {
            flags |= QFloatFlags::RoundUp as u32;
            flags &= !(QFloatFlags::EncodeZero as u32);
        }

        if high == 0.0 && (flags & QFloatFlags::EncodeZero as u32) != 0 {
            flags |= QFloatFlags::RoundDown as u32;
            flags &= !(QFloatFlags::EncodeZero as u32);
        }

        if low > 0.0 || high < 0.0 {
            flags &= !(QFloatFlags::EncodeZero as u32);
        }

        if flags & (QFloatFlags::EncodeInteger as u32) != 0 {
            flags &= !(QFloatFlags::RoundUp as u32
                | QFloatFlags::RoundDown as u32
                | QFloatFlags::EncodeZero as u32);
        }

        // if flags & (QFloatFlags::RoundUp as u32 | QFloatFlags::RoundDown as u32)
        //     == (QFloatFlags::RoundUp as u32 | QFloatFlags::RoundDown as u32)
        // {
        //     panic!("Roundup / Rounddown are mutually exclusive");
        // }

        self.flags = flags;
    }

    fn assign_multipliers(&mut self, steps: u32) {
        let range = self.high - self.low;

        let high = if self.bit_count == 32 {
            0xFFFFFFFEu32
        } else {
            (1 << self.bit_count) - 1
        };

        let mut high_mul = if range.abs() as f64 <= 0.0 {
            high as f32
        } else {
            (high as f32) / range
        };

        if high_mul * range > (high as f32) || (high_mul * range) as f64 > (high as f64) {
            for mult in [0.9999, 0.99, 0.9, 0.8, 0.7] {
                high_mul = (high as f32) / range * mult;
                if high_mul * range <= high as f32 && (high_mul * range) as f64 <= high as f64 {
                    break;
                }
            }
        }

        self.high_low_mul = high_mul;

        self.dec_mul = 1.0 / ((steps - 1) as f32);

        // if self.high_low_mul == 0.0 {
        //     panic!("Error computing high / low multiplier");
        // }
    }

    pub(crate) fn quantize(&self, v: f32) -> f32 {
        if v < self.low {
            // if self.flags & (QFloatFlags::RoundUp as u32) == 0 {
            //     panic!("Field tried to quantize an out of range value")
            // }
            return self.low;
        }

        if v > self.high {
            // if self.flags & (QFloatFlags::RoundUp as u32) == 0 {
            //     panic!("Field tried to quantize an out of range value")
            // }
            return self.high;
        }

        let i = ((v - self.low) * self.high_low_mul) as u32;

        self.low + (self.high - self.low) * i as f32 * self.dec_mul
    }

    pub(crate) fn decode(&self, reader: &mut Reader) -> f32 {
        if self.flags & (QFloatFlags::RoundDown as u32) != 0 && reader.read_bool() {
            return self.low;
        }

        if self.flags & (QFloatFlags::RoundUp as u32) != 0 && reader.read_bool() {
            return self.high;
        }

        if self.flags & (QFloatFlags::EncodeZero as u32) != 0 && reader.read_bool() {
            return 0.0;
        }

        self.low + (self.high - self.low) * (reader.read_bits(self.bit_count) as f32) * self.dec_mul
    }
}
