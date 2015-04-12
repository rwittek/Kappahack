use libc;

pub unsafe fn search_memory(start: *const (), len: usize, pattern: &[u8]) -> Option<*const ()> {
	// BE WARY OF INT OVERFLOW
    let start = start as usize;
	let mut offset = 0us;
	while offset + (pattern.len() as usize) < len {
		if libc::memcmp((start + offset) as *const libc::c_void, pattern.as_ptr() as *const libc::c_void, pattern.len() as u32) == 0 {
			return Some((start + offset) as *const ());
		}
		offset = offset + 1;
	}
	
	None
}
