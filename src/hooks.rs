#![allow(unused_variables)]
use INTERFACES;
use OFFSETS;
use sdk;
use sdk::Vector;
use std::mem;
use libc;
use offsets::ptr_offset;
use interfaces::CreateInterfaceFn;
use vmthook;
use std::ffi::CStr;

pub unsafe fn install_client() {
    let mut hooker = vmthook::VMTHooker::new(INTERFACES.client as *mut _);
    REAL_INIT = hooker.get_orig_method(0);
    hooker.hook(0, mem::transmute::<_, *const ()>(hooked_init));
    REAL_CREATEMOVE = hooker.get_orig_method(21);
    hooker.hook(21, mem::transmute::<_, *const ()>(hooked_createmove));

    INTERFACES.input = locate_cinput(REAL_CREATEMOVE).unwrap();

    let mut hooker = vmthook::VMTHooker::new(INTERFACES.input as *mut _);
    hooker.hook(8, mem::transmute::<_, *const ()>(hooked_getusercmd));

    /*
    let mut hooker = vmthook::VMTHooker::new(INTERFACES.panel as *mut _);
    REAL_PAINTTRAVERSE = hooker.get_orig_method(41);
    sdk::REAL_PAINTTRAVERSE = REAL_PAINTTRAVERSE;
    sdk::IPANEL = INTERFACES.panel as *const ();
    hooker.hook(41, mem::transmute::<_, *const ()>(hooked_painttraverse));
    */
}

pub static mut REAL_CREATEMOVE: *const () = 0 as *const ();
pub static mut REAL_INIT: *const () = 0 as *const ();
pub static mut REAL_PAINTTRAVERSE: *const () = 0 as *const ();

type CreateMoveFn = unsafe extern "stdcall" fn(libc::c_int,
                                               libc::c_float,
                                               bool);

type PaintTraverseFn = unsafe extern "stdcall" fn(libc::c_int,
                                               u8,
                                               u8);

unsafe extern "stdcall" fn hooked_getusercmd(sequence_number: libc::c_int) -> *mut sdk::CUserCmd {
    let cmds = *((INTERFACES.input as usize + 0xC4) as *const *mut sdk::CUserCmd);
    cmds.offset((sequence_number % 90) as isize)
}


unsafe extern "stdcall" fn hooked_init(app_sys_factory: CreateInterfaceFn,
                                             physics_factory: CreateInterfaceFn,
                                             globals: *mut sdk::CGlobalVarsBase) -> libc::c_int
{
    INTERFACES.globals = globals;
    mem::transmute::<_, unsafe extern "stdcall" fn(CreateInterfaceFn,
                                                   CreateInterfaceFn,
                                                   *mut sdk::CGlobalVarsBase) -> libc::c_int
        >(REAL_INIT)(app_sys_factory,
                     physics_factory,
                    globals)
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

    let me_idx = sdk::EngineClient_GetLocalPlayer(INTERFACES.engine);
    let me = sdk::CEntList_GetClientEntity(INTERFACES.entlist, me_idx);

    let wep = ::gameutils::get_active_weapon(me);
    if !wep.is_null() {
        let class = sdk::CBaseEntity_GetClientClass(wep);
        let classname = CStr::from_ptr((*class).name); 
        if classname.to_bytes() == b"CTFMinigun" {
            *ptr_offset::<_, libc::c_int>(wep, OFFSETS.m_iState) = 0;
        }
    }

    mem::transmute::<_, CreateMoveFn>(REAL_CREATEMOVE)(sequence_number,
                    input_sample_frametime,
                    active);

    // curtime is off in createmove, patch it up for now
    let old_curtime = (*INTERFACES.globals).curtime;
    //(*INTERFACES.globals).curtime = (*INTERFACES.globals).interval_per_tick * (*ptr_offset::<_, libc::c_uint>(me, OFFSETS.m_nTickBase) as f32);

    let sendpacket_ptr = ptr_offset::<_, bool>(*ebp, -1);
    let cmds = *((INTERFACES.input as usize + 0xC4) as *const *mut sdk::CUserCmd);
    let cmd_ptr = cmds.offset((sequence_number % 90) as isize);
    let mut cmd = *cmd_ptr;
    let orig_angles = cmd.viewangles;

    let myteam = *ptr_offset::<_, libc::c_int>(me, OFFSETS.m_iTeamNum);
    let meorigin = sdk::CBaseEntity_GetAbsOrigin(me);

    for ent in (1..32) 
        .filter(|&idx| idx != me_idx)
            .map(|idx| sdk::CEntList_GetClientEntity(INTERFACES.entlist, idx))
            .filter(|ent| !ent.is_null())
            {
                let friendly = *ptr_offset::<_, libc::c_int>(ent, OFFSETS.m_iTeamNum) == myteam;
                let alive = *ptr_offset::<_, u8>(ent, OFFSETS.m_lifeState) == 0;
                let dormant = sdk::CBaseEntity_IsDormant(ent); 
                if dormant || !alive { continue }

                *ptr_offset::<_, bool>(ent, 0x0DB4) = !friendly;
                sdk::CBaseEntity_UpdateGlowEffect(ent);
                
            }
    let flags = *ptr_offset::<_, i32>(me, OFFSETS.m_fFlags);
    if flags & 1 == 0 { 
        cmd.buttons &= !(2);
    }

    static mut FLIP: bool = false;
    FLIP = !FLIP;
    
    static mut ANG_ACCUM: f32 = 0.0;
    if false && cmd.buttons & 1 == 0 {
        use std::f32::consts::PI;
        ANG_ACCUM = (ANG_ACCUM + (1.0 / 7.0 * PI)) % (2.0 * PI); 
        let newang = ANG_ACCUM;

        cmd.viewangles.yaw = newang.to_degrees(); 
        cmd.viewangles.pitch = if FLIP {
            89.0
        } else {
            -89.0
        };
    }

    /*if let Some(t) = ::airblast::Targets::new().next() {
        ::aimbot::aim(t, &mut cmd);
        cmd.buttons |= 1<<11;
    }*/

    let meorigin = sdk::CBaseEntity_GetAbsOrigin(me).clone();
    let eyes = meorigin + *ptr_offset::<_, Vector>(me, OFFSETS.m_vecViewOffset);
    let viewray = cmd.viewangles.to_vector(); 


    if let Some(t) = ::aimbot::targets().fold(
        None,
        |acc, target| {
            match acc {
                Some(ref best) if (target.pos - eyes).normalize().dot(&viewray) > (best.pos - eyes).normalize().dot(&viewray) => Some(target),
                Some(best) => Some(best),
                None => Some(target),
            }
        }) {
        use std::num::Float;
        if true || (t.pos - eyes).dot(&viewray) > 30.0.to_radians().cos() { 
            ::aimbot::aim(t, &mut cmd);
        }
    } else {
        cmd.buttons &= !1;
    }

    if cmd.viewangles.pitch > 90.0 {
        cmd.viewangles.pitch -= 360.0;
    }
    if cmd.viewangles.pitch > 90.0 {
        cmd.viewangles.pitch = 90.0;
    }
    if cmd.viewangles.pitch < -90.0 {
        cmd.viewangles.pitch = -90.0;
    }
    if cmd.viewangles.yaw < -180.0 {
        cmd.viewangles.yaw += 360.0;
    }
    if cmd.viewangles.yaw > 180.0 {
        cmd.viewangles.yaw -= 360.0; 
    }

    let (fwd, right, up) = (cmd.forwardmove, cmd.sidemove, cmd.upmove);

    let new_angles = sdk::QAngle { pitch: 0.0, ..cmd.viewangles };
    let orig_angles = sdk::QAngle { pitch: 0.0, ..orig_angles };
	let (orig_fwd, orig_right, orig_up) = orig_angles.to_vectors();
	let (new_fwd, new_right, new_up) = new_angles.to_vectors();
	
    cmd.forwardmove = fwd * new_fwd.dot(&orig_fwd) + right * new_fwd.dot(&orig_right) + up * new_fwd.dot(&orig_up);
    cmd.sidemove = fwd * new_right.dot(&orig_fwd) + right * new_right.dot(&orig_right) + up * new_right.dot(&orig_up);
    cmd.upmove = fwd * new_up.dot(&orig_fwd) + right * new_up.dot(&orig_right) + up * new_up.dot(&orig_up);

    if false && flags & (1<<1) != 0{

        cmd.viewangles.roll = 270.0;
        cmd.viewangles.pitch = 89.0;

        let ay = (-cmd.sidemove).atan2(cmd.forwardmove).to_degrees();
        cmd.viewangles.yaw = (((cmd.viewangles.yaw + ay) % 360.0 )- 180.0);
        cmd.sidemove = -8.0 * (cmd.forwardmove.abs() + cmd.sidemove.abs());
        cmd.forwardmove = 0.0;
    }

    cmd.command_number = 2076615043;
    cmd.random_seed = 39;

    *cmd_ptr = cmd;
    let verified_cmds = *((INTERFACES.input as usize + 0xC8) as *const *mut sdk::CVerifiedUserCmd);
    let verified_cmd = verified_cmds.offset((sequence_number % 90) as isize);
    (*verified_cmd).m_cmd = cmd;
    verify_usercmd(verified_cmd);

    (*INTERFACES.globals).curtime = old_curtime;
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
    let result = ::utils::search_memory(createmove, 100, &[0x8B, 0x0D], &[true, true]);
    match result {
        Some(ptr) => {
            let load_instruction_operand = ((ptr as usize) + 2) as *const *const *mut sdk::CInput;
            let cinput_ptr_ptr = *load_instruction_operand;
            Some((*cinput_ptr_ptr))
        },
        None => None 
    }
}
unsafe extern "stdcall" fn hooked_painttraverse(panel: u32,
                                      unk1: bool,
                                      unk2: bool) {
    sdk::IPanel_PaintTraverse(panel, unk1 , unk2 );
//    ::gui::GUI_MANAGER.draw_text(200, 200, &::gui::Color { r: 2, g: 255, b: 2, a: 255, }, "penis");
}
