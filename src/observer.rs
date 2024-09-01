pub trait Observer {
    fn update(self: &mut Self, event: &str);
}

pub struct Publisher {
    subscribed: Vec<Box<dyn Observer>>
}

impl Publisher {
    pub fn new() -> Self {
        Publisher {
            subscribed: Vec::new()
        }
    }

    pub fn subscribe(self: &mut Self, observer: Box<dyn Observer>) {
        self.subscribed.push(observer);
    }

    pub fn notify(self: &mut Self, event: &str) {
        for observer in self.subscribed.iter_mut() {
            observer.update(event);
        }
    }
}
