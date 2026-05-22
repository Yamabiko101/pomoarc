use anyhow::Result;
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use crate::domain::{session::SessionRecord, stats::StatsSummary, task::Task};

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
}
