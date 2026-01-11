use gpui::Action;

#[derive(Clone, Debug, PartialEq, Action)]
#[action(no_json)]
pub enum KeyBindingAction {
    ToggleSidebar,
    SwitchToDashboard,
    SwitchToAssets,
    SwitchToScan,
    SwitchToVulns,
    SwitchToTraffic,
    SwitchToFlows,
    SwitchToDevices,
    SwitchToSettings,
    SplitHorizontal,
    SplitVertical,
    ClosePanel,
    ZoomPanel,
}

pub struct KeyBinding {
    pub name: String,
    pub key: String,
    pub action: KeyBindingAction,
}

impl KeyBinding {
    pub fn new(name: String, key: String, action: KeyBindingAction) -> Self {
        Self { name, key, action }
    }
}

pub struct KeyboardShortcuts {
    bindings: Vec<KeyBinding>,
}

impl KeyboardShortcuts {
    pub fn new() -> Self {
        Self {
            bindings: vec![
                KeyBinding::new(
                    "Toggle Sidebar".to_string(),
                    "cmd+b".to_string(),
                    KeyBindingAction::ToggleSidebar,
                ),
                KeyBinding::new(
                    "Switch to Dashboard".to_string(),
                    "cmd+1".to_string(),
                    KeyBindingAction::SwitchToDashboard,
                ),
                KeyBinding::new(
                    "Switch to Assets".to_string(),
                    "cmd+2".to_string(),
                    KeyBindingAction::SwitchToAssets,
                ),
                KeyBinding::new(
                    "Switch to Scan".to_string(),
                    "cmd+3".to_string(),
                    KeyBindingAction::SwitchToScan,
                ),
                KeyBinding::new(
                    "Switch to Vulns".to_string(),
                    "cmd+4".to_string(),
                    KeyBindingAction::SwitchToVulns,
                ),
                KeyBinding::new(
                    "Switch to Traffic".to_string(),
                    "cmd+5".to_string(),
                    KeyBindingAction::SwitchToTraffic,
                ),
                KeyBinding::new(
                    "Switch to Flows".to_string(),
                    "cmd+6".to_string(),
                    KeyBindingAction::SwitchToFlows,
                ),
                KeyBinding::new(
                    "Switch to Devices".to_string(),
                    "cmd+7".to_string(),
                    KeyBindingAction::SwitchToDevices,
                ),
                KeyBinding::new(
                    "Switch to Settings".to_string(),
                    "cmd+,".to_string(),
                    KeyBindingAction::SwitchToSettings,
                ),
                KeyBinding::new(
                    "Split Horizontal".to_string(),
                    "cmd+k".to_string(),
                    KeyBindingAction::SplitHorizontal,
                ),
                KeyBinding::new(
                    "Split Vertical".to_string(),
                    "cmd+shift+k".to_string(),
                    KeyBindingAction::SplitVertical,
                ),
                KeyBinding::new(
                    "Close Panel".to_string(),
                    "cmd+w".to_string(),
                    KeyBindingAction::ClosePanel,
                ),
                KeyBinding::new(
                    "Zoom Panel".to_string(),
                    "cmd+=".to_string(),
                    KeyBindingAction::ZoomPanel,
                ),
            ],
        }
    }

    pub fn get_binding_for_action(&self, action: KeyBindingAction) -> Option<&KeyBinding> {
        self.bindings.iter().find(|b| b.action == action)
    }
}
