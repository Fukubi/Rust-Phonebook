mod contact;

use std::io::{stdin, stdout, Write};

use crate::contact::Contact;

fn main() {
    println!("========================");
    println!("    AGENDA TELEFONICA   ");
    println!("========================");

    loop {
        let mut buffer = String::new();

        println!("-");
        println!("--- MENU PRINCIPAL");

        println!("0. Sair");
        println!("1. Adicionar");
        println!("2. Listar");
        println!("3. Pesquisar");
        println!("4. Atualizar");
        println!("5. Deletar");

        print!("\nDigite o que deseja executar> ");
        stdout().flush().expect("Erro ao tentar limpar o buffer");

        stdin()
            .read_line(&mut buffer)
            .expect("Erro ao ler a escolha");

        match buffer.trim().parse::<u8>() {
            Ok(choice) => match choice {
                0 => break,
                1 => add_new_contact(),
                2 => list_all_contacts(),
                3 => list_by_code(),
                4 => update_contact(),
                5 => delete_contact(),
                _ => {}
            },
            Err(err) => panic!("Erro ao converter para intero: {}", err),
        };
    }
}

fn add_new_contact() {
    let mut contact_to_add = Contact::default();

    print!("Digite o nome do contato: ");
    stdout().flush().expect("Erro ao tentar limpar o buffer");
    stdin()
        .read_line(&mut contact_to_add.name)
        .expect("Erro lendo o nome do contato");

    print!("Digite o numero do contato: ");
    stdout().flush().expect("Erro ao tentar limpar o buffer");
    stdin()
        .read_line(&mut contact_to_add.number)
        .expect("Erro lendo o numero do contato");

    print!("Digite o email do contato: ");
    stdout().flush().expect("Erro ao tentar limpar o buffer");
    stdin()
        .read_line(&mut contact_to_add.email)
        .expect("Erro ao ler o email do contato");

    contact_to_add.name = String::from(contact_to_add.name.trim());
    contact_to_add.number = String::from(contact_to_add.number.trim());
    contact_to_add.email = String::from(contact_to_add.email.trim());

    contact::create_contact_in_database(contact_to_add);
}

fn list_all_contacts() {
    contact::list_of_database();
}

fn list_by_code() {
    let mut buffer = String::from("");
    let code;

    print!("Digite o codigo para pesquisar: ");
    stdout().flush().expect("Erro ao tentar limpar o buffer");
    stdin()
        .read_line(&mut buffer)
        .expect("Erro ao ler o codigo do contato");

    match buffer.trim().parse::<i32>() {
        Ok(value) => code = value,
        Err(err) => panic!("Erro ao converter para intero: {}", err),
    }

    contact::list_from_database_using_code(code);
}

fn update_contact() {
    let mut buffer = String::from("");
    let mut contact_to_update = Contact::default();

    print!("Digite o codigo do contato: ");
    stdout().flush().expect("Erro ao tentar limpar o buffer");
    stdin()
        .read_line(&mut buffer)
        .expect("Erro ao ler o codigo do contato");

    match buffer.trim().parse::<i32>() {
        Ok(value) => contact_to_update.id = value,
        Err(err) => panic!("Erro ao converter para inteiro: {}", err),
    }

    buffer.clear();

    contact_to_update = contact::list_from_database_using_code(contact_to_update.id);

    println!("Nome atual: {}", contact_to_update.name);
    print!("Novo nome (Deixe vazio para não alterar): ");
    stdout().flush().expect("Erro ao tentar limpar o buffer");
    stdin()
        .read_line(&mut buffer)
        .expect("Erro ao ler nome do contato");

    buffer = String::from(buffer.trim());

    if !buffer.is_empty() {
        contact_to_update.name = buffer.clone();
    }

    buffer.clear();

    println!("Numero atual: {}", contact_to_update.number);
    print!("Novo numero (Deixe vazio para não alterar): ");
    stdout().flush().expect("Erro ao tentar limpar o buffer");
    stdin()
        .read_line(&mut buffer)
        .expect("Erro ao ler o numero do contato");

    buffer = String::from(buffer.trim());

    if !buffer.is_empty() {
        contact_to_update.number = buffer.clone();
    }

    buffer.clear();

    println!("Email atual: {}", contact_to_update.number);
    print!("Novo email (Deixe vazio para não alterar): ");
    stdout().flush().expect("Erro ao tentar limpar o buffer");
    stdin()
        .read_line(&mut buffer)
        .expect("Erro ao ler o email do contato");

    buffer = String::from(buffer.trim());

    if !buffer.is_empty() {
        contact_to_update.email = buffer.clone();
    }

    buffer.clear();

    contact::update_contact_in_database(contact_to_update);
}

fn delete_contact() {
    let mut buffer = String::from("");
    let mut contact_to_delete = Contact::default();

    print!("Digite o codigo do contato: ");
    stdout().flush().expect("Erro ao tentar limpar o buffer");
    stdin()
        .read_line(&mut buffer)
        .expect("Erro ao ler o codigo do contato");

    match buffer.trim().parse::<i32>() {
        Ok(value) => contact_to_delete.id = value,
        Err(err) => panic!("Erro ao converter para inteiro: {}", err),
    }

    buffer.clear();

    contact_to_delete = contact::list_from_database_using_code(contact_to_delete.id);

    print!("Deseja realmente deletar esse contato [Sim(y)/Nao(n)]: ");
    stdout().flush().expect("Erro ao tentar limpar o buffer");

    loop {
        stdin()
            .read_line(&mut buffer)
            .expect("Erro ao ler o codigo do contato");

        if buffer.trim().eq("y") || buffer.trim().eq("n") {
            break;
        }

        buffer.clear();
    }

    if buffer.trim().eq("y") {
        contact::delete_contact_in_database(contact_to_delete.id);
    }
}
