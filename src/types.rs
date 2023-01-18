use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type MaturaResults = HashMap<String, u32>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecruitmentResult {
    pub university: String,
    pub field: String,
    pub result: f32,
}

pub type RecruitmentResults = Vec<RecruitmentResult>;
