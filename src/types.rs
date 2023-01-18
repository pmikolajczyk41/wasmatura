use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The input type, i.e. the data that is passed from frontend.
pub type MaturaResults = HashMap<String, u32>;

/// Single recruitment result (for a particular `field` of study at `university`.
///
/// Other things like min|max score, department, threshold, etc. are skipped for now.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecruitmentResult {
    pub university: &'static str,
    pub year: u32,
    pub field: &'static str,
    pub result: f32,
}

/// Aggregated results.
pub type RecruitmentResults = Vec<RecruitmentResult>;
