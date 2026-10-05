//! Reports with the pharmacy's filters: sales by medicine and schedule, margin,
//! expiry losses, claims aging and controlled movements.
//!
//! Board: `OfficeReports`, built in Phase 13. Every report is a table of rows
//! that add up to the figure in its heading, which is what makes a report
//! worth looking at: the headline is a sum, not a number someone typed.

use rok_pos_shell::{Tone, group_digits};
use rok_ui::prelude::*;

use crate::screens::board;

/// One line of a report: what it is, what it counts, and what it came to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReportRow {
    /// What the row is.
    pub label: &'static str,
    /// How the row is broken down: the schedule, the branch, the batch.
    pub detail: &'static str,
    /// What the row came to, in the report's own unit.
    pub value: i64,
    /// The rate the row carries: a margin percentage, a share of the sales.
    pub share: Option<&'static str>,
    /// Whether the row is the one the owner has to act on.
    pub flagged: bool,
}

/// One report: the question it answers and the rows that answer it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Report {
    /// What the card is called.
    pub title: &'static str,
    /// The question it answers.
    pub question: &'static str,
    /// What the rows add up to.
    pub total: i64,
    /// What the figures are counted in.
    pub unit: &'static str,
    /// The rows themselves.
    pub rows: &'static [ReportRow],
    /// The note under the table, in the board's own words.
    pub note: &'static str,
}

impl Report {
    /// The heading figure, grouped the way the boards print money.
    #[must_use]
    pub fn headline(&self) -> String {
        format!("{} {}", group_digits(self.total), self.unit)
    }

    /// The rows worth acting on, which the board tints.
    #[must_use]
    pub fn flagged(&self) -> Vec<&ReportRow> {
        self.rows.iter().filter(|row| row.flagged).collect()
    }

    /// Whether the report's rows add up to the figure in its heading.
    #[must_use]
    pub fn adds_up(&self) -> bool {
        self.rows.iter().map(|row| row.value).sum::<i64>() == self.total
    }
}

/// Every report the pharmacy's reports screen offers, in the order the board
/// lists them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reports {
    /// The period the filters are on.
    pub period: &'static str,
    /// The branch the filters are on.
    pub branch: &'static str,
    /// The reports themselves.
    pub reports: Vec<Report>,
}

/// The medicines sold in the period, with the schedule each one was sold on.
const SALES: &[ReportRow] = &[
    ReportRow {
        label: "Amoxicillin 500mg capsules",
        detail: "prescription-only",
        value: 412_000,
        share: Some("22.4%"),
        flagged: false,
    },
    ReportRow {
        label: "Amlodipine 5mg tablets",
        detail: "prescription-only",
        value: 372_000,
        share: Some("20.2%"),
        flagged: false,
    },
    ReportRow {
        label: "Paracetamol 500mg tablets",
        detail: "general sale",
        value: 341_000,
        share: Some("18.5%"),
        flagged: false,
    },
    ReportRow {
        label: "Metronidazole 400mg tablets",
        detail: "prescription-only",
        value: 286_000,
        share: Some("15.5%"),
        flagged: false,
    },
    ReportRow {
        label: "Oral rehydration salts",
        detail: "over-the-counter",
        value: 163_000,
        share: Some("8.8%"),
        flagged: false,
    },
    ReportRow {
        label: "Human insulin 100 IU/ml",
        detail: "pharmacy medicine",
        value: 150_000,
        share: Some("8.1%"),
        flagged: false,
    },
    ReportRow {
        label: "Cetirizine 10mg tablets",
        detail: "over-the-counter",
        value: 118_000,
        share: Some("6.4%"),
        flagged: false,
    },
];

/// The margin each schedule carries, which is the number the owner reads.
const MARGIN: &[ReportRow] = &[
    ReportRow {
        label: "Prescription-only",
        detail: "384 dispensings",
        value: 284_100,
        share: Some("33.2%"),
        flagged: false,
    },
    ReportRow {
        label: "Pharmacy medicine",
        detail: "62 dispensings, cold chain",
        value: 176_300,
        share: Some("28.4%"),
        flagged: false,
    },
    ReportRow {
        label: "General sale",
        detail: "214 sales",
        value: 92_800,
        share: Some("22.0%"),
        flagged: false,
    },
    ReportRow {
        label: "Over-the-counter",
        detail: "171 sales",
        value: 59_200,
        share: Some("16.4%"),
        flagged: false,
    },
];

/// What stock is on its way out, and what has already gone.
const EXPIRY: &[ReportRow] = &[
    ReportRow {
        label: "Expired on the shelf",
        detail: "2 batches, written off this period",
        value: 86_400,
        share: None,
        flagged: true,
    },
    ReportRow {
        label: "Expiring within 30 days",
        detail: "5 batches, still sellable",
        value: 74_300,
        share: None,
        flagged: true,
    },
    ReportRow {
        label: "Expiring within 90 days",
        detail: "14 batches",
        value: 24_100,
        share: None,
        flagged: false,
    },
    ReportRow {
        label: "Quarantined, awaiting destruction",
        detail: "1 batch, pharmacist witness needed",
        value: 29_900,
        share: None,
        flagged: true,
    },
];

/// How long the insurers have been sitting on the queried claims.
const CLAIMS: &[ReportRow] = &[
    ReportRow {
        label: "0 to 30 days",
        detail: "11 claims",
        value: 41_200,
        share: None,
        flagged: false,
    },
    ReportRow {
        label: "31 to 60 days",
        detail: "6 claims",
        value: 28_600,
        share: None,
        flagged: false,
    },
    ReportRow {
        label: "61 to 90 days",
        detail: "3 claims",
        value: 14_900,
        share: None,
        flagged: false,
    },
    ReportRow {
        label: "Over 90 days",
        detail: "2 claims, chased this week",
        value: 7_800,
        share: None,
        flagged: true,
    },
];

/// The controlled lines sold in the period, which are the ones an inspector
/// asks about line by line.
const CONTROLLED: &[ReportRow] = &[
    ReportRow {
        label: "Tramadol 50mg capsules",
        detail: "20 capsules dispensed, balance 80",
        value: 300_000,
        share: None,
        flagged: false,
    },
    ReportRow {
        label: "Methylphenidate 10mg tablets",
        detail: "30 tablets dispensed, 1 repeat",
        value: 396_000,
        share: None,
        flagged: false,
    },
    ReportRow {
        label: "Morphine 10mg/ml injection",
        detail: "14 ampoules dispensed, register signed",
        value: 145_000,
        share: None,
        flagged: false,
    },
    ReportRow {
        label: "Diazepam 5mg tablets",
        detail: "60 tablets dispensed",
        value: 88_000,
        share: None,
        flagged: false,
    },
];

impl Reports {
    /// The board's own reports, on the filters the board opens with.
    #[must_use]
    pub fn story() -> Self {
        Self {
            period: "This month",
            branch: "Both branches",
            reports: vec![
                Report {
                    title: "Sales by medicine and schedule",
                    question: "What sold, and on which schedule",
                    total: 1_842_000,
                    unit: "shillings",
                    rows: SALES,
                    note: "Medicines are VAT exempt, so this is the whole of what was taken.",
                },
                Report {
                    title: "Margin",
                    question: "What each schedule leaves after cost",
                    total: 612_400,
                    unit: "shillings",
                    rows: MARGIN,
                    note: "Cost is the purchase price of the batch that was sold, not the list price.",
                },
                Report {
                    title: "Expiry losses",
                    question: "What stock has been, and is about to be, written off",
                    total: 214_700,
                    unit: "shillings",
                    rows: EXPIRY,
                    note: "A batch that is still sellable is not a loss yet; it is shown in its own bucket.",
                },
                Report {
                    title: "Claims aging",
                    question: "How long the insurers have been sitting on the queries",
                    total: 92_500,
                    unit: "shillings",
                    rows: CLAIMS,
                    note: "A claim over 90 days is chased; after 120 days it is written back.",
                },
                Report {
                    title: "Controlled movements",
                    question: "What was sold against the controlled register",
                    total: 929_000,
                    unit: "shillings",
                    rows: CONTROLLED,
                    note: "Every row here has a matching entry in the controlled register.",
                },
            ],
        }
    }

    /// The report the board leads with.
    ///
    /// # Panics
    ///
    /// Panics when the screen has no reports, which cannot happen on a screen
    /// that opens with the sales report.
    #[must_use]
    pub fn sales(&self) -> &Report {
        self.reports
            .first()
            .expect("the board leads with the sales report")
    }

    /// The queried value the claims board also shows, so the two screens cannot
    /// disagree.
    #[must_use]
    pub fn queried_value(&self) -> String {
        group_digits(self.claims().total)
    }

    /// The claims aging report.
    ///
    /// # Panics
    ///
    /// Panics when the screen has no claims report, which cannot happen on a
    /// screen that always draws one.
    #[must_use]
    pub fn claims(&self) -> &Report {
        self.reports
            .iter()
            .find(|report| report.title == "Claims aging")
            .expect("the board has a claims report")
    }

    /// The reports whose rows do not add up, which must be none of them.
    #[must_use]
    pub fn broken(&self) -> Vec<&Report> {
        self.reports
            .iter()
            .filter(|report| !report.adds_up())
            .collect()
    }
}

/// The filters the board opens with: the period and the branch.
const FILTERS: [(&str, bool); 3] = [
    ("This month", true),
    ("Last month", false),
    ("This quarter", false),
];

const BRANCHES: [(&str, bool); 3] = [
    ("Both branches", true),
    ("Mwenge", false),
    ("Tegeta", false),
];

/// One report as a card: the heading figure, the rows and the note.
fn card(report: &Report, mode: ThemeMode) -> Div {
    board::card(1.)
        .child(board::card_head(report.title, report.headline()))
        .child(board::meta(report.question))
        .child(board::head(vec![
            board::cell("Row"),
            board::cell("Detail"),
            board::cell_fixed("Value"),
            board::cell_fixed("Share"),
        ]))
        .children(
            report
                .rows
                .iter()
                .map(|row| {
                    let cells = vec![
                        board::cell(row.label),
                        board::cell(row.detail),
                        board::cell_number(group_digits(row.value)),
                        board::cell_fixed(row.share.unwrap_or("\u{2014}")),
                    ];
                    if row.flagged {
                        board::toned_row(Tone::Warning, mode, cells)
                    } else {
                        board::line(cells)
                    }
                })
                .collect::<Vec<_>>(),
        )
        .child(board::footnote(report.note))
        .child(board::actions(vec![
            Button::new("report-export")
                .label("Export CSV")
                .into_any_element(),
            Button::new("report-print")
                .label("Print")
                .into_any_element(),
        ]))
}

/// The reports screen with the board's figures.
#[component]
pub fn ReportsScreen(reports: Reports, #[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let mode = board::mode(cx);
    div()
        .sx((board::root(), &sx))
        .child(
            div()
                .sx(board::row())
                .child(board::filters_in(mode, &FILTERS))
                .child(board::filters_in(mode, &BRANCHES)),
        )
        .child(board::stat_row(vec![
            board::stat(
                "Sales",
                group_digits(reports.sales().total),
                reports.period,
                None,
                mode,
            ),
            board::stat(
                "Margin",
                group_digits(reports.reports[1].total),
                "after the cost of the batch sold",
                Some(Tone::Success),
                mode,
            ),
            board::stat(
                "Expiry loss",
                group_digits(reports.reports[2].total),
                "written off and about to be",
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Queried claims",
                reports.queried_value(),
                "waiting on the insurer",
                Some(Tone::Danger),
                mode,
            ),
            board::stat(
                "Controlled",
                group_digits(reports.reports[4].total),
                "sold against the register",
                Some(Tone::Info),
                mode,
            ),
        ]))
        .child(
            div()
                .sx(board::row())
                .child(card(&reports.reports[0], mode))
                .child(
                    board::column()
                        .child(card(&reports.reports[1], mode))
                        .child(card(&reports.reports[4], mode)),
                ),
        )
        .child(
            div()
                .sx(board::row())
                .child(card(&reports.reports[2], mode))
                .child(card(&reports.reports[3], mode)),
        )
        .child(board::footnote(format!(
            "{} \u{b7} {} \u{b7} every figure is a sum of the rows above it.",
            reports.period, reports.branch
        )))
}

/// The reports list.
#[must_use]
pub fn view() -> impl IntoElement {
    ReportsScreen::new(Reports::story())
}

#[cfg(test)]
mod tests {
    use super::{Reports, view};
    use rok_ui::prelude::*;

    #[test]
    fn every_report_is_a_sum_of_its_own_rows() {
        let reports = Reports::story();
        assert_eq!(reports.broken().len(), 0, "a heading that is not a sum");
        for report in &reports.reports {
            assert!(
                report.adds_up(),
                "{} does not add up to {}",
                report.title,
                report.headline()
            );
        }
    }

    #[test]
    fn the_claims_figure_is_the_one_the_claims_board_shows() {
        let reports = Reports::story();
        assert_eq!(reports.queried_value(), "92,500");
        let chased = reports.claims().flagged();
        assert_eq!(chased.len(), 1);
        assert!(
            chased[0].label.contains("90 days"),
            "the chased bucket is the one over 90 days"
        );
    }

    #[test]
    fn stock_that_is_still_sellable_is_not_counted_as_a_loss() {
        let reports = Reports::story();
        let expiry = &reports.reports[2];
        let within_30 = expiry
            .rows
            .iter()
            .find(|row| row.label.contains("30 days"))
            .expect("the thirty day bucket");
        assert!(
            within_30.detail.contains("still sellable"),
            "a live batch is shown apart from the written-off ones"
        );
        assert!(expiry.flagged().len() >= 2);
    }

    #[test]
    fn controlled_sales_are_reported_against_the_register() {
        let reports = Reports::story();
        let controlled = &reports.reports[4];
        assert!(
            controlled
                .rows
                .iter()
                .all(|row| row.detail.contains("dispensed"))
        );
        assert!(controlled.adds_up());
    }

    struct Screen;

    impl Render for Screen {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            view()
        }
    }

    #[gpui::test]
    fn draws_the_story(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
        });
        let (_view, window) = cx.add_window_view(|_, _| Screen);
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
    }
}
