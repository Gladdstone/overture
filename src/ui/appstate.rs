use gpui_component::IndexPath;
use crate::core::AppItem;

#[derive(Default)]
pub struct AppState {
    pub selected_index: Option<IndexPath>,
    pub application_vec: Vec<AppItem>,
}

