use crate::field::{Encoder, FieldProperties, FieldType};
use crate::field_value::FieldValue;
use crate::reader::Reader;

pub enum Decoders {
    VectorNormal,
    Fixed64,
    Boolean,
    String,
    Default,
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

impl Decoders {
    #[inline(always)]
    pub fn from_field(field_type: &FieldType, properties: FieldProperties) -> Self {
        match field_type.base.as_ref() {
            "bool" => Decoders::Boolean,
            "char" | "CUtlString" | "CUtlSymbolLarge" => Decoders::String,
            "int8" => Decoders::Signed8,
            "int16" => Decoders::Signed16,
            "int32" => Decoders::Signed32,
            "int64" => Decoders::Signed64,
            "uint8" | "BloodType" => Decoders::Unsigned8,
            "uint16" => Decoders::Unsigned16,
            "uint32"
            | "color32"
            | "CGameSceneNodeHandle"
            | "Color"
            | "CUtlStringToken"
            | "CHandle"
            | "CEntityHandle" => Decoders::Unsigned32,
            "GameTime_t" => Decoders::NoScale,
            "CBodyComponent" | "CPhysicsComponent" | "CRenderComponent" => Decoders::Component,

            "CNetworkedQuantizedFloat" => Decoders::QuantizedFloat(properties),

            "float32" => Decoders::Float32(properties),

            "Vector" => Decoders::Vector(properties, 3),
            "Vector2D" => Decoders::Vector(properties, 2),
            "Vector4D" => Decoders::Vector(properties, 4),

            "uint64" | "CStrongHandle" | "HeroFacetKey_t" => Decoders::Unsigned64(properties),

            "QAngle" => Decoders::QAngle(properties),

            _ => Decoders::Default,
        }
    }

    #[inline(always)]
    pub(crate) fn decode(&self, reader: &mut Reader) -> FieldValue {
        match self {
            Decoders::VectorNormal => FieldValue::Vector3D(reader.read_3bit_normal()),
            Decoders::Fixed64 => FieldValue::Unsigned64(reader.read_le_u64()),
            Decoders::Boolean => FieldValue::Boolean({
                reader.refill();
                reader.read_bool()
            }),
            Decoders::String => FieldValue::String(reader.read_string()),
            Decoders::Default => FieldValue::Unsigned32(reader.read_var_u32()),
            Decoders::Signed8 => FieldValue::Signed8(reader.read_var_i32() as i8),
            Decoders::Signed16 => FieldValue::Signed16(reader.read_var_i32() as i16),
            Decoders::Signed32 => FieldValue::Signed32(reader.read_var_i32()),
            Decoders::Signed64 => FieldValue::Signed64(reader.read_var_i32() as i64),
            Decoders::FloatCoordinate => FieldValue::Float(reader.read_coordinate()),
            Decoders::NoScale => FieldValue::Float(reader.read_f32()),
            Decoders::RuneTime => FieldValue::Float(f32::from_bits(reader.read_bits(4))),
            Decoders::SimulationTime => FieldValue::Float(reader.read_var_u32() as f32 / 30.0),
            Decoders::Unsigned8 => FieldValue::Unsigned8(reader.read_var_u32() as u8),
            Decoders::Unsigned16 => FieldValue::Unsigned16(reader.read_var_u32() as u16),
            Decoders::Unsigned32 => FieldValue::Unsigned32(reader.read_var_u32()),
            Decoders::Component => FieldValue::Boolean({
                reader.refill();
                reader.read_bool()
            }),
            Decoders::Float32(fp) => match fp.encoder {
                Some(Encoder::Coord) => Decoders::FloatCoordinate.decode(reader),
                Some(Encoder::SimTime) => Decoders::SimulationTime.decode(reader),
                Some(Encoder::RuneTime) => Decoders::RuneTime.decode(reader),
                _ => {
                    if fp.bit_count <= 0 || fp.bit_count >= 32 {
                        return Decoders::NoScale.decode(reader);
                    }
                    Decoders::QuantizedFloat(*fp).decode(reader)
                }
            },
            Decoders::Vector(fp, n) => {
                if *n == 2 {
                    return FieldValue::Vector2D([
                        Decoders::Float32(*fp).decode(reader).as_float(),
                        Decoders::Float32(*fp).decode(reader).as_float(),
                    ]);
                }
                if *n == 3 {
                    if fp.encoder == Some(Encoder::Normal) {
                        return Decoders::VectorNormal.decode(reader);
                    }
                    return FieldValue::Vector3D([
                        Decoders::Float32(*fp).decode(reader).as_float(),
                        Decoders::Float32(*fp).decode(reader).as_float(),
                        Decoders::Float32(*fp).decode(reader).as_float(),
                    ]);
                }
                if *n == 4 {
                    return FieldValue::Vector4D([
                        Decoders::Float32(*fp).decode(reader).as_float(),
                        Decoders::Float32(*fp).decode(reader).as_float(),
                        Decoders::Float32(*fp).decode(reader).as_float(),
                        Decoders::Float32(*fp).decode(reader).as_float(),
                    ]);
                }
                unreachable!()
            }
            Decoders::Unsigned64(fp) => {
                if fp.encoder == Some(Encoder::Fixed64) {
                    return Decoders::Fixed64.decode(reader);
                }
                FieldValue::Unsigned64(reader.read_var_u64())
            }
            Decoders::QuantizedFloat(fp) => {
                FieldValue::Float(QFloatDecoder::new(fp).decode(reader))
            }
            Decoders::QAngle(fp) => {
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
    pub(crate) fn new(field_properties: &FieldProperties) -> Self {
        if field_properties.bit_count == 0 || field_properties.bit_count >= 32 {
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

        if decoder.flags & (QFloatFlags::RoundDown as u32) != 0 {
            decoder.offset = (decoder.high - decoder.low) / (steps as f32);
            decoder.high -= decoder.offset;
        } else if decoder.flags & (QFloatFlags::RoundUp as u32) != 0 {
            decoder.offset = (decoder.high - decoder.low) / (steps as f32);
            decoder.low += decoder.offset;
        }

        if decoder.flags & (QFloatFlags::EncodeInteger as u32) != 0 {
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
        if self.flags == 0 {
            return;
        }

        if self.low == 0.0 && (self.flags & QFloatFlags::RoundDown as u32) != 0
            || self.high == 0.0 && (self.flags & QFloatFlags::RoundUp as u32) != 0
        {
            self.flags &= !(QFloatFlags::EncodeZero as u32);
        }

        if self.low == 0.0 && (self.flags & QFloatFlags::EncodeZero as u32) != 0 {
            self.flags |= QFloatFlags::RoundUp as u32;
            self.flags &= !(QFloatFlags::EncodeZero as u32);
        }

        if self.high == 0.0 && (self.flags & QFloatFlags::EncodeZero as u32) != 0 {
            self.flags |= QFloatFlags::RoundDown as u32;
            self.flags &= !(QFloatFlags::EncodeZero as u32);
        }

        if self.low > 0.0 || self.high < 0.0 {
            self.flags &= !(QFloatFlags::EncodeZero as u32);
        }

        if self.flags & (QFloatFlags::EncodeInteger as u32) != 0 {
            self.flags &= !(QFloatFlags::RoundUp as u32
                | QFloatFlags::RoundDown as u32
                | QFloatFlags::EncodeZero as u32);
        }
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
    }

    pub(crate) fn quantize(&self, v: f32) -> f32 {
        if v < self.low {
            return self.low;
        }

        if v > self.high {
            return self.high;
        }

        let i = ((v - self.low) * self.high_low_mul) as u32;

        self.low + (self.high - self.low) * i as f32 * self.dec_mul
    }

    pub(crate) fn decode(&self, reader: &mut Reader) -> f32 {
        reader.refill();

        if self.flags & (QFloatFlags::RoundDown as u32) != 0 && reader.read_bool() {
            return self.low;
        }

        if self.flags & (QFloatFlags::RoundUp as u32) != 0 && reader.read_bool() {
            return self.high;
        }

        if self.flags & (QFloatFlags::EncodeZero as u32) != 0 && reader.read_bool() {
            return 0.0;
        }

        self.low
            + (self.high - self.low)
                * (reader.read_bits_no_refill(self.bit_count) as f32)
                * self.dec_mul
    }
}
