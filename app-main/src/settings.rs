#[derive(Default, Copy, Clone, Debug)]
pub struct AppSettings {
    pub show_title: bool,
}

impl AppSettings {
    pub fn toggle_show_title(&mut self) {
        self.show_title = !self.show_title;
    }
}
