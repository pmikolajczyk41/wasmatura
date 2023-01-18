use crate::universities::{akademia_kaliska::AkademiaKaliska, University};

/// Gather all available universities.
pub(super) fn registered_universities() -> impl IntoIterator<Item = Box<impl University>> {
    vec![Box::new(AkademiaKaliska)]
}
