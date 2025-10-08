use crate::Verbosity;
use anstyle::{AnsiColor, Style};
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::fmt::format::Writer;

pub(crate) fn init_tracing(verbosity: &Verbosity) {
    tracing_subscriber::fmt()
        .with_max_level(verbosity.filter())
        .without_time()
        .with_target(false)
        .event_format(OnlyMessageFormatter)
        .init();
}

struct OnlyMessageFormatter;

impl<S, N> fmt::FormatEvent<S, N> for OnlyMessageFormatter
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &fmt::FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let style = match *event.metadata().level() {
            Level::ERROR => Style::new().fg_color(Some(AnsiColor::Red.into())).bold(),
            Level::WARN => Style::new().fg_color(Some(AnsiColor::Yellow.into())).bold(),
            Level::INFO => Style::new().fg_color(Some(AnsiColor::Green.into())),
            Level::DEBUG => Style::new().fg_color(Some(AnsiColor::Blue.into())),
            Level::TRACE => Style::new().fg_color(Some(AnsiColor::Magenta.into())),
        };

        write!(writer, "{}", style.render())?;
        ctx.format_fields(writer.by_ref(), event)?;
        writeln!(writer, "{}", style.render_reset())
    }
}
