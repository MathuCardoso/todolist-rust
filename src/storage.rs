use crate::models::Task;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Caminho do arquivo de persistência das tarefas.
pub fn data_file() -> PathBuf {
    let base = std::env::var("XDG_DATA_HOME")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".local/share")
        });
    base.join("todo-app").join("tasks.json")
}

/// Carrega a lista de tarefas do arquivo JSON. Se o arquivo não existir,
/// retorna uma lista vazia.
pub fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    load_tasks_at(&data_file())
}

pub fn load_tasks_at(path: &Path) -> Result<Vec<Task>, Box<dyn Error>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path)?;
    let tasks: Vec<Task> = serde_json::from_str(&content)?;
    Ok(tasks)
}

/// Salva a lista de tarefas no arquivo JSON, criando os diretórios necessários.
pub fn save_tasks(tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    save_tasks_at(tasks, &data_file())
}

pub fn save_tasks_at(tasks: &[Task], path: &Path) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(tasks)?;
    // Escrita atômica: escreve em um arquivo temporário e renomeia.
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, &content)?;
    fs::rename(&tmp, &path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = env::temp_dir().join(format!("todo-test-{}", nanos));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = temp_dir();
        let path = dir.join("tasks.json");
        let tasks = vec![
            Task {
                id: 1,
                name: "Task A".into(),
                description: "Desc A".into(),
                created_at: "2026-11-10".into(),
                done: false,
            },
            Task {
                id: 2,
                name: "Task B".into(),
                description: "Desc B".into(),
                created_at: "2026-11-10".into(),
                done: true,
            },
        ];
        save_tasks_at(&tasks, &path).unwrap();
        assert!(path.exists());
        assert!(!path.with_extension("json.tmp").exists());
        let loaded = load_tasks_at(&path).unwrap();
        assert_eq!(tasks, loaded);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_missing_file_returns_empty() {
        let dir = temp_dir();
        let path = dir.join("tasks.json");
        assert!(!path.exists());
        let loaded = load_tasks_at(&path).unwrap();
        assert!(loaded.is_empty());
        let _ = fs::remove_dir_all(&dir);
    }
}