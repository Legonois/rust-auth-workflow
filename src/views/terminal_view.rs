pub struct Position {
    x: u16,
    y: u16,
}
impl Position {
    fn new(x: u16, y: u16) -> Position {
        return Position { x, y };
    }

    fn zeros() -> Position {
        return Position { x: 0, y: 0 };
    }
}