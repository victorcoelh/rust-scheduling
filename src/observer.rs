pub trait Observer {
    fn update(self: &mut Self);
}

pub trait Subject {
    fn subscribe(observer: Box<dyn Observer>);
    fn alert_observers(event: &str);
}
