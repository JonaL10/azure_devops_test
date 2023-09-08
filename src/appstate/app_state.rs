pub enum AppState {
    Core,
    Side,
}

impl AppState {
    pub fn as_str(&self) -> &str {
        match self {
            AppState::Core => "Core",
            AppState::Side => "Side",
        }
    }
}