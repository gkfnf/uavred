use data::models::AssetNode;

/// Events emitted by AssetDetailPanel for asset actions
#[derive(Clone, Debug)]
pub enum AssetActionEvent {
    /// User clicked Scan button - trigger scanning for this asset
    ScanRequested(AssetNode),
    /// User clicked Edit button - open edit dialog for this asset
    EditRequested(AssetNode),
    /// User clicked Delete button - delete this asset (node_id)
    DeleteRequested(String),
}
