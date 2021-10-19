mod database;

pub struct Contact {
    pub id: i32,
    pub name: String,
    pub number: String,
    pub email: String,
}

impl Default for Contact {
    fn default() -> Self {
        return Contact {
            id: -1,
            name: String::from(""),
            number: String::from(""),
            email: String::from(""),
        };
    }
}

pub fn create_contact_in_database(contact: Contact) {
  database::save_in_database(&contact);
}

pub fn list_of_database() {
    database::list_of_database();
}

pub fn list_from_database_using_code(code: i32) -> Contact {
    return database::list_of_database_by_id(code);
}

pub fn update_contact_in_database(contact: Contact) {
  database::update_in_database(contact);
}

pub fn delete_contact_in_database(code: i32) {
  database::delete_in_database(code);
}