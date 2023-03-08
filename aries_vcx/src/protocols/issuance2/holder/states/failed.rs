use messages::concepts::problem_report::ProblemReport;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Failed {
    problem_report: ProblemReport,
}

impl Failed {
    pub fn new(problem_report: ProblemReport) -> Self {
        Self { problem_report }
    }
}
