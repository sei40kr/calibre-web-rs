use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

pub struct Author {
    pub uuid: Uuid,
    pub name: String,
    pub name_sort: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Author {
    pub fn new(
        uuid: Uuid,
        name: String,
        name_sort: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            uuid,
            name,
            name_sort,
            created_at,
            updated_at,
        }
    }
}

pub struct Tag {
    pub uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tag {
    pub fn new(
        uuid: Uuid,
        name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            uuid,
            name,
            created_at,
            updated_at,
        }
    }
}

pub struct Publisher {
    pub uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Publisher {
    pub fn new(
        uuid: Uuid,
        name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            uuid,
            name,
            created_at,
            updated_at,
        }
    }
}

pub struct Series {
    pub uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Series {
    pub fn new(
        uuid: Uuid,
        name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            uuid,
            name,
            created_at,
            updated_at,
        }
    }
}

pub struct Language {
    pub uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Language {
    pub fn new(
        uuid: Uuid,
        name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            uuid,
            name,
            created_at,
            updated_at,
        }
    }
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

impl Identifier {
    pub fn new(
        identifier_type: IdentifierType,
        value: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            identifier_type,
            value,
            created_at,
            updated_at,
        }
    }
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
    pub publication_date: Option<NaiveDate>,
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

pub struct BookCursor {
    pub uuid: Uuid,
    pub title: String,
    pub title_sort: Option<String>,
    pub rating: Option<Rating>,
    pub publication_date: Option<NaiveDate>,
}

impl Book {
    pub fn new(
        uuid: Uuid,
        title: String,
        title_sort: Option<String>,
        authors: Vec<Author>,
        series: Vec<Series>,
        series_index: Option<f32>,
        publication_date: Option<NaiveDate>,
        tags: Vec<Tag>,
        publishers: Vec<Publisher>,
        languages: Vec<Language>,
        identifier: Option<Identifier>,
        rating: Option<Rating>,
        comment: String,
        has_cover: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            uuid,
            title,
            title_sort,
            authors,
            series,
            series_index,
            publication_date,
            tags,
            publishers,
            languages,
            identifier,
            rating,
            comment,
            has_cover,
            created_at,
            updated_at,
        }
    }
}

impl Into<BookCursor> for Book {
    fn into(self) -> BookCursor {
        BookCursor {
            uuid: self.uuid,
            title: self.title,
            title_sort: self.title_sort,
            rating: self.rating,
            publication_date: self.publication_date,
        }
    }
}
