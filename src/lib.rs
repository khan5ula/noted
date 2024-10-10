pub mod db;
pub mod helpers;
pub mod note;

pub enum SortOrder {
    Asc,
    Desc,
}

impl SortOrder {
    pub fn as_str(&self) -> &str {
        match self {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        }
    }
}
