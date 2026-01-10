// https://github.com/SCLorentz/UwU/blob/main/ARM64/src/main.s

#[macro_export]
macro_rules! write {
	($($x:expr),+ $(,)?) => {
		let s = format!($($x),+);
		let ptr = s.as_ptr();
		let len = s.len();

		#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
		unsafe {
			core::arch::asm!(
				"syscall",
				in("rax") 1,		// SYS_write
				in("rdi") 1,		// stdout
				in("rsi") ptr,
				in("rdx") len,
			)
		}

		#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
		unsafe {
			core::arch::asm!(
				"mov x16, #4",
				"mov x0, #1",
				"svc #0x80",
				in("x1") ptr,
				in("x2") len,
				options(nostack)
			)
		}
	};
}

#[macro_export]
macro_rules! exit {
	($x:expr) => {
		const ret: i32 = $x;

		// https://godbolt.org/ (std::process::exit())
		#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
		unsafe {
			core::arch::asm!(
				"syscall",
				"push rax",
				in("edi") ret,
			)
		}

		#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
		unsafe {
			core::arch::asm!(
				"mov x16, #1",
				"svc 0x80",
				in("x1") ret,
				options(nostack)
			);
		}
	}
}
