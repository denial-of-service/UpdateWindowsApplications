pub(crate) struct Progress {
    current_step: u8,
    total_steps: u8,
}

impl Progress {
    pub(crate) fn new(total_steps: u8) -> Progress {
        assert!(total_steps >= 1);
        Progress {
            current_step: 1,
            total_steps,
        }
    }

    pub(crate) fn update_progress(&mut self, message: &str) {
        assert!(self.current_step <= self.total_steps);
        println!("[{}/{}] {message}", self.current_step, self.total_steps);
        self.current_step += 1;
    }
}
