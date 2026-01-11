use crate::void;

// https://www.geoffchappell.com/studies/windows/win32/ntdll/api/index.htm
#[link(name = "ntdll")]
unsafe extern "system" {
	pub fn NtQuerySystemInformation(
		SystemInformationClass: u32,
		SystemInformation: *mut void,
		SystemInformationLength: u32,
		ReturnLength: *mut u32,
	) -> i32;
}
