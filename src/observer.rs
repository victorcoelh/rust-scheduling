pub trait Observer {
    fn update(self: &mut Self, event: &str);
}

struct EventDispatcher {
    subscribed: Vec<Box<dyn Observer>>
}

impl EventDispatcher {
    fn new() -> Self {
        EventDispatcher {
            subscribed: Vec::new()
        }
    }

    fn subscribe(self: &mut Self, observer: Box<dyn Observer>) {
        self.subscribed.push(observer);
    }

    fn notify(self: &mut Self, event: &str) {
        for observer in self.subscribed.iter_mut() {
            observer.update(event);
        }
    }
}
