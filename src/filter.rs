use crate::lint::{Level, Report};
use crate::{Arguments, Lint};
use std::collections::HashSet;

pub struct FilterRules {
    allow: HashSet<Lint>,
}

impl FilterRules {
    pub fn new(args: &Arguments) -> Self {
        Self {
            allow: args.allow.iter().map(|f| f.lint).collect(),
        }
    }

    pub fn filter(&self, mut report: Report) -> Option<Report> {
        if self.allow.contains(&report.lint) {
            report.level = Level::Info;
        }
        Some(report)
    }
}
