use std::time::Duration;

pub(crate) struct Game;
impl Game {
    pub const INTERVAL: Duration = Duration::from_millis(200);
}
