use crate::field::Field;

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

pub static FIELD_PATCHES: [FieldPatch; 4] = [
    FieldPatch {
        min_build: 0,
        max_build: 990,
        patch: |f: &mut Field| match f.var_name.as_str() {
            "angExtraLocalAngles"
            | "angLocalAngles"
            | "m_angInitialAngles"
            | "m_angRotation"
            | "m_ragAngles"
            | "m_vLightDirection" => {
                f.encoder = if f.parent.as_ref().unwrap() == &"CBodyComponentBaseAnimatingOverlay" {
                    "qangle_pitch_yaw".to_string()
                } else {
                    "QAngle".to_string()
                };
            }
            "dirPrimary"
            | "localSound"
            | "m_flElasticity"
            | "m_location"
            | "m_poolOrigin"
            | "m_ragPos"
            | "m_vecEndPos"
            | "m_vecLadderDir"
            | "m_vecPlayerMountPositionBottom"
            | "m_vecPlayerMountPositionTop"
            | "m_viewtarget"
            | "m_WorldMaxs"
            | "m_WorldMins"
            | "origin"
            | "vecLocalOrigin" => {
                f.encoder = "coord".to_string();
            }
            "m_vecLadderNormal" => {
                f.encoder = "normal".to_string();
            }
            _ => {}
        },
    },
    FieldPatch {
        min_build: 0,
        max_build: 954,
        patch: |f: &mut Field| match f.var_name.as_str() {
            "m_flMana" | "m_flMaxMana" => {
                f.low_value = Some(0.0);
                f.high_value = Some(8192.0f32);
            }
            _ => {}
        },
    },
    FieldPatch {
        min_build: 1016,
        max_build: 1027,
        patch: |f: &mut Field| match f.var_name.as_str() {
            "m_bItemWhiteList"
            | "m_bWorldTreeState"
            | "m_iPlayerIDsInControl"
            | "m_iPlayerSteamID"
            | "m_ulTeamBannerLogo"
            | "m_ulTeamBaseLogo"
            | "m_ulTeamLogo" => {
                f.encoder = "fixed64".to_string();
            }
            _ => {}
        },
    },
    FieldPatch {
        min_build: 0,
        max_build: 0,
        patch: |f: &mut Field| match f.var_name.as_str() {
            "m_flSimulationTime" | "m_flAnimTime" => {
                f.encoder = "simtime".to_string();
            }
            "m_flRuneTime" => {
                f.encoder = "runetime".to_string();
            }
            _ => {}
        },
    },
];