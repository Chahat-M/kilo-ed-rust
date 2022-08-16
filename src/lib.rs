// Defining type alias
// Instead of Result<T, E> we can write EditorResult<T>
pub type EditorResult<T, E> = std::result::Result<T, E>;

pub enum ResultCode {
    KeyReadFail
}

#[derive(Default, Copy, Clone)]  // Gives default values to the argument
pub struct CursorPos {
    pub x : u16,
    pub y : u16,
}

