use INTERFACES;
use OFFSETS;
use offsets::ptr_offset;
use libc;
use sdk::{self, Ray_t, trace_t, Vector};
use std::mem;

const TRIGGER_MASK: libc::c_uint = 0x200400B; 

pub struct Target {
    pub pos: Vector
}

#[allow(dead_code)]
pub struct Targets {
    current_entnum: libc::c_int,
    highest_entnum: libc::c_int,
}
impl Targets {
    pub fn new() -> Targets {
        Targets {
            current_entnum: 0,
            highest_entnum: unsafe {
                sdk::CEntList_GetHighestEntityIndex(INTERFACES.entlist) 
            }
        }
    }

}

impl Iterator for Targets {
    type Item = Target;
    fn next(&mut self) -> Option<Target> {
        while self.current_entnum < self.highest_entnum {
            let targ = unsafe {
                get_target(self.current_entnum)
            };

            self.current_entnum += 1;

            if targ.is_some() {
                return targ;
            }

        }
        None
    }
}

unsafe fn get_target(entnum: libc::c_int) -> Option<Target> {
    use std::ffi::{CStr, CString};
    let ent = sdk::CEntList_GetClientEntity(INTERFACES.entlist, entnum);
    if ent.is_null() {
        return None;
    }
    let dormant = sdk::CBaseEntity_IsDormant(ent); 
    if dormant { return None;
    }
    let class = sdk::CBaseEntity_GetClientClass(ent);
    let classname = CStr::from_ptr((*class).name); 
    let targettable = match classname.to_bytes() {
        b"CTFPlayer" | b"CObjectSentrygun" => true, 
        _ => false
    };
    if !targettable { 
        return None;
    }

    let me_idx = sdk::EngineClient_GetLocalPlayer(INTERFACES.engine);
    let me = sdk::CEntList_GetClientEntity(INTERFACES.entlist, me_idx);
    let myteam = *ptr_offset::<_, libc::c_int>(me, OFFSETS.m_iTeamNum);
    let friendly = *ptr_offset::<_, libc::c_int>(ent, OFFSETS.m_iTeamNum) == myteam;
    let alive = *ptr_offset::<_, i8>(ent, OFFSETS.m_lifeState) == 0;
    let targtime = (*INTERFACES.globals).interval_per_tick * ((*INTERFACES.globals).tickcount as f32); 
    sdk::CBaseEntity_Interpolate(ent, targtime); 
    let mut targpos = Vector { x: 0., y: 0., z: 0. };
    sdk::CBaseEntity_GetWorldSpaceCenter(ent, &mut targpos);

    if !friendly && alive {
        let meorigin = sdk::CBaseEntity_GetAbsOrigin(me).clone();
        let eyes = meorigin + *ptr_offset::<_, Vector>(me, OFFSETS.m_vecViewOffset);

        let ray = Ray_t::new(eyes, targpos);
        let mut tr = mem::zeroed::<trace_t>();
        sdk::CTraceFilterSkipEntity_SetHandle(sdk::GLOBAL_TRACEFILTER_PTR, *sdk::CBaseEntity_GetRefEHandle(me));

        sdk::CEngineTrace_TraceRay(INTERFACES.trace,
                                   &ray,
                                   TRIGGER_MASK,
                                   sdk::GLOBAL_TRACEFILTER_PTR,
                                   &mut tr);
        if tr.ent == ent || tr.fraction > 0.97 {
            Some(Target {
                pos: targpos 
            })
        } else {
            None
        }
    } else {
        None
    }  
}
