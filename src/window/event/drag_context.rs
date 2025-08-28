use crate::util::coordinates::Coordinates;
use winit::window::WindowId;

pub struct DragContext {
    pub target_window_id: WindowId,

    // displacement := position_cursor - position_target
    pub displacement: Coordinates,
}

impl DragContext {
    pub fn new(target_window_id: WindowId, displacement: Coordinates) -> Self {
        DragContext {
            target_window_id,
            displacement,
        }
    }
}
