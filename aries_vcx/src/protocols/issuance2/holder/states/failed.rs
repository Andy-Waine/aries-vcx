use messages::concepts::problem_report::ProblemReport;

use crate::protocols::issuance2::holder::trait_bounds::IsTerminalState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Failed {
    problem_report: ProblemReport,
}

impl Failed {
    pub fn new(problem_report: ProblemReport) -> Self {
        Self { problem_report }
    }
}

impl IsTerminalState for Failed {
    fn is_terminal_state(&self) -> bool {
        true
    }
}
