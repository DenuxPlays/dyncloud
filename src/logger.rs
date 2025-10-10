use crate::Verbosity;
use crate::io_helper::CliWriter;
use clap_verbosity_flag::VerbosityFilter;
use tracing::metadata::LevelFilter;

pub(crate) fn init_logging(verbosity: &Verbosity, debug: bool) -> CliWriter {
    let state = State::from_verbosity_and_debug_flag(verbosity, debug);
    let level_filter = calculate_level_filter(state);

    init_tracing(level_filter);

    CliWriter::new(verbosity)
}

pub(crate) fn init_tracing(filter: impl Into<LevelFilter>) {
    tracing_subscriber::fmt().with_max_level(filter).init();
}

pub(crate) fn calculate_level_filter(state: State) -> LevelFilter {
    match state {
        State::Silent => LevelFilter::ERROR,
        State::Debug => LevelFilter::DEBUG,
        State::Normal => LevelFilter::INFO,
        State::Warn => LevelFilter::WARN,
    }
}

pub(crate) enum State {
    Silent,
    Normal,
    Warn,
    Debug,
}

impl State {
    pub(crate) fn from_verbosity_and_debug_flag(verbosity: &Verbosity, debug: bool) -> Self {
        if debug {
            return Self::Debug;
        }

        match verbosity.filter() {
            VerbosityFilter::Off | VerbosityFilter::Error => Self::Silent,
            VerbosityFilter::Warn => Self::Warn,
            _ => Self::Normal,
        }
    }
}
