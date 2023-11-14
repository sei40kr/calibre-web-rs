use uuid::Uuid;

pub struct Tenant {
    pub uuid: Uuid,
    pub external_uid: String,
}

impl Tenant {
    pub fn new(uuid: Uuid, external_uid: String) -> Self {
        Self { uuid, external_uid }
    }
}
