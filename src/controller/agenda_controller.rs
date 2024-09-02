use crate::model::agenda::Agenda;

pub struct AgendaController {
    agenda: Agenda,
}

impl AgendaController {
    pub fn new(agenda: Agenda) -> Self {
        AgendaController {
            agenda: agenda
        }
    }

    pub fn save_contact_from_string_vec(self: &mut Self, input: Vec<&str>) {
        let first_name = input.get(0).expect("Missing first name.");
        let last_name = input.get(1).expect("Missing last name.");
        let number = input.get(2).expect("Missing number.");

        self.agenda.add_contact(first_name, last_name, number);
        self.agenda.publisher.notify(&self.agenda.to_string());
    }

    pub fn group_contact_from_string_vec(self: &mut Self, input: Vec<&str>) {
        let first_name = input.get(0).expect("Missing first name.");
        let group = input.get(1).expect("Missing group name.");

        self.agenda.get_contact_by_name(first_name).assign_to_group(group);
        self.agenda.publisher.notify(&self.agenda.to_string());
    }
}
