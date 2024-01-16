use crate::reader::{Reader, ReaderMethods};

enum QFloatFlags {
    RoundDown       = 1 << 0,
    RoundUp         = 1 << 1,
    EncodeZero      = 1 << 2,
    EncodeInteger   = 1 << 3
}

#[derive(Debug)]
pub struct QFloatDecoder {
    // no_scale:       bool,
    bit_count:      u32,
    low:            Option<f32>,
    high:           Option<f32>,
    high_low_mul:   Option<f32>,
    dec_mul:        Option<f32>,
    offset:         Option<f32>,
    flags:          Option<u32>,
}

impl QFloatDecoder {
    // default low = 0.0 high = 1.0 flags = 0
    pub fn new(bit_count: i32, mut flags: Option<i32>, mut low_value: Option<f32>, mut high_value: Option<f32>) -> Self {
        if bit_count == 0 || bit_count >= 32 {
            return QFloatDecoder {
                // no_scale: true,
                bit_count: 32,
                low: None,
                high: None,
                high_low_mul: None,
                dec_mul: None,
                offset: None,
                flags: None,
            }
        }

        if low_value.is_none() {
            low_value = Some(0.0);
        }

        if high_value.is_none() {
            high_value = Some(1.0)
        }

        if flags.is_none() {
            flags = Some(0)
        }

        let mut decoder = QFloatDecoder {
            // no_scale: false,
            bit_count: bit_count as u32,
            offset: Some(0.0),
            low: low_value,
            high: high_value,
            flags: Some(flags.unwrap() as u32),
            high_low_mul: None,
            dec_mul: None
        };

        decoder.validate_flags();

        let mut steps = (1 << decoder.bit_count) as u32;

        if decoder.flags.unwrap() & (QFloatFlags::RoundDown as u32) != 0 {
            let range = decoder.high.unwrap() - decoder.low.unwrap();
            decoder.offset = Some(range / (steps as f32));
            decoder.high = Some(decoder.high.unwrap() - decoder.offset.unwrap());
        } else if decoder.flags.unwrap() & (QFloatFlags::RoundUp as u32) != 0 {
            let range = decoder.high.unwrap() - decoder.low.unwrap();
            decoder.offset = Some(range / (steps as f32));
            decoder.low = Some(decoder.low.unwrap() + decoder.offset.unwrap());
        }

        if decoder.flags.unwrap() & (QFloatFlags::EncodeInteger as u32) != 0 {
            let mut delta = decoder.high.unwrap() + decoder.low.unwrap();

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

            decoder.offset = Some((range2 as f32) / (steps as f32));
            decoder.high = Some(decoder.low.unwrap() + range2 as f32 - decoder.offset.unwrap())
        }

        decoder.assign_multipliers(steps);


        if decoder.flags.unwrap() & (QFloatFlags::RoundDown as u32) != 0 {
            if decoder.quantize(decoder.low.unwrap()) == decoder.low.unwrap() {
                decoder.flags = Some(decoder.flags.unwrap() & !(QFloatFlags::RoundDown as u32))
            }
        }

        if decoder.flags.unwrap() & (QFloatFlags::RoundUp as u32) != 0 {
            if decoder.quantize(decoder.high.unwrap()) == decoder.high.unwrap() {
                decoder.flags = Some(decoder.flags.unwrap() & !(QFloatFlags::RoundUp as u32))
            }
        }

        if decoder.flags.unwrap() & (QFloatFlags::EncodeZero as u32) != 0 {
            if decoder.quantize(0.0) == 0.0 {
                decoder.flags = Some(decoder.flags.unwrap() & !(QFloatFlags::EncodeZero as u32))
            }
        }

        decoder
    }

    fn validate_flags(&mut self) {
        let mut flags = self.flags.unwrap();
        let low = self.low.unwrap();
        let high = self.high.unwrap();

        if flags == 0 {
            return;
        }

        if low == 0.0 && (flags & QFloatFlags::RoundDown as u32) != 0 ||
            high == 0.0 && (flags & QFloatFlags::RoundUp as u32) != 0 {
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
            flags &= !(QFloatFlags::RoundUp as u32 | QFloatFlags::RoundDown as u32 | QFloatFlags::EncodeZero as u32);
        }

        if flags & (QFloatFlags::RoundUp as u32 | QFloatFlags::RoundDown as u32) == (QFloatFlags::RoundUp as u32 | QFloatFlags::RoundDown as u32) {
            panic!("Roundup / Rounddown are mutually exclusive");
        }

        self.flags = Some(flags);
    }

    fn assign_multipliers(&mut self, steps: u32) {
        self.high_low_mul = Some(0.0);
        let range = self.high.unwrap() - self.low.unwrap();

        let high = match self.bit_count {
            32 => 0xFFFFFFFEu32,
            x => (1 << x) - 1
        } as u32;

        let mut high_mul = match range.abs() as f64 <= 0.0 {
            true => high as f32,
            false => (high as f32) / range
        };

        if high_mul * range > (high as f32) || (high_mul * range) as f64 > (high as f64) {
            let multipliers: Vec<f32> = vec![0.9999, 0.99, 0.9, 0.8, 0.7];
            for mult in multipliers {
                high_mul = (high as f32) / range * mult;
                if high_mul * range > (high as f32) || (high_mul * range) as f64 > (high as f64) {
                    continue;
                }
                break;
            }
        }

        self.high_low_mul = Some(high_mul);

        self.dec_mul = Some(1.0 / ((steps - 1) as f32));

        if self.high_low_mul.unwrap() == 0.0 {
            panic!("Error computing high / low multiplier");
        }

    }

    pub fn quantize (&self, v: f32) -> f32 {
        if v < self.low.unwrap() {
            if self.flags.unwrap() & (QFloatFlags::RoundUp as u32) == 0 {
                panic!("Field tried to quantize an out of range value")
            }
            return self.low.unwrap();
        }

        if v > self.high.unwrap() {
            if self.flags.unwrap() & (QFloatFlags::RoundUp as u32) == 0 {
                panic!("Field tried to quantize an out of range value")
            }
            return self.high.unwrap();
        }

        let i = ((v - self.low.unwrap()) * self.high_low_mul.unwrap()) as u32;

        self.low.unwrap() + (self.high.unwrap() - self.low.unwrap()) * i as f32 * self.dec_mul.unwrap()
    }

    pub fn decode(&self, reader: &mut Reader) -> f32 {
        let flags = self.flags.unwrap();
        let low = self.low.unwrap();
        let high = self.high.unwrap();
        let dec_mul = self.dec_mul.unwrap();

        if flags & (QFloatFlags::RoundDown as u32) != 0 && reader.read_bool() {
            return low;
        }

        if flags & (QFloatFlags::RoundUp as u32) != 0 && reader.read_bool() {
            return high;
        }

        if flags & (QFloatFlags::EncodeZero as u32) != 0 && reader.read_bool() {
            return 0.0;
        }

        low + (high - low) * (reader.read_bits(self.bit_count) as f32) * dec_mul
    }
}