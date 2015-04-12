use std::mem;
use std::ptr;
use libc;

const VMT_MAX_SIZE_YOLO: usize = 512;
type VMT = [*const (); VMT_MAX_SIZE_YOLO];

pub struct VMTHooker { // this should be renamed.......
	_original_vmt_ptr_ptr: *const VMT,
	original_vmt: VMT,
	patched_vmt_ptr: *mut VMT,
}

impl VMTHooker {
	pub unsafe fn new(vmt_ptr_ptr: *mut *const ()) -> VMTHooker {
		let vmt_ptr: *const VMT = mem::transmute(*vmt_ptr_ptr);
		// yes, we do leak this.
		// yolo.
		let new_vmt = libc::malloc(mem::size_of::<VMT>() as libc::size_t) as *mut VMT;
		if new_vmt.is_null() {
            ::show_popup("Failed to malloc() a VMT!");
            panic!()
		}
		*new_vmt = *vmt_ptr;
		
		let hooker = VMTHooker {
			_original_vmt_ptr_ptr: mem::transmute(vmt_ptr_ptr),
			original_vmt: *vmt_ptr,
			patched_vmt_ptr: new_vmt
		};
		
		*vmt_ptr_ptr = (new_vmt) as *const VMT as *const ();
		
		hooker
	}
	
	pub unsafe fn hook(&mut self, offset: usize, hook: *const ()) {
		(*(self.patched_vmt_ptr))[offset] = hook;
	}
	
	pub unsafe fn get_orig_method(&self, offset: usize) -> *const () {
		self.original_vmt.get(offset).map(|&meth| meth).unwrap_or(ptr::null())
	}
}
