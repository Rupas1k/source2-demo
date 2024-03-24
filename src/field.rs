use crate::field_decoder::Decoders;
use crate::field_path::FieldPath;
use crate::field_state::{FieldState, States};
use crate::field_type::FieldType;
use crate::serializer::Serializer;
use anyhow::bail;
use anyhow::Result;
use std::rc::Rc;
use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct Field {
    pub var_name: Box<str>,
    pub var_type: Box<str>,
    pub serializer_name: Box<str>,
    pub encoder: Box<str>,
    pub encoder_flags: i32,
    pub bit_count: i32,
    pub low_value: f32,
    pub high_value: f32,
    pub field_type: Rc<FieldType>,
    pub serializer: Option<Rc<Serializer>>,
    pub model: FieldModels,

    pub decoder: Decoders,
    pub base_decoder: Decoders,
    pub child_decoder: Decoders,
}

impl Field {
    pub fn get_name_for_field_path(&self, fp: &FieldPath, pos: i32) -> Vec<String> {
        let mut x = vec![self.var_name.as_ref().into()];

        match self.model {
            FieldModels::Simple | FieldModels::FixedArray | FieldModels::VariableArray => {
                if fp.last == pos as usize {
                    x.push(format!("{:04}", fp.path[pos as usize]));
                }
            }
            FieldModels::FixedTable | FieldModels::VariableTable => {
                if fp.last >= pos as usize {
                    x.extend_from_slice(
                        &self
                            .serializer
                            .as_ref()
                            .unwrap()
                            .get_name_for_field_path(fp, pos),
                    );
                }
                if self.model == FieldModels::VariableTable && fp.last != (pos - 1) as usize {
                    x.push(format!("{:04}", fp.path[pos as usize]));
                    if fp.last != pos as usize {
                        x.extend_from_slice(
                            &self
                                .serializer
                                .as_ref()
                                .unwrap()
                                .get_name_for_field_path(fp, pos + 1),
                        )
                    }
                }
            }
        };

        x
    }

    pub fn get_type_for_field_path(&self, fp: &FieldPath, pos: i32) -> &FieldType {
        match self.model {
            FieldModels::Simple => {}
            FieldModels::FixedArray => {
                return self.field_type.as_ref();
            }
            FieldModels::FixedTable => {
                if fp.last as i32 != pos - 1 {
                    return self
                        .serializer
                        .as_ref()
                        .unwrap()
                        .get_type_for_field_path(fp, pos);
                }
            }
            FieldModels::VariableArray => {
                if fp.last as i32 == pos {
                    return self.field_type.as_ref().generic.as_ref().unwrap();
                }
            }
            FieldModels::VariableTable => {
                if fp.last as i32 > pos {
                    return self
                        .serializer
                        .as_ref()
                        .unwrap()
                        .get_type_for_field_path(fp, pos + 1);
                }
            }
        };
        self.field_type.as_ref()
    }

    pub fn get_decoder_for_field_path(&self, fp: &FieldPath, pos: i32) -> &Decoders {
        match self.model {
            FieldModels::Simple => {}
            FieldModels::FixedArray => {
                return &self.decoder;
            }
            FieldModels::FixedTable => {
                if fp.last as i32 == pos - 1 {
                    return &self.base_decoder;
                }
                return self
                    .serializer
                    .as_ref()
                    .unwrap()
                    .get_decoder_for_field_path(fp, pos);
            }
            FieldModels::VariableArray => {
                if fp.last as i32 == pos {
                    return &self.child_decoder;
                }
                return &self.base_decoder;
            }
            FieldModels::VariableTable => {
                if fp.last as i32 > pos {
                    return self
                        .serializer
                        .as_ref()
                        .unwrap()
                        .get_decoder_for_field_path(fp, pos + 1);
                }
                return &self.base_decoder;
            }
        }
        &self.decoder
    }

    pub fn get_field_path_for_name(&self, fp: &mut FieldPath, name: &str) -> Result<()> {
        match self.model {
            FieldModels::Simple => {
                bail!("not supported");
            }
            FieldModels::FixedArray => {
                fp.path[fp.last] = name.parse::<i32>().unwrap();
                return Ok(());
            }
            FieldModels::FixedTable => {
                return self
                    .serializer
                    .as_ref()
                    .unwrap()
                    .get_field_path_for_name(fp, name)
            }
            FieldModels::VariableArray => {
                fp.path[fp.last] = name.parse::<i32>().unwrap();
            }
            FieldModels::VariableTable => {
                fp.path[fp.last] = name[0..4].parse::<i32>().unwrap();
                fp.last += 1;
                return self
                    .serializer
                    .as_ref()
                    .unwrap()
                    .get_field_path_for_name(fp, &name[5..]);
            }
        }
        bail!("No field path for given name")
    }

    pub fn get_field_paths(&self, fp: &mut FieldPath, st: &FieldState) -> Vec<FieldPath> {
        let mut vec: Vec<FieldPath> = vec![];
        match self.model {
            FieldModels::Simple => {
                vec.push(fp.clone());
            }
            FieldModels::FixedArray => {
                if let Some(States::FieldState(s)) = st.get(fp) {
                    fp.last += 1;
                    for (i, v) in s.state.iter().enumerate() {
                        fp.path[fp.last] = i as i32;
                        vec.push(fp.clone());
                    }
                    fp.pop(1);
                }
            }
            FieldModels::FixedTable => {
                if let Some(States::FieldState(v)) = st.get(fp) {
                    fp.last += 1;
                    vec.extend_from_slice(
                        &self.serializer.as_ref().unwrap().get_field_paths(fp, v),
                    );
                    fp.pop(1);
                }
            }
            FieldModels::VariableArray => {
                if let Some(States::FieldState(x)) = st.get(fp) {
                    fp.last += 1;
                    for (i, _) in x.state.iter().enumerate() {
                        fp.path[fp.last] = i as i32;
                        vec.push(fp.clone());
                    }
                    fp.pop(1);
                }
            }
            FieldModels::VariableTable => {
                if let Some(States::FieldState(x)) = st.get(fp) {
                    fp.last += 2;
                    for (i, v) in x.state.iter().enumerate() {
                        if let Some(States::FieldState(vv)) = v.as_ref() {
                            fp.path[fp.last - 1] = i as i32;
                            vec.extend_from_slice(
                                &self.serializer.as_ref().unwrap().get_field_paths(fp, vv),
                            );
                        }
                    }
                    fp.pop(2);
                }
            }
        }
        vec
    }

    // pub fn set_model(&mut self, model: FieldModels) {
    //     self.model = model;
    //     match self.model {
    //         FieldModels::FixedArray => {
    //             self.decoder = Decoders::from_field(self, false);
    //         }
    //         FieldModels::FixedTable => self.base_decoder = Decoders::Boolean,
    //         FieldModels::VariableArray => {
    //             self.base_decoder = Decoders::Unsigned32;
    //             self.child_decoder = Decoders::from_field(self, true)
    //         }
    //         FieldModels::VariableTable => {
    //             self.base_decoder = Decoders::Unsigned32;
    //         }
    //         FieldModels::Simple => {
    //             self.decoder = Decoders::from_field(self, false);
    //         }
    //     }
    // }

    pub fn get_name(&self) -> &str {
        &self.var_name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldModels {
    Simple,
    FixedArray,
    FixedTable,
    VariableArray,
    VariableTable,
}

impl FieldModels {
    pub fn as_str(&self) -> &str {
        match &self {
            FieldModels::Simple => "fixed-array",
            FieldModels::FixedArray => "fixed-table",
            FieldModels::FixedTable => "variable-array",
            FieldModels::VariableArray => "variable-table",
            FieldModels::VariableTable => "simple",
        }
    }
}

pub struct FieldPatch {
    min_build: u32,
    max_build: u32,
    pub patch: fn(&mut Field),
}

impl FieldPatch {
    pub fn should_apply(&self, build: u32) -> bool {
        if self.min_build == 0 && self.max_build == 0 {
            true
        } else {
            build >= self.min_build && build <= self.max_build
        }
    }
}
// pub static FIELD_PATCHES: [FieldPatch; 0] = [];

lazy_static!{
pub static ref FIELD_PATCHES: [FieldPatch; 1] = [
    // FieldPatch {
    //     min_build: 0,
    //     max_build: 990,
    //     patch: |f: &mut Field| match f.var_name.as_ref() {
    //         "angExtraLocalAngles"
    //         | "angLocalAngles"
    //         | "m_angInitialAngles"
    //         | "m_angRotation"
    //         | "m_ragAngles"
    //         | "m_vLightDirection" => {
    //             f.encoder = Box::from(if f.parent.as_ref() == "CBodyComponentBaseAnimatingOverlay" {
    //                 "qangle_pitch_yaw".to_string()
    //             } else {
    //                 "QAngle".to_string()
    //             });
    //         }
    //         "dirPrimary"
    //         | "localSound"
    //         | "m_flElasticity"
    //         | "m_location"
    //         | "m_poolOrigin"
    //         | "m_ragPos"
    //         | "m_vecEndPos"
    //         | "m_vecLadderDir"
    //         | "m_vecPlayerMountPositionBottom"
    //         | "m_vecPlayerMountPositionTop"
    //         | "m_viewtarget"
    //         | "m_WorldMaxs"
    //         | "m_WorldMins"
    //         | "origin"
    //         | "vecLocalOrigin" => {
    //             f.encoder = "coord".to_string().into_boxed_str();
    //         }
    //         "m_vecLadderNormal" => {
    //             f.encoder = "normal".to_string().into_boxed_str();
    //         }
    //         _ => {}
    //     },
    // },
    // FieldPatch {
    //     min_build: 0,
    //     max_build: 954,
    //     patch: |f: &mut Field| match f.var_name.as_ref() {
    //         "m_flMana" | "m_flMaxMana" => {
    //             f.low_value = 0.0;
    //             f.high_value = 8192.0f32;
    //         }
    //         _ => {}
    //     },
    // },
    // FieldPatch {
    //     min_build: 1016,
    //     max_build: 1027,
    //     patch: |f: &mut Field| match f.var_name.as_ref() {
    //         "m_bItemWhiteList"
    //         | "m_bWorldTreeState"
    //         | "m_iPlayerIDsInControl"
    //         | "m_iPlayerSteamID"
    //         | "m_ulTeamBannerLogo"
    //         | "m_ulTeamBaseLogo"
    //         | "m_ulTeamLogo" => {
    //             f.encoder = "fixed64".to_string().into_boxed_str();
    //         }
    //         _ => {}
    //     },
    // },
    FieldPatch {
        min_build: 0,
        max_build: 0,
        patch: |f: &mut Field| match f.var_name.as_ref() {
            "m_flSimulationTime" | "m_flAnimTime" => {
                f.encoder = "simtime".into();
            }
            "m_flRuneTime" => {
                f.encoder = "runetime".into();
            }
            _ => {}
        },
    },
];
}
