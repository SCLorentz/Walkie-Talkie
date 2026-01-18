// https://github.com/SCLorentz/UwU/blob/main/ARM64/src/main.s

mod bindings {
	unsafe extern "C" {
		pub(crate) fn exit(code: crate::f8) -> !;
	}
}

/// Exits the program with a specified exit code
#[allow(unused)]
#[inline]
pub fn exit(code: crate::f8) -> ! { unsafe { bindings::exit(code) } }

#[macro_export]
macro_rules! write {
	($($x:expr),+ $(,)?) => {
		let s = alloc::format!($($x),+);
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
