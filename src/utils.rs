pub unsafe fn search_memory(start: *const (), len: usize, pattern: &[u8], must_match: &[bool]) -> Option<*const ()> {
	// BE WARY OF INT OVERFLOW
    let start = start as usize;
	let mut offset = 0usize;
	while offset + (pattern.len() as usize) < len {
        let mut pos = start + offset;
        let mut good = true;
        for (&byte, &must_match_byte) in pattern.iter().zip(must_match.iter()) {
            if byte != *(pos as *const u8) {
                if must_match_byte {
                    good = false;
                    break;
                }
            }
            pos += 1;
        }
        if good {
			return Some((start + offset) as *const ());
		}
		offset = offset + 1;
	}
	
	None
}
