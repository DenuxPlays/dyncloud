use crate::Verbosity;

pub(crate) fn init_tracing(verbosity: &Verbosity) {
    tracing_subscriber::fmt().with_max_level(verbosity.filter()).init();
}
