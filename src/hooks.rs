use INTERFACES;
use OFFSETS;
use sdk;
use sdk::Vector;
use std::mem;
use libc;
use offsets::ptr_offset;
use vmthook;

pub unsafe fn install_client() {
    let mut hooker = vmthook::VMTHooker::new(INTERFACES.client as *mut _);
    REAL_CREATEMOVE = hooker.get_orig_method(21);
    hooker.hook(21, mem::transmute::<_, *const ()>(hooked_createmove));

    INTERFACES.input = locate_cinput(REAL_CREATEMOVE).unwrap();

    let mut hooker = vmthook::VMTHooker::new(INTERFACES.input as *mut _);
    hooker.hook(8, mem::transmute::<_, *const ()>(hooked_getusercmd));
}

pub static mut REAL_CREATEMOVE: *const () = 0 as *const ();

type CreateMoveFn = unsafe extern "stdcall" fn(libc::c_int,
                                               libc::c_float,
                                               bool);

unsafe extern "stdcall" fn hooked_getusercmd(sequence_number: libc::c_int) -> *mut sdk::CUserCmd {
    let cmds = *((INTERFACES.input as usize + 0xC4) as *const *mut sdk::CUserCmd);
    cmds.offset((sequence_number % 90) as isize)
}

unsafe extern "stdcall" fn hooked_createmove(sequence_number: libc::c_int,
                                      input_sample_frametime: libc::c_float,
                                      active: bool)
{
    let mut ebp: usize;
    asm!("movl %ebp, $0"
         : "=r"(ebp)
         :
         :
         );
    let ebp = ebp as *mut *mut (); 
    mem::transmute::<_, CreateMoveFn>(REAL_CREATEMOVE)(sequence_number,
                    input_sample_frametime,
                    active);

    let sendpacket_ptr = ptr_offset::<_, bool>(*ebp, -1);
    let cmds = *((INTERFACES.input as usize + 0xC4) as *const *mut sdk::CUserCmd);
    let cmd = cmds.offset((sequence_number % 90) as isize);

    let me_idx = sdk::EngineClient_GetLocalPlayer(INTERFACES.engine);
    let me = sdk::CEntList_GetClientEntity(INTERFACES.entlist, me_idx);
    let meorigin = sdk::CBaseEntity_GetAbsOrigin(me);
    let eyes = *meorigin + *ptr_offset::<_, Vector>(me, OFFSETS.m_vecViewOffset);
    let myteam = *ptr_offset::<_, libc::c_int>(me, OFFSETS.m_iTeamNum);

    for ent in (1..32) 
        .filter(|&idx| idx != me_idx)
            .map(|idx| sdk::CEntList_GetClientEntity(INTERFACES.entlist, idx))
            .filter(|ent| !ent.is_null())
            {
                let origin = sdk::CBaseEntity_GetAbsOrigin(ent);

                let friendly = *ptr_offset::<_, libc::c_int>(ent, OFFSETS.m_iTeamNum) == myteam;
                let dormant = sdk::CBaseEntity_IsDormant(ent); 
                if dormant || friendly { continue }

                *ptr_offset::<_, bool>(ent, 0x0DB4) = true;
                sdk::CBaseEntity_UpdateGlowEffect(ent);
                
            }
    if ::triggerbot::should_trigger(me, eyes, (*cmd).viewangles) {
        (*cmd).buttons |= 1;
    }
    let flags = *ptr_offset::<_, i32>(me, OFFSETS.m_fFlags);
    if flags & 1 == 0 {
        (*cmd).buttons &= !(4);
    }

    if flags & 1 == 0 || flags & 2 != 0 { 
        (*cmd).buttons &= !(2);
    }

    static mut FLIP: bool = false;
    FLIP = !FLIP;
    
    static mut ANG_ACCUM: f32 = 0.0;
    if false && (*cmd).buttons & 1 == 0 {
        use std::f32::consts::PI;
        ANG_ACCUM = (ANG_ACCUM + (1.0 / 7.0 * PI)) % (2.0 * PI); 
        let newang = ANG_ACCUM;
        let angdelta = newang - (*cmd).viewangles.yaw.to_radians(); 

        (*cmd).viewangles.yaw = newang.to_degrees(); 
        (*cmd).viewangles.pitch = if FLIP {
            89.0
        } else {
            -89.0
        };

        let (fm, sm) = ((*cmd).forwardmove, (*cmd).sidemove);
        
        (*cmd).forwardmove = fm * angdelta.cos() - sm * angdelta.sin();
        (*cmd).sidemove = fm * angdelta.sin() + sm * angdelta.cos();
    }
    if (*cmd).viewangles.pitch > 90.0 {
        (*cmd).viewangles.pitch = 90.0;
    }
    if (*cmd).viewangles.pitch < -90.0 {
        (*cmd).viewangles.pitch = -90.0;
    }
    if (*cmd).viewangles.yaw < 0.0 {
        (*cmd).viewangles.yaw += 360.0;
    }
    if (*cmd).viewangles.yaw > 360.0 {
        (*cmd).viewangles.yaw -= 360.0;
    }
         

    (*cmd).command_number = 2076615043;
    (*cmd).random_seed = 39;
    let verified_cmds = *((INTERFACES.input as usize + 0xC8) as *const *mut sdk::CVerifiedUserCmd);
    let verified_cmd = verified_cmds.offset((sequence_number % 90) as isize);
    (*verified_cmd).m_cmd = *cmd;
    verify_usercmd(verified_cmd);
}

unsafe fn verify_usercmd(verified_cmd: *mut sdk::CVerifiedUserCmd) {
    // LOL

    use std::slice::from_raw_parts;
    use std::mem::size_of;
    let cmd = &((*verified_cmd).m_cmd);

    let mut buf = vec![];
    buf.push_all(from_raw_parts(
            &(*cmd).command_number as *const _ as *const u8,
            size_of::<i32>()));
    buf.push_all(from_raw_parts(
            &(*cmd).tick_count as *const _ as *const u8,
            size_of::<i32>()));
    buf.push_all(from_raw_parts(
            &(*cmd).viewangles as *const _ as *const u8,
            size_of::<sdk::QAngle>()));

    buf.push_all(from_raw_parts(
            &(*cmd).forwardmove as *const _ as *const u8,
            size_of::<f32>()));
    buf.push_all(from_raw_parts(
            &(*cmd).sidemove as *const _ as *const u8,
            size_of::<f32>()));
    buf.push_all(from_raw_parts(
            &(*cmd).upmove as *const _ as *const u8,
            size_of::<f32>()));
    buf.push_all(from_raw_parts(
            &(*cmd).buttons as *const _ as *const u8,
            size_of::<i32>()));

    buf.push_all(from_raw_parts(
            &(*cmd).impulse as *const _ as *const u8,
            size_of::<u8>()));
    buf.push_all(from_raw_parts(
            &(*cmd).weaponselect as *const _ as *const u8,
            size_of::<i32>()));
    buf.push_all(from_raw_parts(
            &(*cmd).weaponsubtype as *const _ as *const u8,
            size_of::<i32>()));

    buf.push_all(from_raw_parts(
            &(*cmd).random_seed as *const _ as *const u8,
            size_of::<i32>()));

    buf.push_all(from_raw_parts(
            &(*cmd).mousedx as *const _ as *const u8,
            size_of::<u16>()));
    buf.push_all(from_raw_parts(
            &(*cmd).mousedy as *const _ as *const u8,
            size_of::<u16>()));

    let checksum = ::crc::crc32::checksum_ieee(&buf);
    (*verified_cmd).m_crc = checksum;
}
unsafe fn locate_cinput(createmove: *const ()) -> Option<*mut sdk::CInput> {
    let result = ::utils::search_memory(createmove, 100, &[0x8B, 0x0D]);
    match result {
        Some(ptr) => {
            let load_instruction_operand = ((ptr as usize) + 2) as *const *const *mut sdk::CInput;
            let cinput_ptr_ptr = *load_instruction_operand;
            Some((*cinput_ptr_ptr))
        },
        None => None 
    }
}
