//! Where the bytes go.
//!
//! [`Sink`] is the seam: a Windows spooler, a USB thermal printer, a file for a
//! demo, or memory in a test. Nothing above this line knows which it is.

use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::escpos::Job;

/// Why a job did not reach its printer.
#[derive(Debug)]
pub enum PrintError {
    /// The sink could not take the bytes.
    Io(io::Error),
}

impl std::fmt::Display for PrintError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "printing failed: {error}"),
        }
    }
}

impl std::error::Error for PrintError {}

impl From<io::Error> for PrintError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Somewhere a job's bytes can be written.
pub trait Sink {
    /// Take the whole job, or fail.
    ///
    /// # Errors
    ///
    /// Returns [`PrintError::Io`] when the sink cannot take the bytes, and
    /// nothing is left half written.
    fn write(&mut self, bytes: &[u8]) -> Result<(), PrintError>;
}

/// Print `job` to `sink`.
///
/// # Errors
///
/// Returns [`PrintError::Io`] when the sink fails, so a till can tell the
/// customer the receipt did not print.
pub fn print(job: &Job, sink: &mut impl Sink) -> Result<(), PrintError> {
    sink.write(job.bytes())
}

/// A file on disk, so a demo can print without a printer and a bug can be looked at.
#[derive(Debug)]
pub struct FileSink {
    path: PathBuf,
}

impl FileSink {
    /// Write jobs to `path`, creating or truncating it.
    #[must_use]
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// The file jobs go to.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Sink for FileSink {
    fn write(&mut self, bytes: &[u8]) -> Result<(), PrintError> {
        let mut file = File::create(&self.path)?;
        file.write_all(bytes)?;
        file.flush()?;
        Ok(())
    }
}

/// Standard output, for a till without a printer driver yet.
#[derive(Debug, Default)]
pub struct StdoutSink;

impl Sink for StdoutSink {
    fn write(&mut self, bytes: &[u8]) -> Result<(), PrintError> {
        io::stdout().write_all(bytes)?;
        Ok(())
    }
}

/// Bytes in memory, which is what a test prints to.
#[derive(Debug, Default)]
pub struct MemorySink {
    bytes: Vec<u8>,
}

impl MemorySink {
    /// An empty sink.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// What has been printed so far.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Sink for MemorySink {
    fn write(&mut self, bytes: &[u8]) -> Result<(), PrintError> {
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{FileSink, MemorySink, print};
    use crate::escpos::{Job, Paper};
    use crate::label::DosageLabel;

    #[test]
    fn a_job_reaches_the_sink_intact() {
        let mut sink = MemorySink::new();
        let job = DosageLabel::new("Paracetamol", "AMS-2404")
            .patient("Grace N.")
            .strength("500mg")
            .form("x100 tablets")
            .expiry("08/2027")
            .instructions("1 tablet twice a day")
            .price("TZS 12,000")
            .job();

        print(&job, &mut sink).expect("memory never fails");

        assert_eq!(sink.bytes(), job.into_bytes(), "the sink got the whole job");
    }

    #[test]
    fn a_receipt_is_a_job_for_the_roll() {
        let mut job = Job::new(Paper::Mm80);
        job.columns("Paracetamol 500mg x100", "12,000.00");
        assert_eq!(job.paper(), Paper::Mm80);
    }

    #[test]
    fn a_file_sink_writes_the_bytes() {
        let path = std::env::temp_dir().join("rok_pos_hardware_label.bin");
        let mut sink = FileSink::new(&path);
        let mut job = Job::new(Paper::Mm58);
        job.line("Afya Pharmacy");

        print(&job, &mut sink).expect("the temp file is writable");

        let written = std::fs::read(&path).expect("the file has the job");
        assert_eq!(written, job.into_bytes());
        assert_eq!(sink.path(), path.as_path());
        std::fs::remove_file(&path).ok();
    }
}
