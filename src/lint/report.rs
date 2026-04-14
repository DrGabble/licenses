use crate::Lint;
use documented::DocumentedVariants;
use std::fmt::{Display, Formatter};

pub struct Report {
    pub lint: Lint,
    pub level: Level,
    pub items: Vec<String>,
}

impl Display for Report {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}: {}",
            self.items.len(),
            self.lint.get_variant_docs(),
            self.items.join(", ")
        )
    }
}

#[derive(Copy, Clone)]
pub enum Level {
    Info,
    Warning,
    Error,
}

pub trait ReportIfAny {
    fn report_if_any(self, lint: Lint, level: Level) -> Option<Report>;
}

impl<I> ReportIfAny for I
where
    I: IntoIterator<Item = String>,
{
    fn report_if_any(self, lint: Lint, level: Level) -> Option<Report> {
        let mut iterator = self.into_iter();
        let mut items: Vec<_> = std::iter::once(iterator.next()?).chain(iterator).collect();
        items.sort();
        Some(Report { lint, level, items })
    }
}
