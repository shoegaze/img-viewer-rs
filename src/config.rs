#[derive(Default, Copy, Clone, Debug)]
pub struct AppConfig {
    pub show_title: bool,
}

impl AppConfig {
    pub fn toggle_show_title(&mut self) {
        self.show_title = !self.show_title;
    }
}
