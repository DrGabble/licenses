use crate::lint::{Level, Report};
use crate::{Arguments, Filter, Lint};
use std::collections::HashMap;

pub struct FilterRules {
    rules: HashMap<Lint, Level>,
    sub_rules: HashMap<(Lint, String), Level>,
}

impl FilterRules {
    pub fn new(args: &Arguments) -> Self {
        Self {
            rules: rules(args),
            sub_rules: sub_rules(args),
        }
    }

    pub fn filter(&self, mut report: Report) -> Option<Report> {
        if let Some(level) = self.sub_rules.get(&(report.lint, report.item.to_string())) {
            report.level = *level;
        } else if let Some(level) = self.rules.get(&report.lint) {
            report.level = *level;
        }
        Some(report)
    }
}

fn rules(args: &Arguments) -> HashMap<Lint, Level> {
    filter_levels(args)
        .filter(|(filter, _)| filter.sub_filter.is_none())
        .map(|(filter, level)| (filter.lint, level))
        .collect()
}

fn sub_rules(args: &Arguments) -> HashMap<(Lint, String), Level> {
    filter_levels(args)
        .filter_map(|(filter, level)| match &filter.sub_filter {
            Some(sub_filter) => Some(((filter.lint, sub_filter.clone()), level)),
            None => None,
        })
        .collect()
}

fn filter_levels(args: &Arguments) -> impl Iterator<Item = (&Filter, Level)> {
    args.allow
        .iter()
        .map(|filter| (filter, Level::Info))
        .chain(args.warn.iter().map(|filter| (filter, Level::Warning)))
        .chain(args.deny.iter().map(|filter| (filter, Level::Error)))
}
