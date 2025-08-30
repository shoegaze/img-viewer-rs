use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{CursorIcon, WindowId};

use crate::settings::AppSettings;
use crate::window::event::drag_context::DragContext;

pub trait AppUi {
    fn get_cursor_icon(&self) -> CursorIcon;

    fn ui_poll_close(
        &mut self,
        // TODO: Replace key with event.logical_key
        event: &KeyEvent,
        event_loop: &ActiveEventLoop,
        window_id: &WindowId,
    );

    fn ui_poll_toggle_decorations(&mut self, event: &KeyEvent, settings: &mut AppSettings);

    fn ui_window_focus(&self, window_id: &WindowId, state: &ElementState);

    fn ui_update_cursor_context(&mut self, position: &PhysicalPosition<f64>);

    fn ui_update_drag_context(&mut self, window_id: &WindowId, state: &ElementState);

    fn ui_update_double_click_context(&mut self, window_id: &WindowId, state: &ElementState);

    fn sync_drag(&self, drag_context: &DragContext) -> Result<(), &'static str>;

    fn do_double_click(&mut self) -> Result<(), &'static str>;
}
