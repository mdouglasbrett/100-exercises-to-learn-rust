// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, SyncSender, TrySendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
    capacity: usize,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, TrySendError<Command>> {
        let (resp_sender, resp_receiver) = std::sync::mpsc::sync_channel(self.capacity);
        let msg = Command::Insert {
            draft,
            response_channel: resp_sender,
        };
        let attempt = self.sender.try_send(msg);
        match attempt {
            Ok(_) => {
                let ticket_id = resp_receiver.recv().unwrap();
                Ok(ticket_id)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TrySendError<Command>> {
        let (resp_sender, resp_receiver) = std::sync::mpsc::sync_channel(self.capacity);
        let msg = Command::Get {
            id,
            response_channel: resp_sender,
        };
        let attempt = self.sender.try_send(msg);
        match attempt {
            Ok(_) => {
                let ticket = resp_receiver.recv().unwrap();
                Ok(ticket)
            }
            Err(e) => Err(e),
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender, capacity }
}

pub enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    // @mdouglasbrett - Would we bother with the try_send in the server?
    // We are going to swallow the error. There may be something I am missing here.
    // TODO: There is a Jon Gjengset video on this topic that I should watch.
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.try_send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.try_send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
