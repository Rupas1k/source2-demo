use crate::reader::Reader;
use crate::entitiy::EntityFieldType;
use crate::field::Field;
use crate::qfloat::QFloatDecoder;

#[derive(Clone, Debug)]
pub enum Decoders {
    VectorNormal,
    Fixed64,
    Handle,
    Boolean,
    String,
    Default,
    Signed,
    FloatCoordinate,
    NoScale,
    RuneTime,
    SimulationTime,
    Unsigned,
    Component,

    Vector(FieldValues, u8),
    Unsigned64(FieldValues),
    Float32(FieldValues),
    QuantizedFloat(FieldValues),
    QAngle(FieldValues),
}


// TODO: Rewrite without cloning field values
#[derive(Clone, Debug)]
pub struct FieldValues {
    encoder: Box<str>,
    encoder_flags: Option<i32>,
    bit_count: Option<i32>,
    low_value: Option<f32>,
    high_value: Option<f32>
}


impl Decoders {

    // From<&Field> trait?
    pub fn from_field(field: &Field, generic: bool) -> Self {
        let field_values = FieldValues {
            encoder: field.encoder.clone(),
            encoder_flags: field.encoder_flags,
            bit_count: field.bit_count,
            high_value: field.high_value,
            low_value: field.low_value,
        };

        let match_var = match generic {
            true => field.field_type.as_ref().unwrap().generic.as_ref().unwrap().base.as_str(),
            false => field.field_type.as_ref().unwrap().base.as_str()
        };

        match match_var {
            "bool" =>                                                       Decoders::Boolean,
            "char" | "CUtlString" | "CUtlSymbolLarge" =>                    Decoders::String,
            "int8" | "int16" | "int32" | "int64" =>                         Decoders::Signed,
            "uint8" | "uint16" | "uint32" | "color32" |
            "CGameSceneNodeHandle" | "Color" | "CUtlStringToken" |
            "CHandle" | "CEntityHandle" =>                                  Decoders::Unsigned,
            "GameTime_t" =>                                                 Decoders::NoScale,
            "CBodyComponent" | "CPhysicsComponent" | "CRenderComponent" =>  Decoders::Component,

            "CNetworkedQuantizedFloat" => Decoders::QuantizedFloat(field_values),

            "float32" => Decoders::Float32(field_values),

            "Vector" => Decoders::Vector(field_values, 3),
            "Vector2D" => Decoders::Vector(field_values, 2),
            "Vector4D" => Decoders::Vector(field_values, 4),

            "uint64" | "CStrongHandle" => Decoders::Unsigned64(field_values),

            "QAngle" => Decoders::QAngle(field_values),

            _ => { Decoders::Default }
        }
    }

    pub fn decode(&self, reader: &mut Reader) -> EntityFieldType {
        match self {
            Decoders::VectorNormal => {
                EntityFieldType::Vector3D(reader.read_3bit_normal())
            }
            Decoders::Fixed64 => {
                EntityFieldType::Unsigned(reader.read_le_u64())
            }
            Decoders::Handle => {
                EntityFieldType::Unsigned(reader.read_var_u32() as u64)
            }
            Decoders::Boolean => {
                EntityFieldType::Boolean(reader.read_bool())
            }
            Decoders::String => {
                EntityFieldType::String(reader.read_string().unwrap())
            }
            Decoders::Default => {
                EntityFieldType::Unsigned(reader.read_var_u32() as u64)
            }
            Decoders::Signed => {
                EntityFieldType::Signed(reader.read_var_i32() as i64)
            }
            Decoders::FloatCoordinate => {
                EntityFieldType::Float(reader.read_coordinate())
            }
            Decoders::NoScale => {
                EntityFieldType::Float(reader.read_f32())
            }
            Decoders::RuneTime => {
                EntityFieldType::Float(f32::from_bits(reader.read_bits(4)))
            }
            Decoders::SimulationTime => {
                EntityFieldType::Float(reader.read_var_u32() as f32 * (1.0 / 30.0))
            }
            Decoders::Unsigned => {
                EntityFieldType::Unsigned(reader.read_var_u32() as u64)
            }
            Decoders::Component => {
                EntityFieldType::Unsigned(reader.read_bit() as u64)
            }
            Decoders::Float32(fv) => {
                match fv.encoder.as_ref() {
                    "coord" => { return Decoders::FloatCoordinate.decode(reader); }
                    "simtime" => { return Decoders::SimulationTime.decode(reader); }
                    "runetime" => { return Decoders::RuneTime.decode(reader); }
                    _ => {
                        if fv.bit_count.is_none() || (fv.bit_count.unwrap() <= 0 || fv.bit_count.unwrap() >= 32) {
                            return Decoders::NoScale.decode(reader);
                        }
                        return Decoders::QuantizedFloat(fv.clone()).decode(reader);
                    }
                };
            }
            Decoders::Vector(fv, n) => {
                if *n == 2 {
                    let mut r = [0.0f32; 2];
                    r[0] = Decoders::Float32(fv.clone()).decode(reader).as_float();
                    r[1] = Decoders::Float32(fv.clone()).decode(reader).as_float();
                    return EntityFieldType::Vector2D(r)
                }
                if *n == 3 {
                    if fv.encoder.as_ref() == "normal" {
                        return Decoders::VectorNormal.decode(reader)
                    }
                    let mut r = [0.0f32; 3];
                    r[0] = Decoders::Float32(fv.clone()).decode(reader).as_float();
                    r[1] = Decoders::Float32(fv.clone()).decode(reader).as_float();
                    r[2] = Decoders::Float32(fv.clone()).decode(reader).as_float();
                    return EntityFieldType::Vector3D(r)
                }
                if *n == 4 {
                    let mut r = [0.0f32; 4];
                    r[0] = Decoders::Float32(fv.clone()).decode(reader).as_float();
                    r[1] = Decoders::Float32(fv.clone()).decode(reader).as_float();
                    r[2] = Decoders::Float32(fv.clone()).decode(reader).as_float();
                    r[3] = Decoders::Float32(fv.clone()).decode(reader).as_float();
                    return EntityFieldType::Vector4D(r)
                }
                panic!("Unsupported size");
            }
            Decoders::Unsigned64(fv) => {
                if fv.encoder.as_ref() == "fixed64" {
                    return Decoders::Fixed64.decode(reader);
                }
                return EntityFieldType::Unsigned(reader.read_var_u64());
            }
            Decoders::QuantizedFloat(fv) => {
                let qd = QFloatDecoder::new(fv.bit_count.unwrap(), fv.encoder_flags, fv.low_value, fv.high_value);
                EntityFieldType::Float(qd.decode(reader))
            }
            Decoders::QAngle(fv) => {
                if fv.encoder.as_ref() == "qangle_pitch_yaw" {
                    let n = fv.bit_count.unwrap() as u32;
                    return EntityFieldType::Vector3D([reader.read_angle(n), reader.read_angle(n), 0.0]);
                }

                if fv.bit_count.is_some() && fv.bit_count.unwrap() != 0 {
                    let n = fv.bit_count.unwrap() as u32;
                    return EntityFieldType::Vector3D([reader.read_angle(n), reader.read_angle(n), reader.read_angle(n)]);
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
                EntityFieldType::Vector3D(v)
            }
        }
    }
}