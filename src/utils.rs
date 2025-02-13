///macro utils



#[macro_export]
macro_rules! poly {
    ($($val:expr),* $(,)?) => {
        Polynomial::new(vec![$($val.into(),)*])
    }
}

// let p = poly![1, 3, 5]; 
#[macro_export]
macro_rules! fe {
    ($modulus:expr, $value:expr) => {
        FieldElement::<$modulus>::new($value)
    };
}
// let a = fe!(7, 3);
#[macro_export]
macro_rules! field {
    ($name:ident, $modulus:expr) => {
        type $name = FieldElement<$modulus>;
    };
}
// field!(Field7, 7);
// let b = Field7::new(5);


///logger 
use tracing_subscriber::{
    fmt::{self, format::Writer, time::ChronoLocal, FmtContext, FormatEvent, FormatFields},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
    registry::LookupSpan
};
use tracing_core::{Subscriber,Event};
use tracing_appender::rolling;
use chrono::Local;
use tracing_subscriber::fmt::time::FormatTime;
use std::fmt as fmmt;

/// `[timestamp] [LEVEL] [thread THREAD_ID] file:line - message`
struct CustomFormatter;

impl<S, N> FormatEvent<S, N> for CustomFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>, // Writer is NOT Copy
        event: &Event<'_>,
    ) -> fmmt::Result {
        let now = Local::now().format("%Y-%m-%dT%H:%M:%S");
        let metadata = event.metadata();
        let level = metadata.level();
        let file = metadata.file().unwrap_or("<unknown>");
        let line = metadata.line().unwrap_or(0);
        let thread_id = format!("{:?}", std::thread::current().id());

        // Write formatted log prefix
        write!(
            &mut writer,
            "[{}] [{:>5}] [thread {}] {}:{} - ",
            now, level, thread_id, file, line
        )?;

        // Format the event fields
        ctx.format_fields(writer.by_ref(), event)?; 
        writeln!(writer) 
        
    }
}
pub fn setup_tracing() {
    // File-based logging (daily rotation)
    let file_appender = rolling::daily("logs", "output.log");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    // Log format for stdout
    let stdout_layer = fmt::layer()
        .event_format(CustomFormatter) // Use custom log formatter
        .with_writer(std::io::stdout);

    // Log format for file (no ANSI colors)
    let file_layer = fmt::layer()
        .event_format(CustomFormatter)
        .with_writer(file_writer)
        .with_ansi(false); // Disable colors for file logs

    // Allow dynamic filtering via `RUST_LOG`
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Register logging layers
    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .init();
}
