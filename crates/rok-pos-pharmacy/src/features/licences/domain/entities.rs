//! The licences board's own things: a licence, a requirement an inspector
//! asks for, a document in the folder, and the date arithmetic they need.
//!
//! The words are held as [`Text`] because they are drawn every frame; the
//! dates are real dates because the board's rule - a renewal due in days -
//! is arithmetic, not text.

use std::sync::Arc;

/// A word the board draws every frame.
pub type Text = Arc<str>;

/// Make a [`Text`] out of borrowed words.
#[must_use]
pub fn text(value: &str) -> Text {
    Text::from(value)
}

/// A calendar date, as a licence or a deadline carries it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CivilDate {
    /// The year, such as 2026.
    pub year: i32,
    /// The month, 1 to 12.
    pub month: u8,
    /// The day of the month, 1 to 31.
    pub day: u8,
}

impl CivilDate {
    /// A date from its parts. The caller hands over a real calendar date; the
    /// arithmetic below tolerates an impossible one rather than panicking.
    #[must_use]
    pub const fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// Days since 1970-01-01, after Howard Hinnant's civil calendar
    /// algorithm, so a difference between two dates needs no library.
    #[must_use]
    pub fn epoch_days(self) -> i64 {
        let shifted_year = i64::from(self.year) - i64::from(u8::from(self.month <= 2));
        let era = if shifted_year >= 0 {
            shifted_year
        } else {
            shifted_year - 399
        } / 400;
        let year_of_era = shifted_year - era * 400;
        let month = i64::from(self.month);
        let day = i64::from(self.day);
        let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        era * 146_097 + day_of_era - 719_468
    }
}

/// How many days `later` lies after `from`; negative when it lies before.
///
/// The board's "due in 57 days" is this number.
#[must_use]
pub fn days_until(from: CivilDate, later: CivilDate) -> i32 {
    let difference = later.epoch_days() - from.epoch_days();
    i32::try_from(difference).unwrap_or(if difference < 0 { i32::MIN } else { i32::MAX })
}

/// One licence or registration the pharmacy holds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Licence {
    /// What it is called.
    pub title: Text,
    /// Who issued it.
    pub issuer: Text,
    /// Its number.
    pub number: Text,
    /// Which branch or branches it covers.
    pub branch: Text,
    /// When it runs out.
    pub expires_on: CivilDate,
    /// How it stands.
    pub standing: super::enums::Standing,
    /// When the renewal application has to be in, when one is open.
    pub renewal_application_due: Option<CivilDate>,
}

impl Licence {
    /// Whether the pharmacy has to act on this licence.
    #[must_use]
    pub const fn needs_renewal(&self) -> bool {
        self.standing.needs_renewal()
    }

    /// How many days the licence has left from `today`; negative once lapsed.
    #[must_use]
    pub fn days_until_expiry(&self, today: CivilDate) -> i32 {
        days_until(today, self.expires_on)
    }
}

/// One thing the inspector will ask for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Requirement {
    /// What it is.
    pub name: Text,
    /// How it is satisfied, or why it is not.
    pub detail: Text,
    /// Whether the pharmacy has it ready.
    pub ready: bool,
}

/// One document in the folder the inspector reads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Document {
    /// What it is.
    pub name: Text,
    /// Which branch it belongs to.
    pub branch: Text,
    /// When it expires, or when it has to be in by.
    pub expires_on: Option<CivilDate>,
    /// When the reminder goes out.
    pub reminder: super::enums::Reminder,
    /// Whether the file is actually there.
    pub on_file: bool,
}

/// Everything the licences board draws, as one reading of the source.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Licences {
    /// The licences and registrations.
    pub licences: Vec<Licence>,
    /// What an inspector will ask for.
    pub readiness: Vec<Requirement>,
    /// The documents on file.
    pub documents: Vec<Document>,
}

impl Licences {
    /// The renewal the board leads with: the first licence that needs the
    /// pharmacy to act, which on the board is the premises licence.
    #[must_use]
    pub fn renewal(&self) -> Option<&Licence> {
        self.licences.iter().find(|licence| licence.needs_renewal())
    }
}
