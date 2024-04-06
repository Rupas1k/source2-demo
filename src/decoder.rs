use crate::entity::FieldValue;
use crate::field::{Encoder, Field, FieldProperties};
use crate::quantized_float::QFloatDecoder;
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
    pub fn from_field(field: &Field, generic: bool) -> Self {
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
            "uint8" => Decoders::Unsigned8,
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

            "CNetworkedQuantizedFloat" => Decoders::QuantizedFloat(field.properties),

            "float32" => Decoders::Float32(field.properties),

            "Vector" => Decoders::Vector(field.properties, 3),
            "Vector2D" => Decoders::Vector(field.properties, 2),
            "Vector4D" => Decoders::Vector(field.properties, 4),

            "uint64" | "CStrongHandle" => Decoders::Unsigned64(field.properties),

            "QAngle" => Decoders::QAngle(field.properties),

            _ => Decoders::Default,
        }
    }

    pub(crate) fn decode(&self, reader: &mut Reader) -> FieldValue {
        match self {
            Decoders::VectorNormal => FieldValue::Vector3D(reader.read_3bit_normal()),
            Decoders::Fixed64 => FieldValue::Unsigned64(reader.read_le_u64()),
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
                panic!("Unsupported size");
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
