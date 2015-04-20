use libc;
use sdk::{CEngineTrace, CEntList, CHLClient, CInput, DebugOverlay, EngineClient};
use std;
use winapi;
use kernel32;

#[allow(non_snake_case)]
type CreateInterfaceFn = extern "C" fn(
    pName: *const libc::c_char,
    pReturnCode: *mut libc::c_int
) -> *mut libc::c_void;

/*unsafe fn get_vfunc(ptr: *mut (), index: int) -> *const () {
    let vtptr = ptr as *mut *const ();
    *vtptr.offset(index)
}*/

#[derive(Debug)]
pub struct Interfaces {
    pub engine: *mut EngineClient,
    pub client: *mut CHLClient,
    pub trace: *mut CEngineTrace,
    pub entlist: *mut CEntList,
    pub debugoverlay: *mut DebugOverlay,
    pub input: *mut CInput 
}
unsafe impl Send for Interfaces {}
unsafe impl Sync for Interfaces {}

impl Interfaces {
    pub unsafe fn load(&mut self) { 
        let client_factory = get_factory_from_dll("client.dll");
        let engine_factory = get_factory_from_dll("engine.dll");

        *self = Interfaces {
            engine: get_interface_from_factory("VEngineClient014", engine_factory) as *mut EngineClient,
            client: get_interface_from_factory("VClient017", client_factory) as *mut CHLClient,
            trace: get_interface_from_factory("EngineTraceClient003", engine_factory) as *mut CEngineTrace,
            entlist: get_interface_from_factory("VClientEntityList003", client_factory) as *mut CEntList,
            debugoverlay: get_interface_from_factory("VDebugOverlay003", engine_factory) as *mut DebugOverlay,
            .. *self
        }
    }
}

fn get_interface_from_factory(iface: &str, factory: CreateInterfaceFn) -> *mut (){
    let mut status = 0;
    let iface = std::ffi::CString::new(iface).unwrap();
    let result = factory(iface.as_ptr(), &mut status as *mut _);
    if status == 0 && !result.is_null() {
        result as *mut ()
    } else {
        ::show_popup(&format!("Interface not found: {:?}", &iface));
        panic!()
    }
}

fn get_factory_from_dll(name: &str) -> CreateInterfaceFn {
    unsafe {
        let module = get_module(name);
        let createinterface = ::std::ffi::CString::new("CreateInterface").unwrap();

        let factory = 
            kernel32::GetProcAddress(
                module,
                createinterface.as_ptr() 
                );
        std::mem::transmute::<_, CreateInterfaceFn>(factory)
    }
}

pub fn get_module(name: &str) -> winapi::HMODULE {
    let name = ::std::ffi::CString::new(name).unwrap();
    loop {
        let handle = unsafe {
            kernel32::GetModuleHandleA(name.as_ptr())
        };
        if !handle.is_null() {
            return handle;
        }
    }
}

pub static mut INTERFACES: Interfaces = Interfaces {
    engine: 0 as *mut _,
    client: 0 as *mut _,
    trace: 0 as *mut _,
    entlist: 0 as *mut _,
    debugoverlay: 0 as *mut _,
    input: 0 as *mut _
};
