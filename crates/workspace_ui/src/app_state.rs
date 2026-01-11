use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanStatus {
    Idle,
    Running,
    Completed,
    Failed,
}

pub struct AppState {
    pub current_target: Arc<Mutex<String>>,
    pub scan_status: Arc<Mutex<ScanStatus>>,
    pub active_ai_agent: Arc<Mutex<Option<String>>>,
    pub is_dev_mode: bool,
}

impl gpui::Global for AppState {}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_target: Arc::new(Mutex::new(String::new())),
            scan_status: Arc::new(Mutex::new(ScanStatus::Idle)),
            active_ai_agent: Arc::new(Mutex::new(None)),
            is_dev_mode: cfg!(debug_assertions),
        }
    }

    pub fn set_target(&self, target: String) {
        *self.current_target.lock() = target;
    }

    pub fn get_target(&self) -> String {
        self.current_target.lock().clone()
    }

    pub fn set_scan_status(&self, status: ScanStatus) {
        *self.scan_status.lock() = status;
    }

    pub fn get_scan_status(&self) -> ScanStatus {
        *self.scan_status.lock()
    }

    pub fn set_active_ai_agent(&self, agent: Option<String>) {
        *self.active_ai_agent.lock() = agent;
    }

    pub fn get_active_ai_agent(&self) -> Option<String> {
        self.active_ai_agent.lock().clone()
    }
}
