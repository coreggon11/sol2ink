// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

pub use crate::{
    impls,
    traits::*,
};
use ink_prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Storage,
    String,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub status: Status,
    pub todos: Vec<Todo>,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> StructContract for T {
    fn get(&self) -> Result<Status, Error> {
        return Ok(self.data().status)
    }

    fn set(&mut self, status: Status) -> Result<(), Error> {
        self.data().status = status;
        Ok(())
    }

    fn cancel(&mut self) -> Result<(), Error> {
        self.data().status = status.canceled;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), Error> {
        let mut status: delete;
        Ok(())
    }

    fn create_events(&mut self) -> Result<(), Error> {
        self._emit_log(Self::env().caller(), "log event", 9, accepted);
        // Sol2Ink Not Implemented yet: emit AnotherLog();
        Ok(())
    }

    fn create_todo(&mut self, text: String, priority: u8, comment: String) -> Result<(), Error> {
        // 3 ways to initialize a struct
        // - calling it like a function
        self.data()
            .todos
            .push(todo(text, false, priority, comment)?);
        // key value mapping
        // Sol2Ink Not Implemented yet: todos.push(Todo({text: _text, completed: false, priority: _priority, _comment: _comment}))
        // initialize an empty struct and then update it
        let mut todo: Todo;
        todo.text = text;
        // completed initialized to false
        todo.priority = priority;
        todo.comment = comment;
        self.data().todos.push(todo);
        Ok(())
    }

    fn status(&self) -> Status {
        self.data().status
    }

    fn todos(&self) -> Vec<Todo> {
        self.data().todos
    }

}

pub trait Internal {
    fn _emit_log(&self, sender: AccountId, message: String, priority: u8, status: Status);

    fn _emit_another_log(&self);

}

impl<T: Storage<Data>> Internal for T {
    default fn _emit_log(&self, _: AccountId, _: String, _: u8, _: Status) {}

    default fn _emit_another_log(&self) {}

}
