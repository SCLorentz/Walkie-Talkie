//https://github.com/matrix-org/matrix-rust-sdk/blob/main/crates/matrix-sdk/src/account.rs

// https://github.com/matrix-org/matrix-rust-sdk/blob/cd9f433358586e8717417fc043650f46362aa14c/bindings/matrix-sdk-ffi/src/client.rs#L259
#[derive(uniffi::Object)]
pub struct Client {
	pub(crate) inner: AsyncRuntimeDropped<MatrixClient>,

	delegate: OnceLock<Arc<dyn ClientDelegate>>,

	pub(crate) utd_hook_manager: OnceLock<Arc<UtdHookManager>>,

	session_verification_controller:
		Arc<tokio::sync::RwLock<Option<SessionVerificationController>>>,

	store_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Account {
	/// The underlying HTTP client.
	client: Client,
}

impl Account {
	pub fn new(client: Client) -> Self
	{
		Self { client }
	}
}
