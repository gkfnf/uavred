//! TrafficStore - Network traffic capture state management

use gpui::{App, AppContext, Context, Entity, EventEmitter, Global};
use std::sync::{Arc, Mutex};
use crate::models::Traffic;
use crate::repository::Database;

#[derive(Debug, Clone)]
pub enum TrafficStoreEvent {
    TrafficUpdated,
    TrafficSelected(i64),
    AnomalyDetected(i64),
    CaptureStarted,
    CaptureStopped,
}

struct GlobalTrafficStore(Entity<TrafficStore>);
impl Global for GlobalTrafficStore {}

pub struct TrafficStore {
    db: Arc<Mutex<Database>>,
    /// Traffic entries
    traffic: Vec<Traffic>,
    /// Currently selected traffic ID
    selected_traffic_id: Option<i64>,
    /// Whether traffic capture is active
    is_capturing: bool,
    /// Loading state
    is_loading: bool,
    /// Last error message
    last_error: Option<String>,
    /// Traffic statistics cache
    stats_cache: Option<TrafficStats>,
}

/// Traffic statistics for UI display
#[derive(Debug, Clone)]
pub struct TrafficStats {
    pub total_requests: i64,
    pub anomalies: i64,
    pub success_rate: f64,
    pub avg_duration_ms: i32,
    pub by_protocol: Vec<(String, i64)>,
}

impl EventEmitter<TrafficStoreEvent> for TrafficStore {}

impl TrafficStore {
    pub fn new() -> anyhow::Result<Self> {
        let db = Database::open_local()?;
        Ok(Self {
            db: Arc::new(Mutex::new(db)),
            traffic: Vec::new(),
            selected_traffic_id: None,
            is_capturing: false,
            is_loading: false,
            last_error: None,
            stats_cache: None,
        })
    }

    // ============================================
    // Traffic Operations
    // ============================================

    /// Load all traffic entries from database
    pub fn load_traffic(&mut self, cx: &mut Context<Self>) -> anyhow::Result<()> {
        self.load_traffic_with_limit(None, cx)
    }

    /// Load traffic entries with optional limit
    pub fn load_traffic_with_limit(
        &mut self,
        limit: Option<i64>,
        cx: &mut Context<Self>,
    ) -> anyhow::Result<()> {
        self.is_loading = true;
        self.last_error = None;
        cx.notify();

        let db = self.db.lock().unwrap();
        let result = db.traffic().list_all(limit);
        drop(db);

        match result {
            Ok(traffic) => {
                self.traffic = traffic;
                self.is_loading = false;
                self.update_stats_cache();
                cx.emit(TrafficStoreEvent::TrafficUpdated);
                cx.notify();
                Ok(())
            }
            Err(e) => {
                self.is_loading = false;
                self.last_error = Some(format!("Failed to load traffic: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Load traffic for a specific asset
    pub fn load_traffic_by_asset(
        &mut self,
        asset_id: i64,
        limit: Option<i64>,
        cx: &mut Context<Self>,
    ) -> anyhow::Result<()> {
        self.is_loading = true;
        cx.notify();

        let db = self.db.lock().unwrap();
        match db.traffic().list_by_asset(asset_id, limit) {
            Ok(traffic) => {
                self.traffic = traffic;
                self.is_loading = false;
                cx.emit(TrafficStoreEvent::TrafficUpdated);
                cx.notify();
                Ok(())
            }
            Err(e) => {
                self.is_loading = false;
                self.last_error = Some(format!("Failed to load traffic: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Load only anomalous traffic
    pub fn load_anomalies(&mut self, limit: Option<i64>, cx: &mut Context<Self>) -> anyhow::Result<()> {
        self.is_loading = true;
        cx.notify();

        let db = self.db.lock().unwrap();
        match db.traffic().list_anomalies(limit) {
            Ok(traffic) => {
                self.traffic = traffic;
                self.is_loading = false;
                cx.emit(TrafficStoreEvent::TrafficUpdated);
                cx.notify();
                Ok(())
            }
            Err(e) => {
                self.is_loading = false;
                self.last_error = Some(format!("Failed to load anomalies: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Add a new traffic entry
    pub fn add_traffic(&mut self, traffic: Traffic, cx: &mut Context<Self>) -> anyhow::Result<i64> {
        let db = self.db.lock().unwrap();
        match db.traffic().create(&traffic) {
            Ok(id) => {
                // Check if this is an anomaly
                if traffic.is_anomaly {
                    cx.emit(TrafficStoreEvent::AnomalyDetected(id));
                }
                drop(db);
                // Insert at the beginning (newest first)
                let mut traffic = traffic;
                traffic.id = id;
                self.traffic.insert(0, traffic);
                self.update_stats_cache();
                cx.emit(TrafficStoreEvent::TrafficUpdated);
                cx.notify();
                Ok(id)
            }
            Err(e) => {
                self.last_error = Some(format!("Failed to create traffic entry: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Delete old traffic entries
    pub fn delete_old_traffic(
        &mut self,
        older_than_days: i32,
        cx: &mut Context<Self>,
    ) -> anyhow::Result<i64> {
        let db = self.db.lock().unwrap();
        match db.traffic().delete_old(older_than_days) {
            Ok(deleted) => {
                drop(db);
                if deleted > 0 {
                    self.load_traffic(cx)?;
                }
                Ok(deleted)
            }
            Err(e) => {
                self.last_error = Some(format!("Failed to delete old traffic: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    // ============================================
    // Selection Operations
    // ============================================

    /// Select a traffic entry
    pub fn select_traffic(&mut self, traffic_id: i64, cx: &mut Context<Self>) {
        self.selected_traffic_id = Some(traffic_id);
        cx.emit(TrafficStoreEvent::TrafficSelected(traffic_id));
        cx.notify();
    }

    /// Clear selection
    pub fn clear_selection(&mut self, cx: &mut Context<Self>) {
        self.selected_traffic_id = None;
        cx.notify();
    }

    // ============================================
    // Capture State
    // ============================================

    /// Start traffic capture
    pub fn start_capture(&mut self, cx: &mut Context<Self>) {
        self.is_capturing = true;
        cx.emit(TrafficStoreEvent::CaptureStarted);
        cx.notify();
    }

    /// Stop traffic capture
    pub fn stop_capture(&mut self, cx: &mut Context<Self>) {
        self.is_capturing = false;
        cx.emit(TrafficStoreEvent::CaptureStopped);
        cx.notify();
    }

    /// Toggle capture state
    pub fn toggle_capture(&mut self, cx: &mut Context<Self>) {
        if self.is_capturing {
            self.stop_capture(cx);
        } else {
            self.start_capture(cx);
        }
    }

    // ============================================
    // Statistics
    // ============================================

    fn update_stats_cache(&mut self) {
        let db = self.db.lock().unwrap();
        if let Ok(stats) = db.traffic().get_stats() {
            let success_count = stats.total - stats.anomalies;
            let success_rate = if stats.total > 0 {
                (success_count as f64 / stats.total as f64) * 100.0
            } else {
                100.0
            };

            self.stats_cache = Some(TrafficStats {
                total_requests: stats.total,
                anomalies: stats.anomalies,
                success_rate,
                avg_duration_ms: stats.avg_duration_ms,
                by_protocol: stats.by_protocol,
            });
        }
    }

    pub fn refresh_stats(&mut self, cx: &mut Context<Self>) {
        self.update_stats_cache();
        cx.notify();
    }

    // ============================================
    // Getters
    // ============================================

    pub fn traffic(&self) -> &[Traffic] {
        &self.traffic
    }

    pub fn selected_traffic(&self) -> Option<&Traffic> {
        self.selected_traffic_id
            .and_then(|id| self.traffic.iter().find(|t| t.id == id))
    }

    pub fn selected_traffic_id(&self) -> Option<i64> {
        self.selected_traffic_id
    }

    pub fn is_capturing(&self) -> bool {
        self.is_capturing
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn clear_error(&mut self, cx: &mut Context<Self>) {
        self.last_error = None;
        cx.notify();
    }

    pub fn stats(&self) -> Option<&TrafficStats> {
        self.stats_cache.as_ref()
    }

    /// Get count of anomalous traffic entries
    pub fn anomaly_count(&self) -> usize {
        self.traffic.iter().filter(|t| t.is_anomaly).count()
    }

    /// Get traffic entries grouped by protocol
    pub fn traffic_by_protocol(&self) -> Vec<(String, Vec<&Traffic>)> {
        use std::collections::HashMap;

        let mut groups: HashMap<String, Vec<&Traffic>> = HashMap::new();
        for t in &self.traffic {
            groups.entry(t.protocol.clone()).or_default().push(t);
        }

        groups.into_iter().collect()
    }

    /// Format traffic entry as cURL command
    pub fn format_as_curl(&self, traffic: &Traffic) -> String {
        let method = traffic.method.as_deref().unwrap_or("GET");
        let mut curl = format!("curl -X {} ", method);

        // Add headers
        for line in traffic.request_headers.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();
                if !key.is_empty() && !value.is_empty() {
                    curl.push_str(&format!("-H '{}: {}' ", key, value));
                }
            }
        }

        // Add body if present
        if let Some(ref body) = traffic.request_body {
            if let Ok(body_str) = String::from_utf8(body.clone()) {
                curl.push_str(&format!("-d '{}' ", body_str));
            }
        }

        // Add URL
        let scheme = if traffic.dst_port == Some(443) {
            "https"
        } else {
            "http"
        };
        let host = &traffic.dst_ip;
        let path = &traffic.path;
        curl.push_str(&format!("{}://{}{}", scheme, host, path));

        curl
    }
}

impl TrafficStore {
    pub fn global(cx: &mut App) -> Entity<Self> {
        if cx.has_global::<GlobalTrafficStore>() {
            return cx.global::<GlobalTrafficStore>().0.clone();
        }
        panic!("TrafficStore::global() called but no global TrafficStore exists. Call init_traffic_store() first.")
    }
}

pub fn init_traffic_store(cx: &mut App) {
    if !cx.has_global::<GlobalTrafficStore>() {
        let traffic_store = cx.new(|_cx| TrafficStore::new().expect("Failed to initialize TrafficStore"));
        cx.set_global(GlobalTrafficStore(traffic_store));
    }
}

pub fn init_and_load_traffic_store(cx: &mut App) {
    init_traffic_store(cx);
    let store = TrafficStore::global(cx);
    store.update(cx, |store, cx| {
        if let Err(e) = store.load_traffic(cx) {
            tracing::error!("Failed to load traffic: {}", e);
        }
    });
}
