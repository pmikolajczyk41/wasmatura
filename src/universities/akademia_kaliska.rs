use crate::universities::{FieldOfStudy, University};

pub struct AkademiaKaliska;

impl University for AkademiaKaliska {
    const NAME: &'static str = "Akademia Kaliska";
    const YEAR: u32 = 2022;

    const REGISTERED_FIELDS: Vec<Box<dyn FieldOfStudy>> = vec![];
}
