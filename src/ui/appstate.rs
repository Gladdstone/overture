use gpui_component::IndexPath;

#[derive(Default)]
pub struct AppState {
    pub selected_index: Option<IndexPath>,
    pub application_vec: Vec<String>,
}

