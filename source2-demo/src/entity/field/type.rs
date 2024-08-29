#[derive(Clone, Debug)]
pub(crate) struct FieldType {
    pub base: Box<str>,
    pub generic: Option<Box<FieldType>>,
    pub pointer: bool,
    pub count: Option<i32>,
}

impl FieldType {
    pub fn new(name: &str) -> Self {
        let mut base_end = name.len();
        let mut pointer = false;
        let mut count = None;
        let mut generic = None;

        if name.ends_with('*') {
            pointer = true;
            base_end -= 1;
        } else if [
            "PhysicsRagdollPose_t",
            "CBodyComponent",
            "CEntityIdentity",
            "CPhysicsComponent",
            "CRenderComponent",
            "CDOTAGamerules",
            "CDOTAGameManager",
            "CDOTASpectatorGraphManager",
            "CPlayerLocalData",
            "CPlayer_CameraServices",
            "CDOTAGameRules",
        ]
        .contains(&name)
        {
            pointer = true;
        }

        if let Some(open_bracket_pos) = name.find('[') {
            let close_bracket_pos = name.find(']').unwrap();
            count = match &name[(open_bracket_pos + 1)..close_bracket_pos] {
                "MAX_ITEM_STOCKS" => Some(8),
                "MAX_ABILITY_DRAFT_ABILITIES" => Some(48),
                s => Some(s.parse::<i32>().unwrap()),
            };
            base_end = open_bracket_pos;
        }

        if let Some(open_angle_pos) = name.find('<') {
            let close_angle_pos = name.rfind('>').unwrap();
            generic = Some(Box::new(FieldType::new(
                name[(open_angle_pos + 1)..close_angle_pos].trim(),
            )));
            base_end = open_angle_pos;
        }

        let base = name[..base_end].trim().to_string().into_boxed_str();

        FieldType {
            base,
            generic,
            pointer,
            count,
        }
    }

    pub fn as_string(&self) -> String {
        let mut x = self.base.to_string();
        if let Some(generic) = &self.generic {
            x = x + "< " + &generic.as_string() + " >";
        }
        if self.pointer {
            x += "*";
        }
        if let Some(c) = self.count {
            x = x + "[" + &c.to_string() + "]";
        }
        x
    }
}
