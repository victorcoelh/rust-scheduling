use chrono::naive::NaiveDate;
use crate::model::appointment;


pub struct Agenda {
    appointments: Vec<Appointment>
}

impl Agenda {
    fn new() -> Self {
        Agenda {
            appointments: Vec::new()
        }
    }

    fn add_appointment(self: &mut Self, date: NaiveDate, description: &str) {
        let appointment = Appointment::new(date, description);
        self.appointments.push(appointment)
    }

    fn remove_appointment(self: &mut Self, date: NaiveDate) {
        let removed_position = self.appointments
            .iter()
            .position(|val| val.date == date)
            .expect("No appointment found at {date}.");

        self.appointments.swap_remove(removed_position);
    }
}
