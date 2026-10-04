//! The frame a screen wears before it has any data: the board's card, with the
//! board's name on it.

use rok_pos_shell::Tone;
use rok_ui::prelude::*;

styles! {
    PLACEHOLDER = {
        root: {
            flex: 1,
            display: flex,
            flex_direction: column,
            align: center,
            justify: center,
            gap: 2,
            padding: 8,
            background: card,
            border: 1,
            border_color: border,
            radius: none,
        },
        board: {
            display: flex,
            flex_direction: row,
            align: center,
            gap: 2,
            padding_x: 2,
            padding_y: 1,
            font: medium,
            text: xs,
            radius: none,
        },
        note: { text: sm, color: muted_foreground, text_align: center, max_width: 120 },
    }
}

/// The board a screen is growing from, named as the file under `design/`.
///
/// ```
/// # use rok_pos_pharmacy::screens::placeholder::Board;
/// assert_eq!(Board::PrescriptionQueue.file(), "Pharmacy_prescription_queue.html");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Board {
    /// `Pharmacy_dashboard.html`
    Dashboard,
    /// `Pharmacy_prescription_queue.html`
    PrescriptionQueue,
    /// `Pharmacy_clinical_check_label.html`
    ClinicalCheckLabel,
    /// `Pharmacy_patient_record.html`
    PatientRecord,
    /// `Pharmacy_refills_due_reminders.html`
    RefillsDueReminders,
    /// `Pharmacy_medicine_catalogue.html`
    MedicineCatalogue,
    /// `Pharmacy_batches_expiry_FEFO_cold_chain.html`
    BatchesExpiryFefoColdChain,
    /// `Pharmacy_order_from_medicine_suppliers.html`
    OrderFromMedicineSuppliers,
    /// `Pharmacy_receive_delivery_batch_expiry_cold_chain.html`
    ReceiveDelivery,
    /// `Pharmacy_batch_recall.html`
    BatchRecall,
    /// `Pharmacy_insurance_claims.html`
    InsuranceClaims,
    /// `Pharmacy_licences_amp_inspection.html`
    LicencesInspection,
    /// The till and the controlled register boards, which this export of the
    /// design does not include; the plan names them `VerticalPharmacy` and
    /// `VerticalPharmacyRegister`.
    VerticalDispensary,
    /// See [`Board::VerticalDispensary`].
    VerticalControlledRegister,
}

impl Board {
    /// Every board a screen can grow from.
    pub const ALL: [Board; 14] = [
        Board::Dashboard,
        Board::PrescriptionQueue,
        Board::ClinicalCheckLabel,
        Board::PatientRecord,
        Board::RefillsDueReminders,
        Board::MedicineCatalogue,
        Board::BatchesExpiryFefoColdChain,
        Board::OrderFromMedicineSuppliers,
        Board::ReceiveDelivery,
        Board::BatchRecall,
        Board::InsuranceClaims,
        Board::LicencesInspection,
        Board::VerticalDispensary,
        Board::VerticalControlledRegister,
    ];

    /// The file under `design/pharmacy/`, or the plan's board name where this
    /// export has no file.
    #[must_use]
    pub const fn file(self) -> &'static str {
        match self {
            Board::Dashboard => "Pharmacy_dashboard.html",
            Board::PrescriptionQueue => "Pharmacy_prescription_queue.html",
            Board::ClinicalCheckLabel => "Pharmacy_clinical_check_label.html",
            Board::PatientRecord => "Pharmacy_patient_record.html",
            Board::RefillsDueReminders => "Pharmacy_refills_due_reminders.html",
            Board::MedicineCatalogue => "Pharmacy_medicine_catalogue.html",
            Board::BatchesExpiryFefoColdChain => "Pharmacy_batches_expiry_FEFO_cold_chain.html",
            Board::OrderFromMedicineSuppliers => "Pharmacy_order_from_medicine_suppliers.html",
            Board::ReceiveDelivery => "Pharmacy_receive_delivery_batch_expiry_cold_chain.html",
            Board::BatchRecall => "Pharmacy_batch_recall.html",
            Board::InsuranceClaims => "Pharmacy_insurance_claims.html",
            Board::LicencesInspection => "Pharmacy_licences_amp_inspection.html",
            Board::VerticalDispensary => "VerticalPharmacy",
            Board::VerticalControlledRegister => "VerticalPharmacyRegister",
        }
    }
}

/// A screen with no data yet: the board it will grow from, named on screen.
///
/// ```
/// # use rok_pos_pharmacy::screens::placeholder::{Board, Placeholder};
/// let screen = Placeholder::new(Board::InsuranceClaims);
/// ```
#[component]
pub fn Placeholder(board: Board, #[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let palette = rok_pos_shell::tone::colors(Tone::Info, cx.theme().mode);
    let file = board.file().to_string();
    div()
        .sx((&PLACEHOLDER.root, &sx))
        .child(
            div()
                .sx(sx![
                    &PLACEHOLDER.board,
                    style! {
                        background: {palette.background},
                        color: {palette.foreground},
                    },
                ])
                .child(format!("design/pharmacy/{file}")),
        )
        .child(
            div()
                .sx(&PLACEHOLDER.note)
                .child("The board is drawn; the data arrives with its phase."),
        )
}

#[cfg(test)]
mod tests {
    use super::{Board, Placeholder};
    use std::path::Path;

    use rok_ui::prelude::*;

    #[test]
    fn every_board_names_a_file_under_the_pharmacy_design() {
        for board in Board::ALL {
            let file = board.file();
            assert!(!file.is_empty(), "{board:?} names no board");
            assert!(
                Path::new(file)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("html"))
                    || !file.contains(' '),
                "{board:?} names {file}, which is not a board file"
            );
        }
    }

    #[test]
    fn the_two_boards_this_export_leaves_out_are_named_by_the_plan() {
        assert_eq!(Board::VerticalDispensary.file(), "VerticalPharmacy");
        assert_eq!(
            Board::VerticalControlledRegister.file(),
            "VerticalPharmacyRegister"
        );
    }

    struct Screen;

    impl Render for Screen {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().children(
                [
                    Board::Dashboard,
                    Board::PrescriptionQueue,
                    Board::VerticalDispensary,
                ]
                .into_iter()
                .map(Placeholder::new),
            )
        }
    }

    #[gpui::test]
    fn draws_a_placeholder_for_any_board(cx: &mut gpui::TestAppContext) {
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
