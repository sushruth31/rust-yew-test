//! To-do rules. Pure data in, pure data out — no yew, no DOM, no I/O, so the view
//! layer can be a thin `use_reducer` over this and every rule is testable on the host.

/// Completion state of a single item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Todo,
    Done,
}

/// One row. `id` is stable for the life of the list and is never reused, so a
/// callback captured by an earlier render can never address a later item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub id: u64,
    pub text: String,
    pub status: Status,
}

/// Every mutation the view is allowed to request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Add(String),
    Toggle(u64),
    Remove(u64),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TodoList {
    items: Vec<Item>,
    next_id: u64,
}

impl TodoList {
    pub fn items(&self) -> &[Item] {
        &self.items
    }

    pub fn remaining(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.status == Status::Todo)
            .count()
    }

    /// Applies one action, reporting whether the list actually changed.
    pub fn apply(&mut self, action: Action) -> bool {
        match action {
            Action::Add(text) => self.add(&text).is_some(),
            Action::Toggle(id) => self.toggle(id),
            Action::Remove(id) => self.remove(id),
        }
    }

    /// Appends a trimmed item. Blank or whitespace-only input is rejected rather
    /// than stored as an unclickable empty row.
    pub fn add(&mut self, text: &str) -> Option<u64> {
        let text = text.trim();
        if text.is_empty() {
            return None;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(Item {
            id,
            text: text.to_owned(),
            status: Status::Todo,
        });
        Some(id)
    }

    pub fn toggle(&mut self, id: u64) -> bool {
        let Some(item) = self.items.iter_mut().find(|item| item.id == id) else {
            return false;
        };
        item.status = match item.status {
            Status::Todo => Status::Done,
            Status::Done => Status::Todo,
        };
        true
    }

    pub fn remove(&mut self, id: u64) -> bool {
        let before = self.items.len();
        self.items.retain(|item| item.id != id);
        before != self.items.len()
    }
}
