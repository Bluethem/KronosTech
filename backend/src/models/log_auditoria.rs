use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

/// Niveles de log para auditoría
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NivelLog {
    Info,
    Warning,
    Error,
    Success,
    Security,
}

impl std::fmt::Display for NivelLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NivelLog::Info => write!(f, "info"),
            NivelLog::Warning => write!(f, "warning"),
            NivelLog::Error => write!(f, "error"),
            NivelLog::Success => write!(f, "success"),
            NivelLog::Security => write!(f, "security"),
        }
    }
}

/// Modelo de la tabla log_auditoria
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LogAuditoria {
    pub id_log: i32,
    pub nivel: String,
    pub accion: String,
    pub detalles: Option<String>,
    pub modulo: String,
    pub id_usuario: Option<i32>,
    pub email_usuario: Option<String>,
    pub ip_cliente: Option<String>,
    pub user_agent: Option<String>,
    pub fecha_creacion: NaiveDateTime,
}

/// Request para crear un nuevo log
#[derive(Debug, Deserialize)]
pub struct CrearLogRequest {
    pub nivel: String,
    pub accion: String,
    pub detalles: Option<String>,
    pub modulo: String,
    pub email_usuario: Option<String>,
    pub ip_cliente: Option<String>,
    pub user_agent: Option<String>,
}

/// Query params para filtrar logs
#[derive(Debug, Deserialize)]
pub struct FiltrarLogsQuery {
    pub nivel: Option<String>,
    pub modulo: Option<String>,
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Response para listar logs
#[derive(Debug, Serialize)]
pub struct LogResponse {
    pub id: String,
    pub timestamp: String,
    pub level: String,
    pub action: String,
    pub user: String,
    pub ip: String,
    pub details: String,
    pub module: String,
}

impl From<LogAuditoria> for LogResponse {
    fn from(log: LogAuditoria) -> Self {
        LogResponse {
            id: log.id_log.to_string(),
            timestamp: log.fecha_creacion.format("%Y-%m-%dT%H:%M:%S").to_string(),
            level: log.nivel,
            action: log.accion,
            user: log.email_usuario.unwrap_or_else(|| "Sistema".to_string()),
            ip: log.ip_cliente.unwrap_or_else(|| "N/A".to_string()),
            details: log.detalles.unwrap_or_default(),
            module: log.modulo,
        }
    }
}

