use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Provide an `iter` method that returns an iterator over `&Ticket` items.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn iter(&self) -> std::slice::Iter<Ticket> {
        self.tickets.iter()
    }
}

impl IntoIterator for TicketStore {
    type Item = Ticket;
    type IntoIter = std::vec::IntoIter<Ticket>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.into_iter()
    }
}

// @mdouglasbrett - the .iter chapter refers to implementing IntoIterator for
// a reference to the collection. Although it wasn't a requirement to pass this
// exercise, I thought I'd have a go at hacking it and writing a (super clunky) 
// test
impl IntoIterator for &TicketStore {
    type Item = Ticket;
    type IntoIter = std::vec::IntoIter<Ticket>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket1 = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket1);

        let ticket2 = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket2);

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = store.iter().collect();
        assert_eq!(tickets, tickets2);

        let test_tickets = vec![
            Ticket {
                title: ticket_title(),
                description: ticket_description(),
                status: Status::InProgress,
            },
            Ticket {
                title: ticket_title(),
                description: ticket_description(),
                status: Status::ToDo,
            },
        ];

        for i in &store {
            assert!(test_tickets.contains(&i));
        }
    }
}
