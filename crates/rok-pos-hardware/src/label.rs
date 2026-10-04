//! The dosage label: what goes on the box, and what the cashier scans.
//!
//! A label is a [`Job`] on a label roll, so the same bytes go to a 58 mm thermal
//! printer or a label printer. Nothing here needs a printer to be tested.

use crate::escpos::{Align, Job, Paper};

/// One label for one medicine, one patient.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DosageLabel {
    patient: String,
    medicine: String,
    strength: String,
    form: String,
    batch: String,
    expiry: String,
    instructions: String,
    price: String,
}

impl DosageLabel {
    /// A label for a medicine and the batch it came from: the two things it
    /// cannot be written without. The rest is chained on.
    #[must_use]
    pub fn new(medicine: impl Into<String>, batch: impl Into<String>) -> Self {
        Self {
            medicine: medicine.into(),
            batch: batch.into(),
            patient: String::new(),
            strength: String::new(),
            form: String::new(),
            expiry: String::new(),
            instructions: String::new(),
            price: String::new(),
        }
    }

    /// Who the medicine is for.
    #[must_use]
    pub fn patient(mut self, patient: impl Into<String>) -> Self {
        self.patient = patient.into();
        self
    }

    /// The strength, as printed: `500mg`.
    #[must_use]
    pub fn strength(mut self, strength: impl Into<String>) -> Self {
        self.strength = strength.into();
        self
    }

    /// The form and pack, as printed: `x100 tablets`.
    #[must_use]
    pub fn form(mut self, form: impl Into<String>) -> Self {
        self.form = form.into();
        self
    }

    /// The expiry, as printed: `08/2027`.
    #[must_use]
    pub fn expiry(mut self, expiry: impl Into<String>) -> Self {
        self.expiry = expiry.into();
        self
    }

    /// The dose, as printed.
    #[must_use]
    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = instructions.into();
        self
    }

    /// The price, as printed: `TZS 12,000`.
    #[must_use]
    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.price = price.into();
        self
    }

    /// The batch code, which is also what the barcode carries.
    #[must_use]
    pub fn batch(&self) -> &str {
        &self.batch
    }

    /// The label as bytes: medicine, dose, instructions, price, then the barcode.
    #[must_use]
    pub fn job(&self) -> Job {
        let mut job = Job::new(Paper::Mm58);
        job.bold(true).center(&self.medicine).bold(false);
        job.line(&format!("{} {}", self.strength, self.form));
        job.center(&self.patient).align(Align::Left);
        job.columns("Batch", &self.batch);
        job.columns("Expiry", &self.expiry);
        job.columns("Price", &self.price);
        job.line(&self.instructions);
        job.align(Align::Center).barcode(&self.batch);
        job.cut();
        job
    }
}

#[cfg(test)]
mod tests {
    use super::DosageLabel;

    fn label() -> DosageLabel {
        DosageLabel::new("Paracetamol", "AMS-2404")
            .patient("Grace N.")
            .strength("500mg")
            .form("x100 tablets")
            .expiry("08/2027")
            .instructions("1 tablet twice a day")
            .price("TZS 12,000")
    }

    #[test]
    fn the_label_carries_the_batch_and_the_expiry() {
        let job = label().job();
        let text = String::from_utf8(job.into_bytes()).expect("ASCII");
        for expected in [
            "Paracetamol",
            "500mg x100 tablets",
            "Grace N.",
            "AMS-2404",
            "08/2027",
            "TZS 12,000",
        ] {
            assert!(
                text.contains(expected),
                "{expected} is on the label: {text:?}"
            );
        }
    }

    #[test]
    fn the_batch_is_scannable_as_code_128() {
        let job = label().job();
        let barcode = crate::escpos::code128b(label().batch());
        let start = job
            .bytes()
            .windows(3)
            .position(|window| window == [0x1d, b'k', 73])
            .expect("the label carries a barcode");
        assert_eq!(&job.bytes()[start..start + barcode.len()], &barcode);
    }

    #[test]
    fn a_long_name_is_cut_to_the_label_width() {
        let medicine = "Amoxicillin/clavulanic acid 875mg/125mg";
        let label = DosageLabel::new(medicine, "AMX-2409")
            .patient("Grace N.")
            .strength("875mg/125mg")
            .form("x14 film-coated tablets")
            .expiry("11/2027")
            .instructions("1 sachet twice a day")
            .price("TZS 48,000");
        let job = label.job();
        let text = String::from_utf8_lossy(job.bytes()).into_owned();
        assert_eq!(job.paper().columns(), 32, "a 58 mm label is 32 columns");
        assert!(
            !text.contains(medicine),
            "a name longer than the roll is cut, not wrapped: {text:?}"
        );
        assert!(
            text.contains(&medicine[..32]),
            "the first 32 columns are printed"
        );
        assert!(
            text.contains("AMX-2409"),
            "the scan still carries the batch"
        );
    }
}
