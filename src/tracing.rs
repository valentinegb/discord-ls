use std::fmt::{self, Debug, Formatter, Write as _};

use tokio::runtime;
use tower_lsp::{Client, lsp_types::MessageType};
use tracing::{
    Level, Subscriber,
    field::{Field, Visit},
    span::{self, Attributes, Record},
};

struct LspVisitor(String);

impl LspVisitor {
    fn new() -> Self {
        Self(String::new())
    }
}

impl Visit for LspVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        if !self.0.is_empty() {
            write!(&mut self.0, " ").unwrap();
        }

        if field.name() != "message" {
            write!(&mut self.0, "{}=", field.name()).unwrap();
        }

        write!(&mut self.0, "{value:?}").unwrap();
    }
}

impl fmt::Display for LspVisitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A very minimal [`tracing::Subscriber`] that just logs events through LSP via
/// the `window/logMessage` notification.
pub(super) struct LspSubscriber {
    language_client: Client,
    rt: runtime::Handle,
    assigned_ids: std::sync::Mutex<u64>,
}

impl LspSubscriber {
    /// Creates a new [`LspSubscriber`].
    ///
    /// # Panics
    ///
    /// This will panic if called outside the context of a Tokio runtime. That means that you must
    /// call this on one of the threads **being run by the runtime**, or from a thread with an active
    /// `EnterGuard`. Calling this from within a thread created by `std::thread::spawn` (for example)
    /// will cause a panic unless that thread has an active `EnterGuard`.
    pub(super) fn new(language_client: tower_lsp::Client) -> Self {
        Self {
            language_client,
            rt: tokio::runtime::Handle::current(),
            assigned_ids: std::sync::Mutex::new(0),
        }
    }
}

impl Subscriber for LspSubscriber {
    fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
        *metadata.level() < Level::TRACE
    }

    fn new_span(&self, _span: &Attributes<'_>) -> span::Id {
        let mut assigned_ids = self.assigned_ids.lock().unwrap();
        let id = span::Id::from_u64(*assigned_ids);

        *assigned_ids += 1;

        id
    }

    fn record(&self, _span: &span::Id, _values: &Record<'_>) {
        // Noop
    }

    fn record_follows_from(&self, _span: &span::Id, _follows: &span::Id) {
        // Noop
    }

    fn event(&self, event: &tracing::Event<'_>) {
        let level = event.metadata().level();
        let message_type = match *level {
            Level::ERROR => MessageType::ERROR,
            Level::WARN => MessageType::WARNING,
            Level::INFO => MessageType::INFO,
            _ => MessageType::LOG,
        };
        let target = event.metadata().target();
        let mut visitor = LspVisitor::new();

        event.record(&mut visitor);
        self.rt.spawn_blocking({
            let rt = self.rt.clone();
            let language_client = self.language_client.clone();

            move || {
                rt.block_on(async {
                    language_client
                        .log_message(message_type, format!("{level:>5} {target}: {visitor}"))
                        .await;
                });
            }
        });
    }

    fn enter(&self, _span: &span::Id) {
        // Noop
    }

    fn exit(&self, _span: &span::Id) {
        // Noop
    }
}
