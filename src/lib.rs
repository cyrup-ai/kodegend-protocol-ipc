//! Wire protocol types for kodegend IPC
//!
//! This package contains only the wire protocol type definitions shared between
//! the kodegend daemon (server) and IPC clients. It has no dependencies on either
//! the server or client implementations to avoid circular dependencies.

use std::time::Duration;

/// Status query request (sent by CLI)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum StatusQuery {
    /// Query all services
    All,
    /// Query specific service by name
    Service(String),
    /// Query aggregated usage statistics from all backend servers for a specific connection
    UsageStats(String), // connection_id parameter
    /// Query aggregated tool history from all backend servers for a specific connection
    ToolHistory(String), // connection_id parameter
}

/// Aggregated usage statistics from all backend servers
///
/// This type is serialized over Unix socket/Named Pipe to introspection tool.
/// CRITICAL: Must match EXACTLY the type in introspection/src/inspect_usage_stats.rs
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggregatedUsageStats {
    /// Unix timestamp when stats were aggregated
    pub aggregated_at: i64,

    /// Total number of backend servers queried
    pub servers_queried: usize,

    /// Number of servers that failed to respond
    pub servers_failed: usize,

    /// Per-server usage statistics
    pub servers: Vec<ServerStats>,

    /// Global aggregates across all servers
    pub global: GlobalAggregates,
}

/// Usage statistics from a single backend server
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerStats {
    /// Server category (e.g., "filesystem", "git", "terminal")
    pub category: String,

    /// HTTP port the server is running on
    pub port: u16,

    /// Whether the server responded successfully
    pub available: bool,

    /// Error message if server was unreachable
    pub error: Option<String>,

    /// Usage statistics from the server (if available)
    /// This is a direct copy of UsageStats from kodegen-tools-introspection
    pub stats: UsageStatsSnapshot,
}

/// Snapshot of UsageStats from a backend server
/// CRITICAL: Fields must match UsageStats in kodegen-tools-introspection/src/usage_tracker.rs
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsageStatsSnapshot {
    pub total_tool_calls: u64,
    pub successful_calls: u64,
    pub failed_calls: u64,
    pub tool_counts: std::collections::HashMap<String, u64>,
    pub first_used: i64,
    pub last_used: i64,
    pub total_sessions: u64,
}

/// Global aggregated statistics across all servers
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GlobalAggregates {
    pub total_tool_calls: u64,
    pub successful_calls: u64,
    pub failed_calls: u64,
    pub success_rate: f64,
    pub total_sessions: u64,
    pub categories_active: usize,
}

/// Aggregated tool history from all backend servers for a specific connection
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggregatedToolHistory {
    /// Unix timestamp when history was aggregated
    pub aggregated_at: i64,

    /// Connection ID this history belongs to
    pub connection_id: String,

    /// Total number of backend servers queried
    pub servers_queried: usize,

    /// Number of servers that failed to respond
    pub servers_failed: usize,

    /// Per-server tool history
    pub servers: Vec<ServerToolHistory>,

    /// Total number of tool calls across all servers
    pub total_calls: usize,
}

/// Tool history from a single backend server
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerToolHistory {
    /// Server category (e.g., "filesystem", "git", "terminal")
    pub category: String,

    /// HTTP port the server is running on
    pub port: u16,

    /// Whether the server responded successfully
    pub available: bool,

    /// Error message if server was unreachable
    pub error: Option<String>,

    /// Tool call records from the server (if available)
    pub calls: Vec<ToolCallRecord>,
}

/// Single tool call record (matches kodegen_mcp_schema::ToolCallRecord)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolCallRecord {
    /// ISO 8601 timestamp (UTC)
    pub timestamp: String,

    /// Name of the tool that was called
    pub tool_name: String,

    /// Arguments passed to the tool (serialized JSON string)
    pub args_json: String,

    /// Output returned by the tool (serialized JSON string)
    pub output_json: String,

    /// Execution duration in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}

/// Per-service status information
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub state: ServiceStateKind,
    pub pid: Option<u32>,
    pub uptime: Option<Duration>,
    pub restart_count: u32,
    pub max_restarts: Option<u32>,
    pub next_restart_delay: Option<Duration>,
    pub success_window_remaining: Option<Duration>,
    pub failure_reason: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub enum ServiceStateKind {
    Running,
    Stopped,
    Failed,
    Restarting,
    Starting,
}

/// Status query response (sent by manager)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StatusResponse {
    pub daemon_running: bool,
    pub daemon_pid: u32,
    pub daemon_uptime: Duration,
    pub services: Vec<ServiceStatus>,
}
