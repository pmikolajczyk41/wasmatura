use std::{cmp::max, collections::HashMap};

use serde::{Deserialize, Serialize};

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

/// The input type, i.e. the data that is passed from frontend.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaturaResults(HashMap<String, u32>);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Scores {
    pub base_score: u32,
    pub advanced_score: u32,
}

impl MaturaResults {
    fn get(&self, key: &'static str) -> u32 {
        self.0.get(key).map_or(0, |x| *x)
    }

    pub fn chemistry_pair(&self) -> Scores {
        Scores {
            base_score: self.get("chemistry_base"),
            advanced_score: self.get("chemistry_adv"),
        }
    }

    pub fn biology_pair(&self) -> Scores {
        Scores {
            base_score: self.get("biology_base"),
            advanced_score: self.get("biology_adv"),
        }
    }

    pub fn language_pl_pair(&self) -> Scores {
        Scores {
            base_score: self.get("language_pl_base"),
            advanced_score: self.get("language_pl_adv"),
        }
    }

    pub fn any_foreign(&self) -> u32 {
        max(
            self.get("language_english_base"),
            self.get("language_german_base"),
        )
    }

    pub fn any_foreign_advanced(&self) -> u32 {
        max(
            self.get("language_english_adv"),
            self.get("language_german_adv"),
        )
    }
}
