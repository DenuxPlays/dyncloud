use crate::Verbosity;
use anstyle::{AnsiColor, Color, Style};
use clap_verbosity_flag::VerbosityFilter;

// const TRACE_STYLE: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Magenta)));
const DEBUG_STYLE: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue)));
const INFO_STYLE: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green)));
const SUCCESS_STYLE: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))).bold();
// const WARN_STYLE: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow))).bold();
const ERROR_STYLE: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Red))).bold();

/**
The CliWriter is a kind of simple logger that writes styled messages to the command line interface (CLI).
*/
#[derive(Debug, Copy, Clone)]
pub(crate) struct CliWriter {
    filter_u8: u8,
}

impl CliWriter {
    pub(crate) fn new(verbosity: &Verbosity) -> Self {
        Self {
            filter_u8: Self::verbosity_filter_to_u8(&verbosity.filter()),
        }
    }

    // pub(crate) fn trace(&self, message: impl AsRef<str>) {
    //     self.write(message, &VerbosityFilter::Trace, &TRACE_STYLE);
    // }

    pub(crate) fn debug(&self, message: impl AsRef<str>) {
        self.write(message, &VerbosityFilter::Debug, &DEBUG_STYLE);
    }

    pub(crate) fn info(&self, message: impl AsRef<str>) {
        self.write(message, &VerbosityFilter::Info, &INFO_STYLE);
    }

    pub(crate) fn success(&self, message: impl AsRef<str>) {
        self.write(message, &VerbosityFilter::Info, &SUCCESS_STYLE);
    }

    // pub(crate) fn warn(&self, message: impl AsRef<str>) {
    //     self.write(message, &VerbosityFilter::Warn, &WARN_STYLE);
    // }

    pub(crate) fn error(&self, message: impl AsRef<str>) {
        self.write(message, &VerbosityFilter::Error, &ERROR_STYLE);
    }

    pub(crate) fn write(&self, message: impl AsRef<str>, level: &VerbosityFilter, style: &Style) {
        if self.should_print(level) {
            println!("{}{}{}", style.render(), message.as_ref(), style.render_reset());
        }
    }
    fn should_print(&self, level: &VerbosityFilter) -> bool {
        Self::verbosity_filter_to_u8(level) <= self.filter_u8
    }

    fn verbosity_filter_to_u8(filter: &VerbosityFilter) -> u8 {
        match filter {
            VerbosityFilter::Off => 0,
            VerbosityFilter::Error => 1,
            VerbosityFilter::Warn => 2,
            VerbosityFilter::Info => 3,
            VerbosityFilter::Debug => 4,
            VerbosityFilter::Trace => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_print_with_off_filter() {
        let writer = CliWriter {
            filter_u8: 0,
        };
        assert!(!writer.should_print(&VerbosityFilter::Error));
        assert!(!writer.should_print(&VerbosityFilter::Warn));
        assert!(!writer.should_print(&VerbosityFilter::Info));
        assert!(!writer.should_print(&VerbosityFilter::Debug));
        assert!(!writer.should_print(&VerbosityFilter::Trace));
    }

    #[test]
    fn test_should_print_with_error_filter() {
        let writer = CliWriter {
            filter_u8: 1,
        };
        assert!(writer.should_print(&VerbosityFilter::Error));
        assert!(!writer.should_print(&VerbosityFilter::Warn));
        assert!(!writer.should_print(&VerbosityFilter::Info));
        assert!(!writer.should_print(&VerbosityFilter::Debug));
        assert!(!writer.should_print(&VerbosityFilter::Trace));
    }

    #[test]
    fn test_should_print_with_warn_filter() {
        let writer = CliWriter {
            filter_u8: 2,
        };
        assert!(writer.should_print(&VerbosityFilter::Error));
        assert!(writer.should_print(&VerbosityFilter::Warn));
        assert!(!writer.should_print(&VerbosityFilter::Info));
        assert!(!writer.should_print(&VerbosityFilter::Debug));
        assert!(!writer.should_print(&VerbosityFilter::Trace));
    }

    #[test]
    fn test_should_print_with_info_filter() {
        let writer = CliWriter {
            filter_u8: 3,
        };
        assert!(writer.should_print(&VerbosityFilter::Error));
        assert!(writer.should_print(&VerbosityFilter::Warn));
        assert!(writer.should_print(&VerbosityFilter::Info));
        assert!(!writer.should_print(&VerbosityFilter::Debug));
        assert!(!writer.should_print(&VerbosityFilter::Trace));
    }

    #[test]
    fn test_should_print_with_debug_filter() {
        let writer = CliWriter {
            filter_u8: 4,
        };
        assert!(writer.should_print(&VerbosityFilter::Error));
        assert!(writer.should_print(&VerbosityFilter::Warn));
        assert!(writer.should_print(&VerbosityFilter::Info));
        assert!(writer.should_print(&VerbosityFilter::Debug));
        assert!(!writer.should_print(&VerbosityFilter::Trace));
    }

    #[test]
    fn test_should_print_with_trace_filter() {
        let writer = CliWriter {
            filter_u8: 5,
        };
        assert!(writer.should_print(&VerbosityFilter::Error));
        assert!(writer.should_print(&VerbosityFilter::Warn));
        assert!(writer.should_print(&VerbosityFilter::Info));
        assert!(writer.should_print(&VerbosityFilter::Debug));
        assert!(writer.should_print(&VerbosityFilter::Trace));
    }
}
