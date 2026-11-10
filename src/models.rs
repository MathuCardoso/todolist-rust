use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Representa uma tarefa única da lista.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    /// Identificador único e incremental da tarefa.
    pub id: u64,
    /// Nome curto da tarefa.
    pub name: String,
    /// Descrição mais detalhada da tarefa.
    pub description: String,
    /// Data de criação da tarefa no formato ISO 8601 (UTC), ex: 2026-11-10.
    pub created_at: String,
    /// Se a tarefa já foi concluída.
    pub done: bool,
}

impl Task {
    /// Cria uma nova tarefa com o próximo ID disponível e a data/hora atual.
    pub fn new(next_id: u64, name: String, description: String) -> Self {
        Task {
            id: next_id,
            name,
            description,
            created_at: today_iso(),
            done: false,
        }
    }
}

/// Converte a data/hora atual para uma string no formato ISO 8601 (UTC).
fn today_iso() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Converte segundos desde 1970-01-01 para dias, mês, dia, ano.
    // Algoritmo civil (Howard Hinnant) para converter dias -> data.
    let days = secs / 86400;
    let z = days + 719468;
    let era = z / 146097;
    let doe = (z - era * 146097) as u32; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe as i64 + era as i64 * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };

    format!("{:04}-{:02}-{:02}", y, m, d)
}