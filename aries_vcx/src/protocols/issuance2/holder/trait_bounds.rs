use crate::errors::error::VcxResult;

pub trait GetAttributes {
    fn get_attributes(&self) -> VcxResult<String>;
}

pub trait GetAttachment {
    fn get_attachment(&self) -> VcxResult<String>;
}

pub trait IsTerminalState {
    fn is_terminal_state(&self) -> bool;
}
