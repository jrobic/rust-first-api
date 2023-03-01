#[derive(Debug)]

pub enum UserException {
    NotFound,
    Conflict,
    Unknown,
}
