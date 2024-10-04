use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct Note {
    id: Uuid,
    content: String,
    date: u64,
}

impl Note {
    pub fn new(content: &str) -> Self {
        Note {
            id: Uuid::new_v4(),
            content: content.to_string(),
            date: Self::get_unix_time(),
        }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_date(&self) -> u64 {
        self.date
    }

    fn get_unix_time() -> u64 {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }
}
