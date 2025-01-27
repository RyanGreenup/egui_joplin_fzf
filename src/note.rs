use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Note {
    pub title: String,
    pub body: String,
    pub id: String,
}

impl Note {
    pub fn random(title: &str, body: &str) -> Self {
        let title = title.into();
        let body = body.into();
        let rng = thread_rng();
        let id: String = rng
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        Self { title, body, id }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}
