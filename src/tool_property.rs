use derive_more::derive::Display;

#[derive(Debug, Display)]
pub enum ToolProperty {
    Name,
    Message,
    Id,
    Completed,
}
