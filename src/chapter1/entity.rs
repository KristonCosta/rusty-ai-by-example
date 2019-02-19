pub trait Entity {
    fn new(id: i64) -> Self;
    fn update(&mut self);
}