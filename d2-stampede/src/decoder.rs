use crate::field::{Encoder, FieldProperties, FieldType};
use crate::field_value::FieldValue;
use crate::reader::Reader;

pub enum Decoder {
    VectorNormal,
    Fixed64,
    Boolean,
    String,
    Signed8,
    Signed16,
    Signed32,
    Signed64,
    Unsigned8,
    Unsigned16,
    Unsigned32,
    FloatCoordinate,
    NoScale,
    RuneTime,
    SimulationTime,
    Component,

    Vector(FieldProperties, u8),
    Unsigned64(FieldProperties),
    Float32(FieldProperties),
    QuantizedFloat(FieldProperties),
    QAngle(FieldProperties),
}

impl Decoder {
    #[inline]
    pub(crate) fn from_field(field_type: &FieldType, properties: FieldProperties) -> Self {
        match field_type.base.as_ref() {
            "bool" => Decoder::Boolean,
            "char" | "CUtlString" | "CUtlSymbolLarge" => Decoder::String,
            "int8" => Decoder::Signed8,
            "int16" => Decoder::Signed16,
            "int32" => Decoder::Signed32,
            "int64" => Decoder::Signed64,
            "uint8" | "BloodType" => Decoder::Unsigned8,
            "uint16" => Decoder::Unsigned16,
            "uint32"
            | "color32"
            | "CGameSceneNodeHandle"
            | "Color"
            | "CUtlStringToken"
            | "CHandle"
            | "CEntityHandle" => Decoder::Unsigned32,
            "GameTime_t" => Decoder::NoScale,
            "CBodyComponent" | "CPhysicsComponent" | "CRenderComponent" => Decoder::Component,

            "CNetworkedQuantizedFloat" => Decoder::QuantizedFloat(properties),

            "float32" => Decoder::Float32(properties),

            "Vector" => Decoder::Vector(properties, 3),
            "Vector2D" => Decoder::Vector(properties, 2),
            "Vector4D" => Decoder::Vector(properties, 4),

            "uint64" | "CStrongHandle" | "HeroFacetKey_t" => Decoder::Unsigned64(properties),

            "QAngle" => Decoder::QAngle(properties),

            _ => Decoder::Unsigned32,
        }
    }

    #[inline]
    pub(crate) fn decode(&self, reader: &mut Reader) -> FieldValue {
        match self {
            Decoder::VectorNormal => FieldValue::Vector3D(reader.read_3bit_normal()),
            Decoder::Fixed64 => FieldValue::Unsigned64(reader.read_le_u64()),
            Decoder::Boolean => FieldValue::Boolean({
                reader.refill();
                reader.read_bool()
            }),
            Decoder::String => FieldValue::String(reader.read_string()),
            Decoder::Signed8 => FieldValue::Signed8(reader.read_var_i32() as i8),
            Decoder::Signed16 => FieldValue::Signed16(reader.read_var_i32() as i16),
            Decoder::Signed32 => FieldValue::Signed32(reader.read_var_i32()),
            Decoder::Signed64 => FieldValue::Signed64(reader.read_var_i32() as i64),
            Decoder::FloatCoordinate => FieldValue::Float(reader.read_coordinate()),
            Decoder::NoScale => FieldValue::Float(reader.read_f32()),
            Decoder::RuneTime => FieldValue::Float(f32::from_bits(reader.read_bits(4))),
            Decoder::SimulationTime => FieldValue::Float(reader.read_var_u32() as f32 / 30.0),
            Decoder::Unsigned8 => FieldValue::Unsigned8(reader.read_var_u32() as u8),
            Decoder::Unsigned16 => FieldValue::Unsigned16(reader.read_var_u32() as u16),
            Decoder::Unsigned32 => FieldValue::Unsigned32(reader.read_var_u32()),
            Decoder::Component => FieldValue::Boolean({
                reader.refill();
                reader.read_bool()
            }),
            Decoder::Float32(fp) => match fp.encoder {
                Some(Encoder::Coord) => Decoder::FloatCoordinate.decode(reader),
                Some(Encoder::SimTime) => Decoder::SimulationTime.decode(reader),
                Some(Encoder::RuneTime) => Decoder::RuneTime.decode(reader),
                _ => {
                    if fp.bit_count <= 0 || fp.bit_count >= 32 {
                        return Decoder::NoScale.decode(reader);
                    }
                    Decoder::QuantizedFloat(*fp).decode(reader)
                }
            },
            Decoder::Vector(fp, n) => {
                if *n == 2 {
                    return FieldValue::Vector2D([
                        Decoder::Float32(*fp).decode(reader).as_float(),
                        Decoder::Float32(*fp).decode(reader).as_float(),
                    ]);
                }
                if *n == 3 {
                    if fp.encoder == Some(Encoder::Normal) {
                        return Decoder::VectorNormal.decode(reader);
                    }
                    return FieldValue::Vector3D([
                        Decoder::Float32(*fp).decode(reader).as_float(),
                        Decoder::Float32(*fp).decode(reader).as_float(),
                        Decoder::Float32(*fp).decode(reader).as_float(),
                    ]);
                }
                if *n == 4 {
                    return FieldValue::Vector4D([
                        Decoder::Float32(*fp).decode(reader).as_float(),
                        Decoder::Float32(*fp).decode(reader).as_float(),
                        Decoder::Float32(*fp).decode(reader).as_float(),
                        Decoder::Float32(*fp).decode(reader).as_float(),
                    ]);
                }
                unreachable!()
            }
            Decoder::Unsigned64(fp) => {
                if fp.encoder == Some(Encoder::Fixed64) {
                    return Decoder::Fixed64.decode(reader);
                }
                FieldValue::Unsigned64(reader.read_var_u64())
            }
            Decoder::QuantizedFloat(fp) => {
                FieldValue::Float(QuantizedFloatDecoder::new(fp).decode(reader))
            }
            Decoder::QAngle(fp) => {
                reader.refill();
                if fp.encoder == Some(Encoder::QAnglePitchYaw) {
                    return FieldValue::Vector3D([
                        reader.read_angle(fp.bit_count as u32),
                        reader.read_angle(fp.bit_count as u32),
                        0.0,
                    ]);
                }

                if fp.bit_count != 0 {
                    let n = fp.bit_count as u32;
                    return FieldValue::Vector3D([
                        reader.read_angle(n),
                        reader.read_angle(n),
                        reader.read_angle(n),
                    ]);
                }

                let mut v = [0f32; 3];
                let x = reader.read_bool();
                let y = reader.read_bool();
                let z = reader.read_bool();
                if x {
                    v[0] = reader.read_coordinate();
                }
                if y {
                    v[1] = reader.read_coordinate();
                }
                if z {
                    v[2] = reader.read_coordinate();
                }
                FieldValue::Vector3D(v)
            }
        }
    }
}

// Quantized float decoder

enum QuantizedFloatFlags {
    RoundDown = 1 << 0,
    RoundUp = 1 << 1,
    EncodeZero = 1 << 2,
    EncodeInteger = 1 << 3,
}

#[derive(Debug)]
pub(crate) struct QuantizedFloatDecoder {
    bit_count: u32,
    low: f32,
    high: f32,
    high_low_mul: f32,
    dec_mul: f32,
    offset: f32,
    flags: u32,
}

impl QuantizedFloatDecoder {
    pub(crate) fn new(field_properties: &FieldProperties) -> Self {
        if field_properties.bit_count == 0 || field_properties.bit_count >= 32 {
            return QuantizedFloatDecoder {
                bit_count: 32,
                low: 0.0,
                high: 1.0,
                high_low_mul: 0.0,
                dec_mul: 0.0,
                offset: 0.0,
                flags: 0,
            };
        }

        let mut decoder = QuantizedFloatDecoder {
            bit_count: field_properties.bit_count as u32,
            offset: 0.0,
            low: field_properties.low_value,
            high: field_properties.high_value,
            flags: field_properties.encoder_flags as u32,
            high_low_mul: 0.0,
            dec_mul: 0.0,
        };

        decoder.validate_flags();

        let mut steps = (1 << decoder.bit_count) as u32;

        if decoder.flags & (QuantizedFloatFlags::RoundDown as u32) != 0 {
            decoder.offset = (decoder.high - decoder.low) / (steps as f32);
            decoder.high -= decoder.offset;
        } else if decoder.flags & (QuantizedFloatFlags::RoundUp as u32) != 0 {
            decoder.offset = (decoder.high - decoder.low) / (steps as f32);
            decoder.low += decoder.offset;
        }

        if decoder.flags & (QuantizedFloatFlags::EncodeInteger as u32) != 0 {
            let delta = 1.0f32.max(decoder.high + decoder.low).log2().ceil() as u32;
            let range2 = (1 << delta) as u32;
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

        if decoder.flags & (QuantizedFloatFlags::RoundDown as u32) != 0
            && decoder.quantize(decoder.low) == decoder.low
        {
            decoder.flags &= !(QuantizedFloatFlags::RoundDown as u32)
        }

        if decoder.flags & (QuantizedFloatFlags::RoundUp as u32) != 0
            && decoder.quantize(decoder.high) == decoder.high
        {
            decoder.flags &= !(QuantizedFloatFlags::RoundUp as u32)
        }

        if decoder.flags & (QuantizedFloatFlags::EncodeZero as u32) != 0
            && decoder.quantize(0.0) == 0.0
        {
            decoder.flags &= !(QuantizedFloatFlags::EncodeZero as u32)
        }

        decoder
    }

    fn validate_flags(&mut self) {
        if self.flags == 0 {
            return;
        }

        if self.low == 0.0 && (self.flags & QuantizedFloatFlags::RoundDown as u32) != 0
            || self.high == 0.0 && (self.flags & QuantizedFloatFlags::RoundUp as u32) != 0
        {
            self.flags &= !(QuantizedFloatFlags::EncodeZero as u32);
        }

        if self.low == 0.0 && (self.flags & QuantizedFloatFlags::EncodeZero as u32) != 0 {
            self.flags |= QuantizedFloatFlags::RoundUp as u32;
            self.flags &= !(QuantizedFloatFlags::EncodeZero as u32);
        }

        if self.high == 0.0 && (self.flags & QuantizedFloatFlags::EncodeZero as u32) != 0 {
            self.flags |= QuantizedFloatFlags::RoundDown as u32;
            self.flags &= !(QuantizedFloatFlags::EncodeZero as u32);
        }

        if self.low > 0.0 || self.high < 0.0 {
            self.flags &= !(QuantizedFloatFlags::EncodeZero as u32);
        }

        if self.flags & (QuantizedFloatFlags::EncodeInteger as u32) != 0 {
            self.flags &= !(QuantizedFloatFlags::RoundUp as u32
                | QuantizedFloatFlags::RoundDown as u32
                | QuantizedFloatFlags::EncodeZero as u32);
        }

        debug_assert!(
            self.flags
                & (QuantizedFloatFlags::RoundDown as u32 | QuantizedFloatFlags::RoundUp as u32)
                != (QuantizedFloatFlags::RoundDown as u32 | QuantizedFloatFlags::RoundUp as u32),
            "Roundup / Rounddown are mutually exclusive"
        )
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

        debug_assert!(
            self.high_low_mul != 0.0,
            "Error computing high / low multiplier"
        )
    }

    pub(crate) fn quantize(&self, v: f32) -> f32 {
        if v < self.low {
            debug_assert!(
                self.flags & QuantizedFloatFlags::RoundUp as u32 != 0,
                "Field tried to quantize an out of range value"
            );
            return self.low;
        }

        if v > self.high {
            debug_assert!(
                self.flags & QuantizedFloatFlags::RoundDown as u32 != 0,
                "Field tried to quantize an out of range value"
            );
            return self.high;
        }

        let i = ((v - self.low) * self.high_low_mul) as u32;

        self.low + (self.high - self.low) * i as f32 * self.dec_mul
    }

    pub(crate) fn decode(&self, reader: &mut Reader) -> f32 {
        reader.refill();

        if self.flags & (QuantizedFloatFlags::RoundDown as u32) != 0 && reader.read_bool() {
            return self.low;
        }

        if self.flags & (QuantizedFloatFlags::RoundUp as u32) != 0 && reader.read_bool() {
            return self.high;
        }

        if self.flags & (QuantizedFloatFlags::EncodeZero as u32) != 0 && reader.read_bool() {
            return 0.0;
        }

        self.low
            + (self.high - self.low)
                * (reader.read_bits_no_refill(self.bit_count) as f32)
                * self.dec_mul
    }
}
