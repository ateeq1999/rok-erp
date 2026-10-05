//! The prescription queue: every prescription received today, what state it is
//! in, and the one the pharmacist has open.
//!
//! Board: `Pharmacy_prescription_queue.html`. The board's own figures live in
//! [`QueueFigures::story`]; Phase 4 replaces them with the day's prescriptions
//! from the database.

use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::screens::board;

/// Where a prescription has got to, as the board's status column says it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Received, not yet read into the system.
    New,
    /// Read, waiting for the pharmacist's clinical check.
    NeedsCheck,
    /// The prescriber has to answer before it can go on.
    WaitingPrescriber,
    /// Checked, packed and waiting for the patient.
    ReadyToCollect,
    /// Handed over today.
    Dispensed,
}

impl Status {
    /// The word the board's status chip shows.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Status::New => "New",
            Status::NeedsCheck => "Needs pharmacist check",
            Status::WaitingPrescriber => "Waiting for prescriber",
            Status::ReadyToCollect => "Ready to collect",
            Status::Dispensed => "Dispensed today",
        }
    }

    /// How urgent the chip looks.
    #[must_use]
    pub const fn tone(self) -> Tone {
        match self {
            Status::New => Tone::Info,
            Status::NeedsCheck => Tone::Warning,
            Status::WaitingPrescriber => Tone::Danger,
            Status::ReadyToCollect => Tone::Brand,
            Status::Dispensed => Tone::Success,
        }
    }
}

/// Where a prescription came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// Handed over on paper at the counter.
    Paper,
    /// Photographed and sent on WhatsApp.
    Photo,
    /// Fetched from a prescriber by its code.
    Electronic,
}

impl Source {
    /// The word the board's source column shows.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Source::Paper => "Paper at counter",
            Source::Photo => "WhatsApp photo",
            Source::Electronic => "E-prescription",
        }
    }
}

/// One prescription in the queue.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Queued {
    /// Its reference, `RX-2217`.
    pub reference: &'static str,
    /// When it reached the pharmacy, as the board writes it: `"Received 15:11"`.
    pub received: &'static str,
    /// Who it is for.
    pub patient: &'static str,
    /// What was prescribed, as the board's second line.
    pub medicines: &'static str,
    /// Where it came from.
    pub source: Source,
    /// Where it has got to.
    pub status: Status,
    /// Anything the pharmacist must not miss, in the board's own words.
    pub flags: &'static [&'static str],
}

/// One of the five boxes in "Today's queue by status".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusCount {
    /// Which state it counts.
    pub status: Status,
    /// How many are in it.
    pub count: u32,
    /// The line under the count, as the board writes it.
    pub note: &'static str,
}

/// One prescribed medicine on the open prescription.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Item {
    /// The medicine as prescribed.
    pub name: &'static str,
    /// How many were prescribed.
    pub quantity: u32,
    /// The directions, in the pharmacist's language.
    pub directions: &'static str,
    /// Whether it can be sold right now, and from which batch.
    pub availability: &'static str,
}

/// Everything the queue board draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueueFigures {
    /// The range of references on the board, for the card's meta line.
    pub range: &'static str,
    /// How many are waiting for the pharmacist.
    pub waiting: u32,
    /// How long the oldest has waited, as the board writes it.
    pub oldest_waited: &'static str,
    /// The five status boxes, in the board's order.
    pub counts: Vec<StatusCount>,
    /// Today's prescriptions, waiting first then dispensed.
    pub queue: Vec<Queued>,
    /// The prescription the rail opens.
    pub selected: &'static str,
}

impl QueueFigures {
    /// The board's queue for Friday 2 October.
    #[must_use]
    pub fn story() -> Self {
        Self {
            range: "All prescriptions today \u{b7} RX-2210 to RX-2219",
            waiting: 6,
            oldest_waited: "oldest waiting 2 h 45 min",
            counts: vec![
                (Status::New, 2, "to read and enter"),
                (Status::NeedsCheck, 2, "1 interaction, 1 recall flag"),
                (Status::WaitingPrescriber, 1, "call-back due 15:30"),
                (Status::ReadyToCollect, 1, "patient texted"),
                (Status::Dispensed, 4, "last at 14:05"),
            ]
            .into_iter()
            .map(|(status, count, note)| StatusCount {
                status,
                count,
                note,
            })
            .collect(),
            queue: vec![
                Queued {
                    reference: "RX-2217",
                    received: "Received 15:11",
                    patient: "Daudi M.",
                    medicines: "Metformin 500mg, Atorvastatin 20mg",
                    source: Source::Electronic,
                    status: Status::New,
                    flags: &["National health insurance"],
                },
                Queued {
                    reference: "RX-2216",
                    received: "Received 15:06",
                    patient: "[Patient name]",
                    medicines: "Photo not read yet",
                    source: Source::Photo,
                    status: Status::New,
                    flags: &["Photo needs reading"],
                },
                Queued {
                    reference: "RX-2215",
                    received: "Received 15:02",
                    patient: "Rehema Juma",
                    medicines: "Child antibiotic syrup \u{b7} Amoxicillin 250mg/5ml",
                    source: Source::Paper,
                    status: Status::NeedsCheck,
                    flags: &["Batch AMS-2404 recalled \u{b7} use another batch"],
                },
                Queued {
                    reference: "RX-2214",
                    received: "Received 14:48",
                    patient: "Mzee Salim R.",
                    medicines: "Metronidazole 400mg, Amoxicillin 500mg",
                    source: Source::Paper,
                    status: Status::NeedsCheck,
                    flags: &["Interaction: warfarin"],
                },
                Queued {
                    reference: "RX-2218",
                    received: "Received 13:35",
                    patient: "Asha P.",
                    medicines: "Dose unclear on paper \u{b7} called clinic 13:50",
                    source: Source::Paper,
                    status: Status::WaitingPrescriber,
                    flags: &["Call-back due 15:30"],
                },
                Queued {
                    reference: "RX-2219",
                    received: "Received 12:30",
                    patient: "Khamis B.",
                    medicines: "Amlodipine 5mg, Losartan 50mg \u{b7} packed by John M.",
                    source: Source::Electronic,
                    status: Status::ReadyToCollect,
                    flags: &["Text sent 13:15"],
                },
                Queued {
                    reference: "RX-2210",
                    received: "Dispensed 14:05",
                    patient: "Ali Hassan",
                    medicines: "Amoxicillin, Paracetamol, ORS, Tramadol \u{b7} Grace N.",
                    source: Source::Paper,
                    status: Status::Dispensed,
                    flags: &["Controlled item signed"],
                },
                Queued {
                    reference: "RX-2213",
                    received: "Dispensed 11:55",
                    patient: "Mariam S.",
                    medicines: "Ferrous sulphate, Folic acid \u{b7} John M.",
                    source: Source::Electronic,
                    status: Status::Dispensed,
                    flags: &[],
                },
                Queued {
                    reference: "RX-2212",
                    received: "Dispensed 10:40",
                    patient: "Juma K.",
                    medicines: "Salbutamol inhaler \u{b7} Grace N.",
                    source: Source::Photo,
                    status: Status::Dispensed,
                    flags: &[],
                },
                Queued {
                    reference: "RX-2211",
                    received: "Dispensed 09:12",
                    patient: "Upendo L.",
                    medicines: "Levothyroxine 50mcg \u{b7} John M.",
                    source: Source::Paper,
                    status: Status::Dispensed,
                    flags: &[],
                },
            ],
            selected: "RX-2214",
        }
    }

    /// The prescription the rail opens, or `None` when the queue is empty.
    #[must_use]
    pub fn selected(&self) -> Option<&Queued> {
        self.queue
            .iter()
            .find(|queued| queued.reference == self.selected)
    }

    /// How many prescriptions carry at least one flag, which is what the
    /// pharmacist works through first.
    #[must_use]
    pub fn flagged(&self) -> usize {
        self.queue
            .iter()
            .filter(|queued| !queued.flags.is_empty())
            .count()
    }
}

/// The rail's open prescription: what it is, what it carries, and what to do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenPrescription {
    /// Its reference.
    pub reference: &'static str,
    /// How it reached the pharmacy and when.
    pub arrived: &'static str,
    /// Where it has got to.
    pub status: Status,
    /// Who it is for and how to reach them.
    pub patient: &'static str,
    /// The patient line under the name.
    pub patient_detail: &'static str,
    /// The button that opens the patient's record.
    pub record: &'static str,
    /// The clinical alert, if the check found one.
    pub alert: Option<(&'static str, &'static str)>,
    /// What was prescribed.
    pub items: &'static [Item],
    /// What the patient already takes.
    pub also_takes: &'static str,
    /// Known allergies.
    pub allergies: &'static str,
    /// Who pays.
    pub paid_by: &'static str,
    /// Who prescribed it.
    pub prescriber: &'static str,
    /// The rule under the buttons.
    pub rule: &'static str,
}

impl OpenPrescription {
    /// The board's open prescription, RX-2214.
    #[must_use]
    pub const fn story() -> Self {
        Self {
            reference: "RX-2214",
            arrived: "Paper at counter \u{b7} received 14:48",
            status: Status::NeedsCheck,
            patient: "Mzee Salim R.",
            patient_detail: "Male \u{b7} 68 years \u{b7} +255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 508",
            record: "Record",
            alert: Some((
                "Interaction alert: warfarin + metronidazole.",
                "Metronidazole may increase the effect of warfarin and the risk of bleeding. Contact the prescriber.",
            )),
            items: &[
                Item {
                    name: "Metronidazole 400mg tablets",
                    quantity: 21,
                    directions: "1 tablet 3 times a day for 7 days, avoid alcohol",
                    availability: "In stock",
                },
                Item {
                    name: "Amoxicillin 500mg capsules",
                    quantity: 15,
                    directions: "1 capsule 3 times a day for 5 days",
                    availability: "In stock \u{b7} AMX-2409",
                },
            ],
            also_takes: "Warfarin (as directed by clinic), Metformin 500mg, Amlodipine 5mg",
            allergies: "none known",
            paid_by: "National health insurance \u{b7} member active",
            prescriber: "[Prescriber name] \u{b7} [Dental clinic name]",
            rule: "Only a pharmacist can approve a prescription with an interaction alert.",
        }
    }
}

/// "Today's queue by status": the five boxes across the top.
fn by_status(figures: &QueueFigures, mode: ThemeMode) -> Div {
    board::card(1.4)
        .child(board::card_title("Today's queue by status"))
        .child(board::list(
            figures
                .counts
                .iter()
                .map(|count| {
                    let tone = count.status.tone();
                    board::toned_row(
                        tone,
                        mode,
                        vec![
                            board::chip(count.status.label(), tone),
                            div()
                                .sx(board::grow_style())
                                .child(count.count.to_string())
                                .child(board::meta(count.note))
                                .into_any_element(),
                        ],
                    )
                    .into_any_element()
                })
                .collect::<Vec<_>>(),
        ))
}

/// "Add prescription": how a prescription can be brought in.
fn add_prescription() -> Div {
    board::card(1.)
        .child(board::card_head("Add prescription", "3 ways in"))
        .child(board::meta("Add by:"))
        .child(board::list(
            [
                "Scan paper prescription",
                "Attach WhatsApp photo",
                "Fetch e-prescription by code",
            ]
            .into_iter()
            .map(|way| board::option(way).into_any_element())
            .collect::<Vec<_>>(),
        ))
}

/// The queue table: one row per prescription, waiting first.
fn queue_table(figures: &QueueFigures) -> Div {
    let rows = figures.queue.iter().enumerate().map(|(index, queued)| {
        let flags: Vec<AnyElement> = queued
            .flags
            .iter()
            .map(|flag| {
                board::chip(
                    (*flag).to_string(),
                    if queued.status == Status::NeedsCheck {
                        Tone::Danger
                    } else {
                        Tone::Warning
                    },
                )
            })
            .collect();
        board::link_to(("prescription-row", index), "/prescriptions")
            .sx(board::line_style())
            .child(board::cell_fixed_stack(
                format!("{}\u{a0}\u{a0}{}", queued.reference, queued.received),
                queued.patient,
            ))
            .child(board::cell_stack(queued.medicines, queued.source.label()))
            .child(board::chip(queued.status.label(), queued.status.tone()))
            .child(board::chip_line(flags))
    });
    board::card(2.)
        .child(board::card_head(
            figures.range,
            format!(
                "{} waiting \u{b7} {}",
                figures.waiting, figures.oldest_waited
            ),
        ))
        .child(board::head(vec![
            board::cell_fixed("Prescription"),
            board::cell("Patient and medicines"),
            board::cell_fixed("Status"),
            board::cell_fixed("Flags"),
        ]))
        .children(rows.collect::<Vec<_>>())
}

/// The rail: the open prescription, its alert, its items and its buttons.
fn detail(open: &OpenPrescription, mode: ThemeMode) -> Div {
    let items: Vec<AnyElement> = open
        .items
        .iter()
        .map(|item| {
            board::line(vec![
                board::cell_stack(item.name, item.directions),
                board::cell_number(format!("\u{d7} {}", item.quantity)),
                board::chip(
                    item.availability.to_string(),
                    if item.availability.starts_with("In stock") {
                        Tone::Success
                    } else {
                        Tone::Warning
                    },
                ),
            ])
            .into_any_element()
        })
        .collect();
    board::card(1.1)
        .child(board::card_head(open.reference, open.arrived.to_string()))
        .child(board::chip_line(vec![board::chip(
            open.status.label(),
            open.status.tone(),
        )]))
        .child(board::cell_stack(open.patient, open.patient_detail))
        .child(board::link("open-patient-record", "/patients", open.record).into_any_element())
        .when_some(open.alert, |rail, (title, body)| {
            rail.child(board::alert(title, body, Tone::Danger, mode))
        })
        .child(board::section_label(
            "New prescription \u{b7} dental clinic",
        ))
        .children(items)
        .child(board::panel(vec![
            board::key_value("Also takes:", open.also_takes).into_any_element(),
            board::key_value("Allergies:", open.allergies).into_any_element(),
            board::key_value("Paid by:", open.paid_by).into_any_element(),
            board::key_value("Prescriber:", open.prescriber).into_any_element(),
        ]))
        .child(board::actions(vec![
            Button::new("check-and-dispense")
                .label("Check and dispense")
                .variant(ButtonVariant::Primary)
                .into_any_element(),
            Button::new("view-image")
                .label("View prescription image")
                .into_any_element(),
            Button::new("put-on-hold")
                .label("Put on hold")
                .into_any_element(),
        ]))
        .child(board::footnote(open.rule))
}

/// The queue board.
#[component]
pub fn PrescriptionQueue(
    figures: QueueFigures,
    open: OpenPrescription,
    #[sx] sx: Sx,
    cx: &mut Cx,
) -> impl IntoElement {
    let mode = board::mode(cx);
    div()
        .sx((board::root(), &sx))
        .child(board::stat_row(vec![
            board::stat(
                "Waiting",
                figures.waiting.to_string(),
                figures.oldest_waited,
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Flagged for the pharmacist",
                figures.flagged().to_string(),
                "interaction, recall or unread photo",
                Some(Tone::Danger),
                mode,
            ),
            board::stat(
                "Dispensed today",
                figures
                    .counts
                    .iter()
                    .find(|count| count.status == Status::Dispensed)
                    .map_or(0, |count| count.count)
                    .to_string(),
                "last at 14:05",
                Some(Tone::Success),
                mode,
            ),
        ]))
        .child(
            div()
                .sx(board::row())
                .child(by_status(&figures, mode))
                .child(add_prescription()),
        )
        .child(
            div()
                .sx(board::row())
                .child(queue_table(&figures))
                .child(detail(&open, mode)),
        )
}

/// The queue with the board's figures.
#[must_use]
pub fn view() -> impl IntoElement {
    PrescriptionQueue::new(QueueFigures::story(), OpenPrescription::story())
}

#[cfg(test)]
mod tests {
    use super::{OpenPrescription, QueueFigures, Source, Status, view};
    use rok_ui::prelude::*;

    #[test]
    fn the_story_is_the_boards_queue() {
        let figures = QueueFigures::story();
        assert_eq!(figures.queue.len(), 10);
        assert_eq!(figures.waiting, 6);
        assert_eq!(figures.selected(), Some(&figures.queue[3]));
        assert_eq!(
            figures.selected().map(|queued| queued.reference),
            Some("RX-2214")
        );
    }

    #[test]
    fn the_status_boxes_add_up_to_the_board() {
        let figures = QueueFigures::story();
        let total: u32 = figures.counts.iter().map(|count| count.count).sum();
        assert_eq!(total, 10);
        assert_eq!(figures.counts[0].status.label(), "New");
        assert_eq!(
            figures.counts[1].note, "1 interaction, 1 recall flag",
            "the board says what is waiting in this box"
        );
    }

    #[test]
    fn a_prescription_with_nothing_to_say_has_no_flags() {
        let figures = QueueFigures::story();
        let flagged: Vec<&str> = figures
            .queue
            .iter()
            .filter(|queued| !queued.flags.is_empty())
            .map(|queued| queued.reference)
            .collect();
        assert_eq!(
            flagged,
            [
                "RX-2217", "RX-2216", "RX-2215", "RX-2214", "RX-2218", "RX-2219", "RX-2210"
            ]
        );
    }

    #[test]
    fn the_open_prescription_is_the_interaction_alert() {
        let open = OpenPrescription::story();
        assert_eq!(open.status, Status::NeedsCheck);
        assert!(
            open.alert.is_some(),
            "RX-2214 is the one the board opens, because of the warfarin alert"
        );
        assert_eq!(open.items.len(), 2);
        assert_eq!(open.items[0].quantity, 21);
    }

    #[test]
    fn every_status_and_source_has_the_boards_wording() {
        assert_eq!(Status::New.label(), "New");
        assert_eq!(Status::Dispensed.label(), "Dispensed today");
        assert_eq!(Source::Photo.label(), "WhatsApp photo");
        assert_eq!(Source::Electronic.label(), "E-prescription");
    }

    struct Screen;

    impl Render for Screen {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            view()
        }
    }

    #[gpui::test]
    fn draws_the_queue(cx: &mut gpui::TestAppContext) {
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
