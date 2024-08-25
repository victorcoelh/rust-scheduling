use chrono::naive::NaiveDate;


struct Appointment {
    date: NaiveDate,
    description: &str,
}

impl Appointment {
    fn new(date: NaiveDate, description: &str) -> Self {
        Appointment {
            date,
            description
        }
    }
}
