use std::thread_local;
use std::cell::Cell;
use std::marker::Copy;
use std::clone::Clone;

thread_local! {
    pub static LAST_ERROR: Cell<Error> = const {Cell::new(Error::Success)};
}

#[derive(Clone, Copy)]
pub enum Error {
    Success,
    FileNotFound,
    IsDirectory,
    WriteFail,
    ReadFail,
}

impl Error {
    pub fn last() -> Self
    {
        LAST_ERROR.with(|cell| cell.get())
    }

    pub fn make_last(self)
    {
        LAST_ERROR.with(|cell| cell.set(self));
    }
}