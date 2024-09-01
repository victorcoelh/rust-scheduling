use rust_scheduling::model::agenda::Agenda;
use rust_scheduling::view::console_view::ConsoleView;

fn main() {
    let mut agenda = Agenda::new();
    let view = ConsoleView::new();

    agenda.add_contact("davi", "zap", "85minhapica");
    agenda.add_contact("lucas", "winle", "85kk");
    
    agenda.get_contact_by_name("davi").assign_to_group("viados");
    agenda.get_contact_by_name("lucas").assign_to_group("viados");

    agenda.publisher.subscribe(Box::new(view));
    agenda.publisher.notify(&agenda.to_string());

    println!("ended testing.");
}
