use common::write;
use log::{Metadata, Log, Record};

pub struct Logger {}

impl Log for Logger
{
	fn enabled(&self, metadata: &Metadata) -> bool { true }

	fn log(&self, record: &Record)
	{
		let message = format!("{}\n", record.level().to_string());
		write!("{}", message);
	}

	fn flush(&self) {}
}
