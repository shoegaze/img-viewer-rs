use glium::implement_buffer_content;

#[allow(dead_code)]
pub struct ImageData {
    pub buffer: [u8],
}

implement_buffer_content!(ImageData);
