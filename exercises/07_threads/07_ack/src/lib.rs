use data::{Ticket, TicketDraft};
use std::sync::mpsc::{Receiver, Sender};
use store::{TicketId, TicketStore};

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_sender: Sender<Option<Ticket>>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_sender,
            }) => {
                let id = store.add_ticket(draft);
                response_sender.send(id).expect("Something went wrong!");
            }
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
                if let Some(ticket) = store.get(id) {
                    response_sender
                        // @mdouglasbrett - bleurgh, I feel like I should not be doing this...
                        .send(Some(ticket.clone()))
                        .expect("Something went wrong!");
                }
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
