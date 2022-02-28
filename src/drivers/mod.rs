use super::context::{Context, Settings};

pub fn get_context(settings: Settings) -> Context {
    Context { settings }
}
