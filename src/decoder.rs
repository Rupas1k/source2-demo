use crate::entity::FieldValue;
use crate::field::Field;
use crate::utils::{QFloatDecoder, Reader};

#[derive(Clone, Debug)]
pub enum Decoders {
    VectorNormal,
    Fixed64,
    Handle,
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

#[derive(Clone, Debug)]
pub struct FieldProperties {
    encoder: Box<str>,
    encoder_flags: i32,
    bit_count: i32,
    low_value: f32,
    high_value: f32,
}

impl Decoders {
    pub fn from_field(field: &Field, generic: bool) -> Self {
        let field_properties = FieldProperties {
            encoder: field.encoder.to_string().into_boxed_str(),
            encoder_flags: field.encoder_flags,
            bit_count: field.bit_count,
            high_value: field.high_value,
            low_value: field.low_value,
        };

        let match_var = match generic {
            true => field.field_type.generic.as_ref().unwrap().base.as_ref(),
            false => field.field_type.base.as_ref(),
        };

        match match_var {
            "bool" => Decoders::Boolean,
            "char" | "CUtlString" | "CUtlSymbolLarge" => Decoders::String,
            "int8" => Decoders::Signed8,
            "int16" => Decoders::Signed16,
            "int32" => Decoders::Signed32,
            "int64" => Decoders::Signed64,
            "uint8" => Decoders::Signed8,
            "uint16" => Decoders::Signed16,
            "uint32"
            | "color32"
            | "CGameSceneNodeHandle"
            | "Color"
            | "CUtlStringToken"
            | "CHandle"
            | "CEntityHandle" => Decoders::Unsigned32,
            "GameTime_t" => Decoders::NoScale,
            "CBodyComponent" | "CPhysicsComponent" | "CRenderComponent" => Decoders::Component,

            "CNetworkedQuantizedFloat" => Decoders::QuantizedFloat(field_properties),

            "float32" => Decoders::Float32(field_properties),

            "Vector" => Decoders::Vector(field_properties, 3),
            "Vector2D" => Decoders::Vector(field_properties, 2),
            "Vector4D" => Decoders::Vector(field_properties, 4),

            "uint64" | "CStrongHandle" => Decoders::Unsigned64(field_properties),

            "QAngle" => Decoders::QAngle(field_properties),

            _ => Decoders::Default,
        }
    }

    pub(crate) fn decode(&self, reader: &mut Reader) -> FieldValue {
        match self {
            Decoders::VectorNormal => FieldValue::Vector3D(reader.read_3bit_normal()),
            Decoders::Fixed64 => FieldValue::Unsigned64(reader.read_le_u64()),
            Decoders::Handle => FieldValue::Unsigned32(reader.read_var_u32()),
            Decoders::Boolean => FieldValue::Boolean(reader.read_bool()),
            Decoders::String => FieldValue::String(reader.read_string().unwrap()),
            Decoders::Default => FieldValue::Unsigned32(reader.read_var_u32()),
            Decoders::Signed8 => FieldValue::Signed8(reader.read_var_i32() as i8),
            Decoders::Signed16 => FieldValue::Signed16(reader.read_var_i32() as i16),
            Decoders::Signed32 => FieldValue::Signed32(reader.read_var_i32()),
            Decoders::Signed64 => FieldValue::Signed64(reader.read_var_i32() as i64),
            Decoders::FloatCoordinate => FieldValue::Float(reader.read_coordinate()),
            Decoders::NoScale => FieldValue::Float(reader.read_f32()),
            Decoders::RuneTime => FieldValue::Float(f32::from_bits(reader.read_bits(4))),
            Decoders::SimulationTime => {
                FieldValue::Float(reader.read_var_u32() as f32 * (1.0 / 30.0))
            }
            Decoders::Unsigned8 => FieldValue::Unsigned8(reader.read_var_u32() as u8),
            Decoders::Unsigned16 => FieldValue::Unsigned16(reader.read_var_u32() as u16),
            Decoders::Unsigned32 => FieldValue::Unsigned32(reader.read_var_u32()),
            Decoders::Component => FieldValue::Boolean(reader.read_bool()),
            Decoders::Float32(fp) => match fp.encoder.as_ref() {
                "coord" => Decoders::FloatCoordinate.decode(reader),
                "simtime" => Decoders::SimulationTime.decode(reader),
                "runetime" => Decoders::RuneTime.decode(reader),
                _ => {
                    if fp.bit_count <= 0 || fp.bit_count >= 32 {
                        return Decoders::NoScale.decode(reader);
                    }
                    Decoders::QuantizedFloat(fp.clone()).decode(reader)
                }
            },
            Decoders::Vector(fp, n) => {
                if *n == 2 {
                    return FieldValue::Vector2D([
                        Decoders::Float32(fp.clone()).decode(reader).as_float(),
                        Decoders::Float32(fp.clone()).decode(reader).as_float(),
                    ]);
                }
                if *n == 3 {
                    if fp.encoder.as_ref() == "normal" {
                        return Decoders::VectorNormal.decode(reader);
                    }
                    return FieldValue::Vector3D([
                        Decoders::Float32(fp.clone()).decode(reader).as_float(),
                        Decoders::Float32(fp.clone()).decode(reader).as_float(),
                        Decoders::Float32(fp.clone()).decode(reader).as_float(),
                    ]);
                }
                if *n == 4 {
                    return FieldValue::Vector4D([
                        Decoders::Float32(fp.clone()).decode(reader).as_float(),
                        Decoders::Float32(fp.clone()).decode(reader).as_float(),
                        Decoders::Float32(fp.clone()).decode(reader).as_float(),
                        Decoders::Float32(fp.clone()).decode(reader).as_float(),
                    ]);
                }
                panic!("Unsupported size");
            }
            Decoders::Unsigned64(fp) => {
                if fp.encoder.as_ref() == "fixed64" {
                    return Decoders::Fixed64.decode(reader);
                }
                FieldValue::Unsigned64(reader.read_var_u64())
            }
            Decoders::QuantizedFloat(fp) => {
                let qd =
                    QFloatDecoder::new(fp.bit_count, fp.encoder_flags, fp.low_value, fp.high_value);
                FieldValue::Float(qd.decode(reader))
            }
            Decoders::QAngle(fp) => {
                if fp.encoder.as_ref() == "qangle_pitch_yaw" {
                    let n = fp.bit_count as u32;
                    return FieldValue::Vector3D([reader.read_angle(n), reader.read_angle(n), 0.0]);
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
