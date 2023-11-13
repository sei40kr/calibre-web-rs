use chrono::{Date, DateTime, Utc};
use uuid::Uuid;

pub struct Author {
    pub uuid: Uuid,
    pub name: String,
    pub name_sort: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Tag {
    pub uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Publisher {
    pub uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Series {
    pub uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Language {
    pub uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum IdentifierType {
    Isbn,
    Lccn,
}

pub struct Identifier {
    pub identifier_type: IdentifierType,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum Rating {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

pub struct Book {
    pub uuid: Uuid,
    pub title: String,
    pub title_sort: Option<String>,
    pub authors: Vec<Author>,
    pub series: Vec<Series>,
    pub series_index: Option<f32>,
    pub publication_date: Option<Date<Utc>>,
    pub tags: Vec<Tag>,
    pub publishers: Vec<Publisher>,
    pub languages: Vec<Language>,
    pub identifier: Option<Identifier>,
    pub rating: Option<Rating>,
    pub comment: String,
    pub has_cover: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
