mod akademia_kaliska;
mod uni_registry;

use crate::{
    types::{MaturaResults, RecruitmentResult, RecruitmentResults},
    universities::uni_registry::registered_universities,
};

/// Calculate recruitment results for every registered university for `matura_results`.
pub fn process_matura_results(matura_results: &MaturaResults) -> RecruitmentResults {
    registered_universities()
        .into_iter()
        .flat_map(|uni| uni.calculate_aggregated_results(matura_results))
        .collect()
}

/// Common API for every university.
trait University {
    const NAME: &'static str;
    const YEAR: u32;

    /// Get all available fields of study.
    fn registered_fields(&self) -> Vec<Box<dyn FieldOfStudy>>;

    /// Combine results from every registered field of study.
    fn calculate_aggregated_results(&self, matura_results: &MaturaResults) -> RecruitmentResults {
        self.registered_fields()
            .iter()
            .map(|f| {
                let result = (*f).calculate_result(matura_results);
                RecruitmentResult {
                    university: Self::NAME,
                    year: Self::YEAR,
                    field: f.name(),
                    result,
                }
            })
            .collect()
    }
}

/// Common API for every field of study.
trait FieldOfStudy {
    /// Plain field name.
    fn name(&self) -> &'static str;

    /// Compute float result for `matura_results`.
    fn calculate_result(&self, matura_results: &MaturaResults) -> f32;
}
