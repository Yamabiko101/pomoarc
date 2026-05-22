use anyhow::Result;
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use crate::domain::{note::Note, session::SessionRecord, stats::StatsSummary, task::Task};

#[derive(Debug, Clone)]
pub struct JsonlStore {
    sessions_path: PathBuf,
}

impl JsonlStore {
    pub fn new(sessions_path: PathBuf) -> Self {
        Self { sessions_path }
    }

    pub fn tasks_path(&self) -> PathBuf {
        self.sessions_path.with_file_name("tasks.jsonl")
    }

    pub fn notes_path(&self) -> PathBuf {
        self.sessions_path.with_file_name("notes.jsonl")
    }

    pub fn append_session(&self, record: &SessionRecord) -> Result<()> {
        self.append_json(&self.sessions_path, record)
    }

    pub fn sessions(&self) -> Result<Vec<SessionRecord>> {
        self.read_jsonl(&self.sessions_path)
    }

    pub fn stats(&self) -> Result<StatsSummary> {
        Ok(StatsSummary::from_sessions(&self.sessions()?))
    }

    pub fn add_task(&self, task: &Task) -> Result<()> {
        self.append_json(&self.tasks_path(), task)
    }

    pub fn tasks(&self) -> Result<Vec<Task>> {
        self.read_jsonl(&self.tasks_path())
    }

    pub fn add_note(&self, body: String) -> Result<Note> {
        let id = self
            .notes()?
            .into_iter()
            .map(|note| note.id)
            .max()
            .unwrap_or_default()
            + 1;
        let note = Note::new(id, body);
        self.append_json(&self.notes_path(), &note)?;
        Ok(note)
    }

    pub fn notes(&self) -> Result<Vec<Note>> {
        self.read_jsonl(&self.notes_path())
    }

    pub fn update_note(&self, id: u64, body: String) -> Result<Option<Note>> {
        let mut notes = self.notes()?;
        let mut updated = None;
        for note in &mut notes {
            if note.id == id {
                note.body = body.clone();
                note.updated_at = chrono::Local::now();
                updated = Some(note.clone());
                break;
            }
        }
        self.write_jsonl(&self.notes_path(), &notes)?;
        Ok(updated)
    }

    pub fn complete_note(&self, id: u64) -> Result<Option<Note>> {
        let mut notes = self.notes()?;
        let mut completed = None;
        for note in &mut notes {
            if note.id == id {
                note.completed = true;
                note.updated_at = chrono::Local::now();
                completed = Some(note.clone());
                break;
            }
        }
        self.write_jsonl(&self.notes_path(), &notes)?;
        Ok(completed)
    }

    pub fn delete_note(&self, id: u64) -> Result<bool> {
        let mut notes = self.notes()?;
        let before = notes.len();
        notes.retain(|note| note.id != id);
        self.write_jsonl(&self.notes_path(), &notes)?;
        Ok(notes.len() != before)
    }

    fn append_json<T: serde::Serialize>(&self, path: &PathBuf, value: &T) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(file, "{}", serde_json::to_string(value)?)?;
        Ok(())
    }

    fn read_jsonl<T: serde::de::DeserializeOwned>(&self, path: &PathBuf) -> Result<Vec<T>> {
        if !path.exists() {
            return Ok(Vec::new());
        }
        let file = fs::File::open(path)?;
        let mut values = Vec::new();
        for line in BufReader::new(file).lines() {
            let line = line?;
            if !line.trim().is_empty() {
                values.push(serde_json::from_str(&line)?);
            }
        }
        Ok(values)
    }

    fn write_jsonl<T: serde::Serialize>(&self, path: &PathBuf, values: &[T]) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path)?;
        for value in values {
            writeln!(file, "{}", serde_json::to_string(value)?)?;
        }
        Ok(())
    }
}
