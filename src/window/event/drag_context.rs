use winit::window::WindowId;

use crate::util::vec2::Vec2;

pub struct DragContext {
    pub target_window_id: WindowId,

    // displacement := position_cursor - position_target
    pub displacement: Vec2,
}

impl DragContext {
    pub fn new(target_window_id: WindowId, displacement: Vec2) -> Self {
        DragContext {
            target_window_id,
            displacement,
        }
    }
}
