use once_cell::sync::OnceCell;

pub struct Option {
    pub silent: bool,
}
pub static INSTANCE: OnceCell<Option> = OnceCell::new();

impl Option {
    pub fn new() -> Self {
        Self { silent: false }
    }
}
