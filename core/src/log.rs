use std::{collections::HashMap, fmt::Display};

use tracing_subscriber::{prelude::*, Layer};

use crate::{Config, EngineError};

pub fn init_logging(_config: &Config) -> Result<(), EngineError> {
    tracing_subscriber::registry().with(PrintLnLayer).init();

    Ok(())
}

pub struct PrintLnLayer;

impl<S> Layer<S> for PrintLnLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut fields = HashMap::new();
        let mut visitor = PrintLnVisitor(&mut fields);
        event.record(&mut visitor);

        let location =
            if let (Some(file), Some(line)) = (event.metadata().file(), event.metadata().line()) {
                format!("{}:{}", file, line)
            } else {
                "".to_string()
            };

        println!(
            "[{}][{}] {} - {}",
            event.metadata().level(),
            event.metadata().target(),
            fields
                .iter()
                .map(|(k, v)| { format!("{}={}", k, v) })
                .collect::<Vec<_>>()
                .join(" "),
            location,
        );
    }
}

enum LogValue {
    F64(f64),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    Bool(bool),
    String(String),
}

impl Display for LogValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogValue::F64(value) => write!(f, "{}", value),
            LogValue::I64(value) => write!(f, "{}", value),
            LogValue::U64(value) => write!(f, "{}", value),
            LogValue::I128(value) => write!(f, "{}", value),
            LogValue::U128(value) => write!(f, "{}", value),
            LogValue::Bool(value) => write!(f, "{}", value),
            LogValue::String(value) => write!(f, "\"{}\"", value),
        }
    }
}

struct PrintLnVisitor<'a>(&'a mut HashMap<&'static str, LogValue>);

impl<'a> tracing::field::Visit for PrintLnVisitor<'a> {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.0.insert(field.name(), LogValue::F64(value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0.insert(field.name(), LogValue::I64(value));
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.0.insert(field.name(), LogValue::U64(value));
    }

    fn record_i128(&mut self, field: &tracing::field::Field, value: i128) {
        self.0.insert(field.name(), LogValue::I128(value));
    }

    fn record_u128(&mut self, field: &tracing::field::Field, value: u128) {
        self.0.insert(field.name(), LogValue::U128(value));
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.0.insert(field.name(), LogValue::Bool(value));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0
            .insert(field.name(), LogValue::String(value.to_string()));
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        self.0
            .insert(field.name(), LogValue::String(value.to_string()));
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0
            .insert(field.name(), LogValue::String(format!("{:?}", value)));
    }
}
