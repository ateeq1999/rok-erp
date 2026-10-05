//! Licences and inspection readiness: what the pharmacy is allowed to do, when
//! each permission runs out, and whether an inspector would find the paperwork.
//!
//! Board: `Pharmacy_licences_amp_inspection.html`. The board's own licences
//! live in [`Licences::story`].

use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::screens::board;

/// How a licence stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// Held and valid.
    Valid,
    /// Held, but the renewal is due.
    Renewing,
    /// Not held yet.
    Missing,
}

impl State {
    /// What the board's chip says.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            State::Valid => "Valid",
            State::Renewing => "Renew soon",
            State::Missing => "Not held",
        }
    }

    /// How urgent the chip looks.
    #[must_use]
    pub const fn tone(self) -> Tone {
        match self {
            State::Valid => Tone::Success,
            State::Renewing => Tone::Warning,
            State::Missing => Tone::Danger,
        }
    }
}

/// One licence or registration the pharmacy holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Licence {
    /// What it is called.
    pub title: &'static str,
    /// Who issued it.
    pub issuer: &'static str,
    /// Its number.
    pub number: &'static str,
    /// Which branch it covers.
    pub branch: &'static str,
    /// When it runs out.
    pub expiry: &'static str,
    /// How the board colours the expiry date.
    pub state: State,
}

/// One thing the inspector will ask for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Requirement {
    /// What it is.
    pub label: &'static str,
    /// How it is satisfied, or why it is not.
    pub detail: &'static str,
    /// Whether the pharmacy has it ready.
    pub ready: bool,
}

/// One document in the folder the inspector reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Document {
    /// What it is.
    pub name: &'static str,
    /// Which branch it belongs to.
    pub branch: &'static str,
    /// When it expires.
    pub expires: &'static str,
    /// When the reminder goes out.
    pub reminder: &'static str,
    /// Whether the file is actually there.
    pub on_file: bool,
}

/// Everything the licences board draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Licences {
    /// The licences and registrations.
    pub licences: Vec<Licence>,
    /// What an inspector will ask for.
    pub readiness: Vec<Requirement>,
    /// The documents on file.
    pub documents: Vec<Document>,
    /// Which licence the board opens on.
    pub focus: &'static str,
}

/// The licences and registrations the pharmacy holds.
const HELD: [Licence; 5] = [
    Licence {
        title: "Pharmacy premises licence",
        issuer: "Pharmacy Council",
        number: "PC/TZ/MWE/2214",
        branch: "Mwenge",
        expiry: "30 Nov 2026",
        state: State::Renewing,
    },
    Licence {
        title: "Pharmacy premises licence",
        issuer: "Pharmacy Council",
        number: "PC/TZ/TGT/2098",
        branch: "Tegeta",
        expiry: "14 Mar 2027",
        state: State::Valid,
    },
    Licence {
        title: "Drug trader licence",
        issuer: "TFDA",
        number: "TFDA/DT/08841",
        branch: "Mwenge and Tegeta",
        expiry: "02 Feb 2027",
        state: State::Valid,
    },
    Licence {
        title: "Controlled substances registration",
        issuer: "Pharmacy Council",
        number: "PC/CS/MWE/0071",
        branch: "Mwenge",
        expiry: "31 Dec 2026",
        state: State::Renewing,
    },
    Licence {
        title: "Business licence",
        issuer: "District council",
        number: "MMC/BL/2019/4412",
        branch: "Mwenge and Tegeta",
        expiry: "30 Jun 2027",
        state: State::Valid,
    },
];

/// What an inspector asks for, hardest to satisfy first.
const READINESS: [Requirement; 6] = [
    Requirement {
        label: "Prescription registers for both branches",
        detail: "kept as issued, no corrections after the fact",
        ready: true,
    },
    Requirement {
        label: "Controlled substances register",
        detail: "running balance per substance, per patient",
        ready: true,
    },
    Requirement {
        label: "Temperature logs for the fridge",
        detail: "continuous logger, read every working day",
        ready: true,
    },
    Requirement {
        label: "Batch recall record",
        detail: "RC-0047 in progress, the list is kept either way",
        ready: true,
    },
    Requirement {
        label: "Premises licence renewal application",
        detail: "due 28 Nov, not yet submitted",
        ready: false,
    },
    Requirement {
        label: "Pharmacist's professional registration",
        detail: "Grace N. renews in January",
        ready: true,
    },
];

/// The folder the inspector reads.
const DOCUMENTS: [Document; 5] = [
    Document {
        name: "Pharmacy premises licence",
        branch: "Mwenge",
        expires: "30 Nov 2026",
        reminder: "90 days before",
        on_file: true,
    },
    Document {
        name: "Controlled substances registration",
        branch: "Mwenge",
        expires: "31 Dec 2026",
        reminder: "90 days before",
        on_file: true,
    },
    Document {
        name: "Pharmacy premises licence",
        branch: "Tegeta",
        expires: "14 Mar 2027",
        reminder: "90 days before",
        on_file: true,
    },
    Document {
        name: "Drug trader licence",
        branch: "Mwenge and Tegeta",
        expires: "02 Feb 2027",
        reminder: "60 days before",
        on_file: true,
    },
    Document {
        name: "Premises licence renewal application",
        branch: "Mwenge",
        expires: "28 Nov 2026",
        reminder: "submit by",
        on_file: false,
    },
];

impl Licences {
    /// The board's own licences.
    #[must_use]
    pub fn story() -> Self {
        Self {
            licences: HELD.to_vec(),
            readiness: READINESS.to_vec(),
            documents: DOCUMENTS.to_vec(),
            focus: "Premises licence renewal due in 57 days",
        }
    }

    /// How many days the board says the renewal has left.
    #[must_use]
    pub fn days_left(&self) -> u32 {
        57
    }

    /// What is not held, or held but about to lapse.
    #[must_use]
    pub fn needs_work(&self) -> Vec<&Licence> {
        self.licences
            .iter()
            .filter(|licence| licence.state != State::Valid)
            .collect()
    }

    /// What the inspector would not find in the folder.
    #[must_use]
    pub fn not_ready(&self) -> Vec<&Requirement> {
        self.readiness
            .iter()
            .filter(|requirement| !requirement.ready)
            .collect()
    }

    /// The renewal the board leads with.
    ///
    /// # Panics
    ///
    /// Panics when no licence is renewing, which cannot happen on a board that
    /// opens on a renewal.
    #[must_use]
    pub fn renewal(&self) -> &Licence {
        self.licences
            .iter()
            .find(|licence| licence.state == State::Renewing)
            .expect("the board has a licence to renew")
    }
}

/// The board's headline: the renewal with the days left on it.
fn renewal_alert(licences: &Licences, mode: ThemeMode) -> Div {
    let licence = licences.renewal();
    board::alert(
        licences.focus,
        board::dotted(&[
            &format!(
                "{} \u{b7} {} \u{b7} expires {}",
                licence.issuer, licence.branch, licence.expiry
            ),
            &format!("submit by 28 Nov \u{b7} {} days left", licences.days_left()),
            "A lapsed premises licence is the one expiry that closes the pharmacy.",
        ]),
        Tone::Warning,
        mode,
    )
}

/// The licences the pharmacy holds.
fn held(licences: &Licences) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Licences and registrations",
            "Both branches",
        ))
        .child(board::head(vec![
            board::cell("Licence"),
            board::cell("Branch"),
            board::cell("Number"),
            board::cell_fixed("Expires"),
            board::cell("State"),
        ]))
        .children(
            licences
                .licences
                .iter()
                .map(|licence| {
                    board::line(vec![
                        board::cell_stack(licence.title, licence.issuer),
                        board::cell(licence.branch),
                        board::cell(licence.number),
                        board::cell_fixed(licence.expiry),
                        board::chip(licence.state.label(), licence.state.tone()).into_any_element(),
                    ])
                })
                .collect::<Vec<_>>(),
        )
}

/// What an inspector will ask for, which is the list that actually matters.
fn readiness(licences: &Licences, mode: ThemeMode) -> Div {
    let missing = licences.not_ready().len();
    board::card(1.)
        .child(board::card_head(
            "Inspection readiness",
            format!("{missing} of {} not ready", licences.readiness.len()),
        ))
        .children(
            licences
                .readiness
                .iter()
                .map(|requirement| {
                    let mark = if requirement.ready { "Ready" } else { "Missing" };
                    let tone = if requirement.ready {
                        Tone::Success
                    } else {
                        Tone::Danger
                    };
                    board::line(vec![
                        board::chip(mark, tone).into_any_element(),
                        board::cell_stack(requirement.label, requirement.detail),
                    ])
                })
                .collect::<Vec<_>>(),
        )
        .child(board::actions(vec![
            Button::new("licences-renew")
                .label("Start the renewal")
                .into_any_element(),
            Button::new("licences-export")
                .label("Export the folder")
                .into_any_element(),
        ]))
        .child(board::alert(
            "The folder is what an inspector reads",
            "A licence held in the database but not on file as a document is a licence the inspector cannot see.",
            Tone::Info,
            mode,
        ))
}

/// The documents themselves, and whether the file is actually there.
fn documents(licences: &Licences) -> Div {
    let missing = licences
        .documents
        .iter()
        .filter(|document| !document.on_file)
        .count();
    board::card(1.)
        .child(board::card_head(
            "Documents on file",
            format!("{missing} not on file"),
        ))
        .child(board::head(vec![
            board::cell("Document"),
            board::cell("Branch"),
            board::cell_fixed("Expires"),
            board::cell("Reminder"),
            board::cell("File"),
        ]))
        .children(
            licences
                .documents
                .iter()
                .map(|document| {
                    board::line(vec![
                        board::cell(document.name),
                        board::cell(document.branch),
                        board::cell_fixed(document.expires),
                        board::cell(document.reminder),
                        board::chip(
                            if document.on_file {
                                "On file"
                            } else {
                                "Missing"
                            },
                            if document.on_file {
                                Tone::Success
                            } else {
                                Tone::Danger
                            },
                        )
                        .into_any_element(),
                    ])
                })
                .collect::<Vec<_>>(),
        )
}

/// The licences and inspection board.
#[component]
pub fn LicencesAndInspection(licences: Licences, #[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let mode = board::mode(cx);
    div()
        .sx((board::root(), &sx))
        .child(renewal_alert(&licences, mode))
        .child(board::stat_row(vec![
            board::stat(
                "Licences held",
                licences.licences.len().to_string(),
                "across both branches",
                None,
                mode,
            ),
            board::stat(
                "Renewing",
                licences.needs_work().len().to_string(),
                "the premises licence is the one that closes the shop",
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Inspection readiness",
                format!(
                    "{}/{}",
                    licences.readiness.len() - licences.not_ready().len(),
                    licences.readiness.len()
                ),
                "what an inspector would find",
                Some(Tone::Brand),
                mode,
            ),
            board::stat(
                "Documents missing",
                licences
                    .documents
                    .iter()
                    .filter(|document| !document.on_file)
                    .count()
                    .to_string(),
                "the folder is what the inspector reads",
                Some(Tone::Danger),
                mode,
            ),
        ]))
        .child(held(&licences))
        .child(
            div()
                .sx(board::row())
                .child(readiness(&licences, mode))
                .child(documents(&licences)),
        )
        .child(board::footnote(
            "Nothing on this screen is a substitute for the paper: it is the list of what the folder has to contain.",
        ))
}

/// The licences and inspection board with the board's figures.
#[must_use]
pub fn view() -> impl IntoElement {
    LicencesAndInspection::new(Licences::story())
}

#[cfg(test)]
mod tests {
    use super::{Licences, State, view};
    use rok_ui::prelude::*;

    #[test]
    fn the_premises_licence_is_the_one_the_board_leads_with() {
        let licences = Licences::story();
        assert_eq!(licences.days_left(), 57);
        let renewal = licences.renewal();
        assert_eq!(renewal.state, State::Renewing);
        assert!(
            renewal.title.contains("premises"),
            "a lapsed premises licence closes the pharmacy, so it leads"
        );
    }

    #[test]
    fn a_licence_held_but_not_on_file_does_not_count_as_ready() {
        let licences = Licences::story();
        let renewal = licences.renewal();
        let filed = licences
            .documents
            .iter()
            .find(|document| document.name == renewal.title && document.branch == renewal.branch)
            .expect("the board files the premises licence");
        assert!(
            filed.on_file,
            "the licence itself is on file; what is missing is the renewal application"
        );
        let missing = licences
            .documents
            .iter()
            .find(|d| !d.on_file)
            .expect("one gap");
        assert!(
            missing.name.contains("renewal application"),
            "the gap is the application, not the licence"
        );
    }

    #[test]
    fn readiness_counts_the_one_item_still_missing() {
        let licences = Licences::story();
        let missing = licences.not_ready();
        assert_eq!(missing.len(), 1);
        assert!(
            missing[0].label.contains("renewal application"),
            "the application is the only thing not ready"
        );
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
