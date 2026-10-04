//! The hardware rok POS talks to, and none of it is allowed to reach the screen
//! layer: this crate takes values in and hands bytes out.
//!
//! Two things live here for Phase 0, both testable without a device:
//!
//! - [`escpos`]: the byte language thermal printers speak, as a [`escpos::Job`].
//! - [`label`]: the dosage label as a job, with a scannable batch barcode.
//!
//! [`printer`] is the seam where those bytes go: a file, standard output, or
//! memory in a test. The scanner and the cash drawer arrive with the till.
//!
//! ```
//! use rok_pos_hardware::{escpos::{Job, Paper}, label::DosageLabel, printer::{MemorySink, print}};
//!
//! let label = DosageLabel::new("Paracetamol", "AMS-2404")
//!     .patient("Grace N.")
//!     .strength("500mg")
//!     .form("x100 tablets")
//!     .expiry("08/2027")
//!     .instructions("1 tablet twice a day")
//!     .price("TZS 12,000");
//!
//! let mut sink = MemorySink::new();
//! print(&label.job(), &mut sink).expect("memory never fails");
//!
//! let printed = String::from_utf8(sink.bytes().to_vec()).expect("ASCII");
//! assert!(printed.contains("AMS-2404"));
//! ```

pub mod escpos;
pub mod label;
pub mod printer;

pub use escpos::{Job, Paper};
pub use label::DosageLabel;
pub use printer::{FileSink, MemorySink, PrintError, Sink, StdoutSink, print};
