use super::Contact;

use rusqlite::{params, Connection};

pub fn save_in_database(contact: &Contact) {
    let connection = Connection::open("Database.db").expect("Erro ao abrir o banco de dados");

    prepare_database(&connection);

    connection
        .execute(
            "INSERT INTO contacts (name, number, email) VALUES (?1, ?2, ?3);",
            params![contact.name, contact.number, contact.email],
        )
        .expect("Erro ao salvar contato");
}

pub fn list_of_database() {
    let connection = Connection::open("Database.db").expect("Erro ao abrir o banco de dados");

    prepare_database(&connection);

    let mut stmt = connection
        .prepare("SELECT * FROM contacts;")
        .expect("Erro ao listar contatos");
    let contact_iter = stmt
        .query_map([], |row| {
            Ok(Contact {
                id: row.get(0).expect("Erro ao tentar ler ID"),
                name: row.get(1).expect("Eror ao tentar ler nome"),
                number: row.get(2).expect("Erro ao tentar o numero"),
                email: row.get(3).expect("Erro ao tentar ler o email"),
            })
        })
        .expect("Erro ao criar a lista de contatos");

    for contact in contact_iter {
        let found = contact.unwrap();
        println!(
            "{} | {} | {} | {}",
            found.id, found.name, found.number, found.email
        );
    }
}

pub fn list_of_database_by_id(id: i32) -> Contact {
    let mut contact_found: Contact = Contact::default();

    let connection = Connection::open("Database.db").expect("Erro ao abrir o banco de dados");

    prepare_database(&connection);

    let mut stmt = connection
        .prepare("SELECT * FROM contacts WHERE id = ?1;")
        .expect("Erro ao listar contatos");
    let contact_iter = stmt
        .query_map([id], |row| {
            Ok(Contact {
                id: row.get(0).expect("Erro ao tentar ler ID"),
                name: row.get(1).expect("Eror ao tentar ler nome"),
                number: row.get(2).expect("Erro ao tentar o numero"),
                email: row.get(3).expect("Erro ao tentar ler o email"),
            })
        })
        .expect("Erro ao criar a lista de contatos");

    for contact in contact_iter {
        let found = contact.unwrap();
        println!(
            "{} | {} | {} | {}",
            found.id, found.name, found.number, found.email
        );

        contact_found = found;
    }

    return contact_found;
}

pub fn update_in_database(contact: Contact) {
    let connection = Connection::open("Database.db").expect("Erro ao abrir o banco de dados");

    prepare_database(&connection);

    connection
        .execute(
            "UPDATE contacts 
              SET name = ?1, 
                  number = ?2,
                  email = ?3
              WHERE id = ?4",
            [
                contact.name,
                contact.number,
                contact.email,
                contact.id.to_string(),
            ],
        )
        .expect("Erro ao atualizar contato");
}

pub fn delete_in_database(id: i32) {
    let connection = Connection::open("Database.db").expect("Erro ao abrir o banco de dados");

    prepare_database(&connection);

    connection
        .execute(
            "DELETE FROM contacts
            WHERE id = ?1",
            [id],
        )
        .expect("Erro ao deletar contato");
}

fn prepare_database(connection: &Connection) {
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS contacts (
                  id INTEGER PRIMARY KEY, 
                  name TEXT NOT NULL, 
                  number TEXT NOT NULL, 
                  email TEXT);",
            [],
        )
        .expect("Erro ao tentar criar tabela");
}
