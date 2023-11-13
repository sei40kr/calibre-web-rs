use uuid::Uuid;

pub struct Tenant {
    pub uuid: Uuid,
    pub external_uid: String,
}
