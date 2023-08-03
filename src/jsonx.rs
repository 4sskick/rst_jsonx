use std::io::{BufReader, BufWriter, ErrorKind, Read, Write};

const BUF_SIZE: usize = 1024 * 16;

const C_CR: u8 = b'\r';
const C_LF: u8 = b'\n';
const C_TAB: u8 = b'\t';
const C_SPACE: u8 = b' ';

const C_COMMA: u8 = b',';
const C_COLON: u8 = b':';
const C_QUOTE: u8 = b'"';
const C_BACKSLASH: u8 = b'\\';

const C_LEFT_BRACE: u8 = b'{';
const C_LEFT_BRACKET: u8 = b'[';
const C_RIGHT_BRACE: u8 = b'}';
const C_RIGHT_BRACKET: u8 = b']';

pub struct Formatter {
    /// Used for beginning-of-line indentation in arrays and objects.
    pub indent: String,

    /// Used inside arrays and objects.
    pub line_separator: String,

    /// Used between root-level arrays and objects.
    pub record_separator: String,

    /// Used after a colon inside objects.
    pub after_colon: String,

    /// Used at very end of output.
    pub trailing_output: String,

    /// Add a record_separator as soon as a record ends, before seeing a
    /// subsequent record. Useful when there's a long time between records.
    pub eager_record_separators: bool,

    // private mutable state
    depth: usize,       // current nesting depth
    in_string: bool,    // is the next byte part of a string?
    in_backslash: bool, // does the next byte follow a backslash in a string?
    empty: bool,        // is the next byte in an empty object or array?
    first: bool,        // is this the first byte of input?
}

impl Formatter {
    fn default() -> Formatter {
        Formatter {
            indent: String::from("  "),
            line_separator: String::from("\n"),
            record_separator: String::from("\n"),
            after_colon: String::from(" "),
            trailing_output: String::from(""),
            eager_record_separators: false,
            depth: 0,
            in_string: false,
            in_backslash: false,
            empty: false,
            first: true,
        }
    }

    pub fn minimizer() -> Formatter {
        let mut fmt = Formatter::default();
        fmt.indent = String::from("");
        fmt.line_separator = String::from("");
        fmt.record_separator = String::from("\n");
        fmt.after_colon = String::from("");
        fmt
    }

    pub fn do_format(
        &mut self,
        input: &mut dyn Read,
        output: &mut dyn Write,
    ) -> Result<(), std::io::Error> {
        let mut reader = BufReader::new(input);
        let mut writer = BufWriter::new(output);

        self.do_format_stream(&mut reader, &mut writer);

        Err(std::io::Error::new(
            ErrorKind::BrokenPipe,
            "Dummy rror broken pipe",
        ))
    }

    fn do_format_stream(
        &mut self,
        input: &mut impl Read,
        output: &mut impl Write,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
}
