//! Refills due: the chronic patients whose medicines run out this week, what
//! reminder has gone out, and what is being held ready for them.
//!
//! Board: `Pharmacy_refills_due_reminders.html`. The board's own week lives in
//! [`Refills::story`].

use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::screens::board;

/// Whether a reminder has been sent, is booked, or was never allowed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reminder {
    /// Already sent by text message.
    Sent,
    /// Booked to go out two days before the refill is due.
    Scheduled,
    /// The patient asked for private reminders, so remind them at the counter.
    NoConsent,
}

impl Reminder {
    /// The words on the reminder chip.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Reminder::Sent => "Sent",
            Reminder::Scheduled => "Scheduled",
            Reminder::NoConsent => "No consent",
        }
    }

    /// How the board colours it.
    #[must_use]
    pub const fn tone(self) -> Tone {
        match self {
            Reminder::Sent => Tone::Success,
            Reminder::Scheduled => Tone::Info,
            Reminder::NoConsent => Tone::Warning,
        }
    }
}

/// One refill due in the week.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DueRefill {
    /// Who it is for.
    pub patient: &'static str,
    /// Their phone, masked as the board masks it.
    pub phone: &'static str,
    /// The medicine.
    pub medicine: &'static str,
    /// Which batch the board would draw from, where it says so.
    pub batch: &'static str,
    /// When the last reminder went out, or when the next one will.
    pub reminder_note: &'static str,
    /// When the medicine was last filled.
    pub last_filled: &'static str,
    /// The day it is due, with its weekday.
    pub due: &'static str,
    /// How many days are left.
    pub days_left: u32,
    /// What has been done about it.
    pub reminder: Reminder,
    /// The button on the row.
    pub action: &'static str,
}

/// What the board's four figures count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Figure {
    /// What it counts.
    pub label: &'static str,
    /// The figure.
    pub value: &'static str,
    /// The line under it.
    pub note: &'static str,
    /// How the board colours it.
    pub tone: Tone,
}

/// Everything the refills board draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refills {
    /// The week the board is looking at.
    pub window: &'static str,
    /// The four figures across the top.
    pub figures: Vec<Figure>,
    /// How many rows the board has selected for preparing in advance.
    pub selected: u32,
    /// The refills, earliest due first.
    pub rows: Vec<DueRefill>,
    /// What the board says under the table.
    pub summary: &'static str,
}

impl Refills {
    /// The board's week, 3 to 9 October.
    #[must_use]
    pub fn story() -> Self {
        Self {
            window: "Chronic patients \u{b7} next 7 days (3 to 9 October)",
            figures: vec![
                Figure {
                    label: "Refills due in 7 days",
                    value: "9",
                    note: "3 to 9 October",
                    tone: Tone::Brand,
                },
                Figure {
                    label: "Reminders sent",
                    value: "3",
                    note: "by text message",
                    tone: Tone::Success,
                },
                Figure {
                    label: "Scheduled",
                    value: "4",
                    note: "sent 2 days before due",
                    tone: Tone::Info,
                },
                Figure {
                    label: "No consent",
                    value: "2",
                    note: "remind at the counter",
                    tone: Tone::Warning,
                },
            ],
            selected: 6,
            rows: vec![
                refill(
                    "Zainabu A.",
                    "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 117",
                    "Levothyroxine 50mcg tablets",
                    "",
                    "Sent 1 Oct",
                    "03 Sep",
                    "Sat 3 Oct",
                    1,
                    Reminder::Sent,
                    "Prepare",
                ),
                refill(
                    "Neema K.",
                    "+255 6\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 342",
                    "Amlodipine 5mg tablets",
                    "",
                    "Sent 1 Oct",
                    "04 Sep",
                    "Sun 4 Oct",
                    2,
                    Reminder::Sent,
                    "Prepare",
                ),
                refill(
                    "Mzee Salim R.",
                    "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 508",
                    "Metformin 500mg tablets",
                    "Use batch MTF-2509",
                    "",
                    "05 Sep",
                    "Mon 5 Oct",
                    3,
                    Reminder::Scheduled,
                    "Open record",
                ),
                refill(
                    "Baraka M.",
                    "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 860",
                    "Salbutamol 100mcg inhaler",
                    "",
                    "Call at pickup",
                    "06 Sep",
                    "Tue 6 Oct",
                    4,
                    Reminder::NoConsent,
                    "Prepare",
                ),
                refill(
                    "Fatuma H.",
                    "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 093",
                    "Levothyroxine 100mcg tablets",
                    "",
                    "Sent 2 Oct",
                    "06 Sep",
                    "Tue 6 Oct",
                    4,
                    Reminder::Sent,
                    "Prepare",
                ),
                refill(
                    "Joseph L.",
                    "+255 6\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 451",
                    "Metformin 500mg tablets",
                    "Use batch MTF-2509",
                    "",
                    "07 Sep",
                    "Wed 7 Oct",
                    5,
                    Reminder::Scheduled,
                    "Prepare",
                ),
                refill(
                    "Mwanaisha T.",
                    "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 278",
                    "Losartan 50mg tablets",
                    "",
                    "6 Oct 09:00",
                    "08 Sep",
                    "Thu 8 Oct",
                    6,
                    Reminder::Scheduled,
                    "Prepare",
                ),
                refill(
                    "Said O.",
                    "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 734",
                    "Amlodipine 10mg tablets",
                    "",
                    "6 Oct 09:00",
                    "08 Sep",
                    "Thu 8 Oct",
                    6,
                    Reminder::Scheduled,
                    "Prepare",
                ),
                refill(
                    "Emmanuel D.",
                    "+255 6\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 519",
                    "Salbutamol 100mcg inhaler",
                    "",
                    "Call at pickup",
                    "09 Sep",
                    "Fri 9 Oct",
                    7,
                    Reminder::NoConsent,
                    "Prepare",
                ),
            ],
            summary: "Metformin 500mg: prepare from the new batch MTF-2509 (expires 05/2028). Batch MTF-2405 expires 30 Oct, before a 30-day supply ends.",
        }
    }

    /// How many refills each reminder state covers.
    #[must_use]
    pub fn count(&self, reminder: Reminder) -> usize {
        self.rows
            .iter()
            .filter(|row| row.reminder == reminder)
            .count()
    }

    /// The refill due first, which is the one that cannot slip.
    #[must_use]
    pub fn first(&self) -> Option<&DueRefill> {
        self.rows.iter().min_by_key(|row| row.days_left)
    }
}

/// Build one row; the board is a list of the same shape.
#[allow(clippy::too_many_arguments)]
const fn refill(
    patient: &'static str,
    phone: &'static str,
    medicine: &'static str,
    batch: &'static str,
    reminder_note: &'static str,
    last_filled: &'static str,
    due: &'static str,
    days_left: u32,
    reminder: Reminder,
    action: &'static str,
) -> DueRefill {
    DueRefill {
        patient,
        phone,
        medicine,
        batch,
        reminder_note,
        last_filled,
        due,
        days_left,
        reminder,
        action,
    }
}

/// The reminder text, in the language the patient asked for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Preview {
    /// Who it is for.
    pub patient: &'static str,
    /// The medicine.
    pub medicine: &'static str,
    /// When it goes out.
    pub scheduled: &'static str,
    /// The Kiswahili text.
    pub kiswahili: &'static str,
    /// The English text.
    pub english: &'static str,
    /// The rule under it.
    pub rule: &'static str,
}

impl Preview {
    /// The board's preview for Mzee Salim.
    #[must_use]
    pub const fn story() -> Self {
        Self {
            patient: "Mzee Salim R.",
            medicine: "Metformin",
            scheduled: "scheduled Sat 3 Oct 09:00",
            kiswahili: "Habari Mzee Salim, dawa yako ya Metformin itaisha tarehe 5 Oktoba. Tunaweza kukuandalia. Afya Pharmacy Mwenge.",
            english: "Hello Mzee Salim, your Metformin will run out on 5 October. We can prepare it for you. Afya Pharmacy Mwenge.",
            rule: "No medicine names are sent for patients who asked for private reminders. Messages go only to patients with consent on file.",
        }
    }
}

/// How well the chronic patients are collecting on time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Adherence {
    /// The percentage, as the board writes it.
    pub percent: &'static str,
    /// How the board measures it.
    pub rule: &'static str,
}

impl Adherence {
    /// The board's figure for this month.
    #[must_use]
    pub const fn story() -> Self {
        Self {
            percent: "82%",
            rule: "Late collections are counted after 3 days past the due date.",
        }
    }
}

/// The refills table.
fn table(refills: &Refills) -> Div {
    let rows = refills.rows.iter().enumerate().map(|(index, row)| {
        board::link_to(("refill-row", index), "/refills")
            .sx(board::line_style())
            .child(board::cell_fixed_stack(row.patient, row.phone))
            .child(board::cell_fixed_stack(row.medicine, row.batch))
            .child(board::cell_fixed(row.reminder_note))
            .child(board::cell_fixed(row.last_filled))
            .child(board::cell_fixed(row.due))
            .child(board::cell_number(row.days_left.to_string()))
            .child(board::chip(row.reminder.label(), row.reminder.tone()))
            .child(board::cell_fixed(row.action))
    });
    board::card(2.4)
        .child(board::card_head(
            format!(
                "{} refills due \u{b7} sorted by due date",
                refills.rows.len()
            ),
            format!("{} selected", refills.selected),
        ))
        .child(board::head(vec![
            board::cell_fixed("\u{2713}"),
            board::cell_fixed("Patient"),
            board::cell_fixed("Medicine"),
            board::cell_fixed("Reminder note"),
            board::cell_fixed("Last filled"),
            board::cell_fixed("Due"),
            board::cell_number("Days left"),
            board::cell_fixed("Reminder"),
            board::cell_fixed("Action"),
        ]))
        .children(rows.collect::<Vec<_>>())
        .child(board::footnote(refills.summary))
}

/// The reminder preview and the adherence figure.
fn rail(preview: &Preview, adherence: &Adherence) -> Div {
    board::card(1.)
        .child(board::card_title("Reminder message preview"))
        .child(board::meta(format!(
            "{} \u{b7} {} \u{b7} {}",
            preview.patient, preview.medicine, preview.scheduled
        )))
        .child(board::panel(vec![
            board::meta(preview.kiswahili).into_any_element(),
        ]))
        .child(board::option("English version"))
        .child(board::panel(vec![
            board::meta(preview.english).into_any_element(),
        ]))
        .child(board::footnote(preview.rule))
        .child(board::actions(vec![
            Button::new("edit-template")
                .label("Edit template")
                .into_any_element(),
            Button::new("send-now")
                .label("Send now")
                .variant(ButtonVariant::Primary)
                .into_any_element(),
        ]))
        .child(board::card_head("Adherence", "this month"))
        .child(board::stat(
            "Patients who collected on time",
            adherence.percent,
            adherence.rule,
            Some(Tone::Success),
            ThemeMode::Light,
        ))
}

/// The refills board.
#[component]
pub fn RefillsDue(
    refills: Refills,
    preview: Preview,
    adherence: Adherence,
    #[sx] sx: Sx,
    cx: &mut Cx,
) -> impl IntoElement {
    let mode = board::mode(cx);
    div()
        .sx((board::root(), &sx))
        .child(board::filters_in(
            mode,
            &[
                ("All medicines", true),
                ("Diabetes", false),
                ("Blood pressure", false),
                ("Asthma inhalers", false),
                ("Thyroid", false),
            ],
        ))
        .child(board::stat_row(
            refills
                .figures
                .iter()
                .map(|figure| {
                    board::stat(
                        figure.label,
                        figure.value,
                        figure.note,
                        Some(figure.tone),
                        mode,
                    )
                })
                .collect::<Vec<_>>(),
        ))
        .child(board::actions(vec![
            Button::new("prepare-in-advance")
                .label(format!(
                    "Prepare in advance ({} selected)",
                    refills.selected
                ))
                .variant(ButtonVariant::Primary)
                .into_any_element(),
        ]))
        .child(
            div()
                .sx(board::row())
                .child(table(&refills))
                .child(rail(&preview, &adherence)),
        )
}

/// The refills with the board's figures.
#[must_use]
pub fn view() -> impl IntoElement {
    RefillsDue::new(Refills::story(), Preview::story(), Adherence::story())
}

#[cfg(test)]
mod tests {
    use super::{Adherence, Preview, Refills, Reminder, view};
    use rok_ui::prelude::*;

    #[test]
    fn the_reminder_states_add_up_to_the_refills_due() {
        let refills = Refills::story();
        assert_eq!(refills.rows.len(), 9);
        assert_eq!(refills.count(Reminder::Sent), 3);
        assert_eq!(refills.count(Reminder::Scheduled), 4);
        assert_eq!(refills.count(Reminder::NoConsent), 2);
        assert_eq!(
            refills.count(Reminder::Sent)
                + refills.count(Reminder::Scheduled)
                + refills.count(Reminder::NoConsent),
            9
        );
    }

    #[test]
    fn the_earliest_refill_is_the_one_the_board_leads_with() {
        let refills = Refills::story();
        assert_eq!(refills.first().map(|row| row.patient), Some("Zainabu A."));
        assert_eq!(refills.first().map(|row| row.days_left), Some(1));
    }

    #[test]
    fn the_two_metformin_rows_name_the_batch_to_draw_from() {
        let refills = Refills::story();
        let metformin: Vec<&str> = refills
            .rows
            .iter()
            .filter(|row| row.medicine == "Metformin 500mg tablets")
            .map(|row| row.batch)
            .collect();
        assert_eq!(metformin, ["Use batch MTF-2509", "Use batch MTF-2509"]);
        assert!(
            refills.summary.contains("MTF-2405"),
            "the board says why the older batch is not used"
        );
    }

    #[test]
    fn a_reminder_names_the_patient_and_both_languages() {
        let preview = Preview::story();
        assert_eq!(preview.patient, "Mzee Salim R.");
        assert!(preview.kiswahili.starts_with("Habari"));
        assert!(preview.english.starts_with("Hello"));
    }

    #[test]
    fn the_adherence_rule_is_the_boards() {
        assert_eq!(Adherence::story().percent, "82%");
    }

    struct Screen;

    impl Render for Screen {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            view()
        }
    }

    #[gpui::test]
    fn draws_the_refills_board(cx: &mut gpui::TestAppContext) {
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
