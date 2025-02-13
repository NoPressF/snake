use std::thread;
use std::time::Duration;

pub(crate) struct Game;
impl Game {
    pub(crate) fn update() {
        thread::sleep(Self::INTERVAL);
    }

    const INTERVAL: Duration = Duration::from_millis(50);
}
