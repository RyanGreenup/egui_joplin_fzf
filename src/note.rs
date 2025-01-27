use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rusqlite::{Connection, Result as SqlResult};
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

    pub fn load_all(db_path: &str) -> SqlResult<Vec<Note>> {
        let conn = Connection::open(db_path)?;
        
        let mut stmt = conn.prepare("SELECT title, body, id FROM notes")?;
        let note_iter = stmt.query_map([], |row| {
            Ok(Note {
                title: row.get(0)?,
                body: row.get(1)?,
                id: row.get(2)?,
            })
        })?;

        let mut notes = Vec::new();
        for note in note_iter {
            notes.push(note?);
        }
        
        Ok(notes)
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}
