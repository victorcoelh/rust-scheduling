use std::collections::HashMap;

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

    pub fn get_contacts_by_group(self: &Self) -> HashMap<String, Vec<&Contact>> {
        let mut groups = HashMap::new();

        for contact in self.contacts.iter() {
            let default_vec: Vec<&Contact> = Vec::new();
            let group = contact.group.clone().unwrap_or("Sem Grupo".to_string());
            let group = groups.entry(group).or_insert(default_vec);

            group.push(contact);
        }
        groups
    }
}

impl ToString for Agenda {
    fn to_string(&self) -> String {
        let contacts_by_group = self.get_contacts_by_group();
        let mut output = String::new();
        output.push_str("\nContatos:");

        for (group_name, members) in contacts_by_group.iter() {
            output.push_str(&format!("\nGrupo {}:\n", group_name));

            for contact in members.iter() {
                output.push_str(&contact.to_string());
            }
        }
        output
    }
}
