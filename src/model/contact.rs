pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub number: String,
    pub group: Option<String>
}

impl Contact {
    pub fn new(first_name: &str, last_name: &str, number: &str) -> Self {
        Contact {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            number: number.to_string(),
            group: None
        }
    }

    pub fn assign_to_group(self: &mut Self, group: &str) {
        self.group = Some(group.to_string());
    }

    pub fn remove_from_group(self: &mut Self) {
        self.group = None;
    }
}

impl ToString for Contact {
    fn to_string(&self) -> String {
        format!("{} {} - {}\n", self.first_name, self.last_name, self.number)
    }
}
