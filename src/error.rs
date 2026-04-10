/// Lightweight error type for egui_term internals.
///
/// Replaces the upstream `anyhow` dependency with a single,
/// message-carrying struct. Intentionally minimal — no error
/// chaining or downcasting.
#[derive(Debug)]
pub struct EguiTermError(String);

impl EguiTermError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

impl std::fmt::Display for EguiTermError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for EguiTermError {}
