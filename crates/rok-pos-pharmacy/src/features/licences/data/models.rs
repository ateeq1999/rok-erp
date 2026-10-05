//! The records a source hands over for the licences board, and the entities
//! they become.
//!
//! A source writes dates and reminders as the folder spells them; the parse
//! into [`CivilDate`] and [`Reminder`] is the data layer's job, so a source's
//! spelling never reaches the domain.

use crate::features::licences::domain::entities::{
    CivilDate, Document, Licence, Licences, Requirement, text,
};
use crate::features::licences::domain::enums::{Reminder, Standing};

/// One licence the pharmacy holds, as a source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LicenceRecord {
    /// What it is called.
    pub title: String,
    /// Who issued it.
    pub issuer: String,
    /// Its number.
    pub number: String,
    /// Which branch or branches it covers.
    pub branch: String,
    /// When it runs out, as the folder spells it: `30 Nov 2026`.
    pub expires_on: String,
    /// How it stands, as the source names the state.
    pub standing: String,
    /// When the renewal application has to be in, when one is open.
    pub renewal_application_due: Option<String>,
}

/// One thing the inspector will ask for, as a source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequirementRecord {
    /// What it is.
    pub name: String,
    /// How it is satisfied, or why it is not.
    pub detail: String,
    /// Whether the pharmacy has it ready.
    pub ready: bool,
}

/// One document in the folder, as a source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentRecord {
    /// What it is.
    pub name: String,
    /// Which branch it belongs to.
    pub branch: String,
    /// When it expires, when the source spells it.
    pub expires_on: Option<String>,
    /// When the reminder goes out, as the folder spells it.
    pub reminder: String,
    /// Whether the file is actually there.
    pub on_file: bool,
}

/// The whole folder, as a source hands it over.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FolderRecord {
    /// The licences and registrations.
    pub licences: Vec<LicenceRecord>,
    /// What an inspector will ask for.
    pub readiness: Vec<RequirementRecord>,
    /// The documents on file.
    pub documents: Vec<DocumentRecord>,
}

/// What a source calls a licence's standing.
fn standing(standing: &str) -> Standing {
    match standing {
        "Valid" => Standing::Valid,
        "Missing" => Standing::Missing,
        _ => Standing::Renewing,
    }
}

/// The month a folder's spelling names.
fn month(spelled: &str) -> Option<u8> {
    match spelled {
        "Jan" => Some(1),
        "Feb" => Some(2),
        "Mar" => Some(3),
        "Apr" => Some(4),
        "May" => Some(5),
        "Jun" => Some(6),
        "Jul" => Some(7),
        "Aug" => Some(8),
        "Sep" => Some(9),
        "Oct" => Some(10),
        "Nov" => Some(11),
        "Dec" => Some(12),
        _ => None,
    }
}

/// A date as the folder spells it: `30 Nov 2026`.
fn date(spelled: &str) -> Option<CivilDate> {
    let mut parts = spelled.split(' ');
    let day = parts.next()?.parse::<u8>().ok()?;
    let month = month(parts.next()?)?;
    let year = parts.next()?.parse::<i32>().ok()?;
    parts
        .next()
        .is_none()
        .then_some(CivilDate::new(year, month, day))
}

/// What a source calls a document's reminder: `90 days before`, `submit by`.
fn reminder(spelled: &str) -> Reminder {
    if spelled == "submit by" {
        Reminder::SubmitBy
    } else {
        let days = spelled
            .split(' ')
            .next()
            .and_then(|count| count.parse::<u32>().ok())
            .unwrap_or(0);
        Reminder::DaysBefore(days)
    }
}

/// Where a licence's expiry lands when a source misspells it: the epoch, which
/// reads as long lapsed rather than silently fine. The story data is parsed
/// for real in the domain tests, so a broken spelling fails there first.
const UNPARSED: CivilDate = CivilDate::new(1970, 1, 1);

impl From<LicenceRecord> for Licence {
    fn from(record: LicenceRecord) -> Self {
        Self {
            title: text(&record.title),
            issuer: text(&record.issuer),
            number: text(&record.number),
            branch: text(&record.branch),
            expires_on: date(&record.expires_on).unwrap_or(UNPARSED),
            standing: standing(&record.standing),
            renewal_application_due: record.renewal_application_due.as_deref().and_then(date),
        }
    }
}

impl From<RequirementRecord> for Requirement {
    fn from(record: RequirementRecord) -> Self {
        Self {
            name: text(&record.name),
            detail: text(&record.detail),
            ready: record.ready,
        }
    }
}

impl From<DocumentRecord> for Document {
    fn from(record: DocumentRecord) -> Self {
        Self {
            name: text(&record.name),
            branch: text(&record.branch),
            expires_on: record.expires_on.as_deref().and_then(date),
            reminder: reminder(&record.reminder),
            on_file: record.on_file,
        }
    }
}

impl From<FolderRecord> for Licences {
    fn from(record: FolderRecord) -> Self {
        Self {
            licences: record.licences.into_iter().map(Licence::from).collect(),
            readiness: record
                .readiness
                .into_iter()
                .map(Requirement::from)
                .collect(),
            documents: record.documents.into_iter().map(Document::from).collect(),
        }
    }
}
