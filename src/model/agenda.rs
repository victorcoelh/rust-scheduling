use crate::model::contact::Contact;
use crate::observer::Publisher;


pub struct Agenda {
    contacts: Vec<Contact>,
    pub publisher: Publisher,
}

impl Agenda {
    pub fn new() -> Self {
        Agenda {
            contacts: Vec::new(),
            publisher: Publisher::new(),
        }
    }

    pub fn add_contact(self: &mut Self, first_name: &str, last_name: &str, number: &str) {
        let contact = Contact::new(first_name, last_name, number);
        self.contacts.push(contact)
    }

    pub fn remove_contact(self: &mut Self, number: &str) {
        let removed_position = self.contacts
            .iter()
            .position(|val| val.number == number)
            .expect("No contact found with number {number}.");

        self.contacts.swap_remove(removed_position);
    }

    pub fn get_contact_by_name(self: &mut Self, first_name: &str) -> &mut Contact {
        self.contacts
            .iter_mut()
            .find(|val| val.first_name == first_name)
            .expect("No contact found with name {first_name}")
    }
}

impl ToString for Agenda {
    fn to_string(&self) -> String {
        let mut output = String::new();

        for contact in self.contacts.iter() {
            output.push_str(&contact.to_string());
            output.push('\n')
        }
        output
    }
}
