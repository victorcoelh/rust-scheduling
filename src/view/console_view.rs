use crate::observer::Observer;

pub struct ConsoleView { }

impl ConsoleView {
    pub fn new() -> Self {
        ConsoleView { }
    }

    pub fn display(self: &Self, msg: &str) {
        println!("{msg}");
    }
}

impl Observer for ConsoleView {
    fn update(self: &mut Self, event: &str) {
        self.display(event);
    }
}
