use std::cmp::max;

use crate::{
    types::{MaturaResults, Scores},
    universities::{FieldOfStudy, University},
};

pub struct AkademiaKaliska;

impl University for AkademiaKaliska {
    const NAME: &'static str = "Akademia Kaliska";
    const YEAR: u32 = 2022;

    fn registered_fields(&self) -> Vec<Box<dyn FieldOfStudy>> {
        vec![
            Box::new(Kosmetologia),
            Box::new(Pielegniarstwo),
            Box::new(PielegniarstwoFiliaWeWrzesni),
        ]
    }
}

fn get_score(
    Scores {
        base_score,
        advanced_score,
    }: Scores,
) -> u32 {
    let base = match base_score {
        0..=29 => 0,
        30..=50 => 1,
        51..=70 => 3,
        71..=80 => 4,
        81..=90 => 5,
        _ => 6,
    };
    let adv = match advanced_score {
        0..=9 => 0,
        10..=30 => 1,
        31..=50 => 3,
        51..=70 => 4,
        81..=90 => 5,
        _ => 6,
    };

    max(base, adv)
}

fn group1_calculate(matura_results: &MaturaResults) -> f32 {
    [
        get_score(matura_results.chemistry_pair()),
        get_score(matura_results.biology_pair()),
        get_score(matura_results.language_pl_pair()),
        get_score(Scores {
            base_score: matura_results.any_foreign(),
            advanced_score: matura_results.any_foreign_advanced(),
        }),
    ]
    .into_iter()
    .sum::<u32>() as f32
}

struct Kosmetologia;
impl FieldOfStudy for Kosmetologia {
    fn name(&self) -> &'static str {
        "Kosmetologia"
    }

    fn calculate_result(&self, matura_results: &MaturaResults) -> f32 {
        group1_calculate(matura_results)
    }
}

struct Pielegniarstwo;
impl FieldOfStudy for Pielegniarstwo {
    fn name(&self) -> &'static str {
        "Pielęgniarstwo"
    }

    fn calculate_result(&self, matura_results: &MaturaResults) -> f32 {
        group1_calculate(matura_results)
    }
}

struct PielegniarstwoFiliaWeWrzesni;
impl FieldOfStudy for PielegniarstwoFiliaWeWrzesni {
    fn name(&self) -> &'static str {
        "Pielęgniarstwo: Filia we Wrześni"
    }

    fn calculate_result(&self, matura_results: &MaturaResults) -> f32 {
        group1_calculate(matura_results)
    }
}
