fn main() {
	if std::env::var("CARGO_CFG_DOC").is_ok() {
		return;
	}
}
