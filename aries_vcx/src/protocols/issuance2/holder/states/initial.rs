use crate::protocols::issuance2::holder::trait_bounds::IsTerminalState;

pub struct Initial;

impl IsTerminalState for Initial {
    fn is_terminal_state(&self) -> bool {
        false
    }
}
