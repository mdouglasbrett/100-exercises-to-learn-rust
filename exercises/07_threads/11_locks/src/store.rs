use crate::data::{Status, Ticket, TicketDraft};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<Mutex<Ticket>>>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        self.tickets.insert(id, Arc::new(Mutex::new(ticket)));
        id
    }

    // The `get` method should return a handle to the ticket
    // which allows the caller to either read or modify the ticket.
    pub fn get(&self, id: TicketId) -> Option<Arc<Mutex<Ticket>>> {
        // @mdouglasbrett - I don't have to do the if let here, I could
        // have done self.tickets.get(&id).cloned() on the returned option
        // TODO: tho the tests pass, not sure this doesn't change behaviour
        if let Some(ticket) = self.tickets.get(&id) {
            Some(ticket.clone())
        } else {
            None
        }
    }
}
