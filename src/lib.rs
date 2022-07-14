// Defining type alias
// Instead of Result<T, E> we can write EDitorResult<T>
pub type EditorResult<T, E> = std::result::Result<T, E>;

pub enum ResultCode {
    KeyReadFail
}
