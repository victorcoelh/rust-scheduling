use std::io::stdin;
use std::process::exit;

use rust_scheduling::model::agenda::Agenda;
use rust_scheduling::view::console_view::ConsoleView;
use rust_scheduling::controller::agenda_controller::AgendaController;

fn main() {
    let mut agenda = Agenda::new();
    let view = ConsoleView::new();
    agenda.publisher.subscribe(Box::new(view));
    let mut controller = AgendaController::new(agenda);

    loop {
        println!("Please enter your command:");
        let mut input = String::new();

        stdin().read_line(&mut input).expect("Failed to read stdin");
        input = sanitize(&input);
        handle_console_command(&input, &mut controller)
    }
}

fn handle_console_command(input: &str, controller: &mut AgendaController) {
    println!("pica{}pica", input.split(" ").last().unwrap());
    let mut words = input.split(" ");
    let command = words.next();

    match command.expect("Missing command.") {
        "add" => controller.save_contact_from_string_vec(words.collect()),
        "group" => controller.group_contact_from_string_vec(words.collect()),
        "quit" => exit(0),
        _ => panic!("Invalid command.")
    }
}

fn sanitize(input: &str) -> String {
    let mut sanitized = input.chars();
    sanitized.next_back();
    sanitized.collect()
}
