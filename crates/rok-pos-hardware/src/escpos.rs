//! ESC/POS: the byte language thermal printers speak.
//!
//! A [`Job`] is built as bytes and nothing else, so the printer is the last
//! thing in the chain and the job can be tested headless: assert on the bytes.
//!
//! ```
//! use rok_pos_hardware::escpos::{Job, Paper};
//!
//! let mut job = Job::new(Paper::Mm80);
//! job.center("Afya Pharmacy").line("AMS-2404").cut();
//!
//! assert_eq!(&job.into_bytes()[..2], b"\x1b@", "a job starts by resetting the printer");
//! ```
//!
//! Text is ASCII, which is what the plan asks for and what a thermal head prints
//! without a font table. Anything else is written as `?` rather than mangled.

/// `ESC`: starts the two-byte commands.
pub const ESC: u8 = 0x1b;
/// `GS`: starts the group commands, which is where barcode and cut live.
pub const GS: u8 = 0x1d;

/// The paper a receipt is cut for. Thermal printers measure width in millimetres
/// but print in columns, and these are the two widths a pharmacy till has.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Paper {
    /// 58 mm receipt roll, 32 columns of 12 dot font A.
    Mm58,
    /// 80 mm receipt roll, 48 columns of 12 dot font A.
    Mm80,
}

impl Paper {
    /// The columns that fit on the roll.
    #[must_use]
    pub const fn columns(self) -> usize {
        match self {
            Self::Mm58 => 32,
            Self::Mm80 => 48,
        }
    }
}

/// Where a line starts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    /// Flush left, the default.
    Left,
    /// Centered, for headings.
    Center,
    /// Flush right, for amounts.
    Right,
}

impl Align {
    const fn command(self) -> u8 {
        match self {
            Self::Left => 0,
            Self::Center => 1,
            Self::Right => 2,
        }
    }
}

/// One print job: bytes in, paper out.
#[derive(Clone, Debug)]
pub struct Job {
    paper: Paper,
    bytes: Vec<u8>,
}

impl Job {
    /// Start a job, resetting the printer first so the last job's settings are
    /// gone whatever happened before.
    #[must_use]
    pub fn new(paper: Paper) -> Self {
        let mut bytes = vec![ESC, b'@'];
        bytes.extend([ESC, 0x74, 0x10]); // code page: WPC1252
        Self { paper, bytes }
    }

    /// The roll this job is cut for.
    #[must_use]
    pub const fn paper(&self) -> Paper {
        self.paper
    }

    /// The bytes to hand to the printer.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Take the bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Set where following lines start.
    pub fn align(&mut self, align: Align) -> &mut Self {
        self.bytes.extend([ESC, b'a', align.command()]);
        self
    }

    /// Turn bold on or off.
    pub fn bold(&mut self, on: bool) -> &mut Self {
        self.bytes.extend([ESC, b'E', u8::from(on)]);
        self
    }

    /// Double the width of what follows, as a receipt total wants.
    pub fn double_width(&mut self, on: bool) -> &mut Self {
        self.bytes.extend([ESC, b'W', u8::from(on)]);
        self
    }

    /// Print one line of text, cut to the roll's width: a thermal head wraps rather
    /// than truncates, and a wrapped label is a label no one can read.
    pub fn line(&mut self, text: &str) -> &mut Self {
        self.bytes
            .extend(take_ascii(text, self.paper.columns()).into_bytes());
        self.bytes.push(b'\n');
        self
    }

    /// Print centered text.
    pub fn center(&mut self, text: &str) -> &mut Self {
        self.align(Align::Center).line(text)
    }

    /// Print right-aligned text.
    pub fn right(&mut self, text: &str) -> &mut Self {
        self.align(Align::Right).line(text)
    }

    /// Print a left and a right column on one line: `left` and `right` are
    /// truncated to fit, then the line is padded to exactly the roll's width.
    pub fn columns(&mut self, left: &str, right: &str) -> &mut Self {
        self.align(Align::Left);
        let width = self.paper.columns();
        let right = take_ascii_right(right, width);
        let gap = width - right.len();
        let left = take_ascii(left, gap);
        let padding = " ".repeat(gap - left.len());
        self.line(&format!("{left}{padding}{right}"));
        self
    }

    /// Print `width` blank columns, one line each.
    pub fn feed(&mut self, lines: usize) -> &mut Self {
        for _ in 0..lines {
            self.bytes.push(b'\n');
        }
        self
    }

    /// Print a Code 128 barcode: the batch code and expiry as a label, so the
    /// cashier scans instead of typing.
    pub fn barcode(&mut self, data: &str) -> &mut Self {
        self.bytes.extend(code128b(data));
        self
    }

    /// Cut the receipt, feeding it out of the way first so the cut is complete.
    pub fn cut(&mut self) -> &mut Self {
        self.feed(3);
        self.bytes.extend([GS, b'V', 66, 0]);
        self
    }
}

/// One ASCII byte, or `?` for anything a 58 mm thermal head cannot print.
fn ascii(character: char) -> u8 {
    if character.is_ascii() {
        character as u8
    } else {
        b'?'
    }
}

/// Take `width` ASCII bytes from the left of `text`, padding nothing.
fn take_ascii(text: &str, width: usize) -> String {
    let bytes: Vec<u8> = text.chars().map(ascii).take(width).collect();
    String::from_utf8(bytes).unwrap_or_default()
}

/// Take `width` ASCII bytes from the right of `text`.
fn take_ascii_right(text: &str, width: usize) -> String {
    let bytes: Vec<u8> = text
        .chars()
        .rev()
        .map(ascii)
        .take(width)
        .collect::<Vec<u8>>()
        .into_iter()
        .rev()
        .collect();
    String::from_utf8(bytes).unwrap_or_default()
}

/// The `GS k` barcode command for Code 128 set B, which covers every character a
/// batch code and a price use.
#[must_use]
pub fn code128b(data: &str) -> Vec<u8> {
    let values: Vec<u8> = data.chars().map(|character| character as u8 - 32).collect();
    let mut bytes = vec![GS, b'k', 73];
    bytes.extend([104u8]); // switch to set B
    bytes.extend(&values);

    let mut checksum = 104u32;
    for (weight, value) in (1u32..).zip(&values) {
        checksum += u32::from(*value) * weight;
    }
    bytes.push((checksum % 103) as u8);
    bytes.extend([106, 0]); // stop pattern, then NUL
    bytes
}

/// The printable text of a job: escape sequences dropped, so a test can look at
/// what the paper shows.
#[cfg(test)]
pub(crate) fn printed(job: &Job) -> String {
    String::from_utf8_lossy(job.bytes())
        .chars()
        .filter(|character| character.is_ascii_graphic() || *character == ' ')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Align, Job, Paper, code128b, printed};

    #[test]
    fn a_job_resets_the_printer_before_anything_else() {
        let job = Job::new(Paper::Mm80);
        assert_eq!(&job.into_bytes()[..2], b"\x1b@");
    }

    #[test]
    fn columns_fill_the_roll_exactly() {
        let mut job = Job::new(Paper::Mm80);
        job.columns("AMS-2404", "58,800.00");
        let text = printed(&job);
        let gap = Paper::Mm80.columns() - "AMS-2404".len() - "58,800.00".len();
        assert!(
            text.contains(&format!("AMS-2404{:gap$}58,800.00", "")),
            "name left, amount right, the gap filled: {text:?}"
        );

        let mut job = Job::new(Paper::Mm58);
        job.columns("Paracetamol 500mg x100", "58,800.00");
        let text = printed(&job);
        let gap = Paper::Mm58.columns() - "Paracetamol 500mg x100".len() - "58,800.00".len();
        assert!(
            text.contains(&format!("Paracetamol 500mg x100{:gap$}58,800.00", "")),
            "a name that fits 58 mm still lines up: {text:?}"
        );
    }

    #[test]
    fn bold_and_alignment_are_toggled_not_stacked() {
        let mut job = Job::new(Paper::Mm80);
        job.bold(true)
            .center("TOTAL")
            .bold(false)
            .align(Align::Left)
            .line("120,000.00");
        let text = String::from_utf8(job.into_bytes()).expect("ASCII");
        assert_eq!(
            text.matches("\x1bE\x01").count(),
            1,
            "bold on once: {text:?}"
        );
        assert_eq!(
            text.matches("\x1bE\x00").count(),
            1,
            "bold off once: {text:?}"
        );
        assert_eq!(
            text.matches("\x1ba\x01").count(),
            1,
            "center once: {text:?}"
        );
    }

    #[test]
    fn a_receipt_ends_with_a_cut() {
        let mut job = Job::new(Paper::Mm80);
        job.line("Afya Pharmacy").cut();
        let bytes = job.into_bytes();
        assert_eq!(&bytes[bytes.len() - 4..], b"\x1dV\x42\x00");
    }

    #[test]
    fn a_barcode_is_code_128_set_b_with_its_checksum() {
        assert_eq!(
            code128b("P12345"),
            vec![0x1d, b'k', 73, 104, 48, 17, 18, 19, 20, 21, 27, 106, 0]
        );
    }

    #[test]
    fn non_ascii_is_printed_as_a_question_mark() {
        let mut job = Job::new(Paper::Mm58);
        job.line("TZS 12,000 / \u{20ac}");
        let text = String::from_utf8(job.into_bytes()).expect("ASCII");
        assert!(
            text.contains("TZS 12,000 / ?"),
            "unprintable is a '?': {text:?}"
        );
    }
}
