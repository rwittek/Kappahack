use INTERFACES;
use OFFSETS;
use offsets::ptr_offset;
use libc;
use sdk::{self, Ray_t, trace_t, Vector};
use std::mem;

const TRIGGER_MASK: libc::c_uint = 0x4200400B; 

pub struct Target {
    pub pos: Vector
}

#[allow(dead_code)]
pub struct Targets {
    current_entnum: libc::c_int,
    highest_entnum: libc::c_int,
}
impl Targets {
    pub unsafe fn new() -> Targets {
        Targets {
            current_entnum: 0,
            highest_entnum: 
                sdk::CEntList_GetHighestEntityIndex(INTERFACES.entlist) 
        
        }
    }

}

impl Iterator for Targets {
    type Item = Target;
    fn next(&mut self) -> Option<Target> {
        while self.current_entnum < self.highest_entnum {
            let targ = unsafe {
                let kek = self.current_entnum;
                self.get_target(kek)
            };

            self.current_entnum += 1;

            if targ.is_some() {
                return targ;
            }

        }
        None
    }

}
impl Targets {
    unsafe fn get_target(&mut self, entnum: libc::c_int) -> Option<Target> {
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
        let (targettable, is_player) = match classname.to_bytes() {
            b"CTFPlayer" => (true, true),
            b"CObjectSentrygun" | b"CTFTankBoss" => (true, false),
            _ => (false, false) 
        };
        if !targettable { 
            return None;
        }

        let me_idx = sdk::EngineClient_GetLocalPlayer(INTERFACES.engine);
        let me = sdk::CEntList_GetClientEntity(INTERFACES.entlist, me_idx);
        let myteam = *ptr_offset::<_, libc::c_int>(me, OFFSETS.m_iTeamNum);
        let friendly = *ptr_offset::<_, libc::c_int>(ent, OFFSETS.m_iTeamNum) == myteam;
        let alive = *ptr_offset::<_, i8>(ent, OFFSETS.m_lifeState) == 0;
        let condok = if is_player {
            let cond = *ptr_offset::<_, libc::c_int>(ent, OFFSETS.m_nPlayerCond);
            let condex = *ptr_offset::<_, libc::c_int>(ent, OFFSETS.m_nPlayerCondEx);
            (cond & (1<<14 | 1<<5 | 1<<13) == 0) && (condex & (1<<19) == 0) 
        } else {
            true
        };

        if !friendly && alive && condok {
            //let targtime = (*INTERFACES.globals).curtime;
            //sdk::CBaseEntity_Interpolate(ent, targtime); 
            if is_player {
                let mut target = None;
                let bone_matrices = super::bone::get_all_bone_matrices(ent);
                for targpos in bone_matrices
                    .iter()
                    .filter(|mat| !mat.is_zero())
                    .map(|mat| mat.transform_point(&Vector::zero()))
                         { 
                    if self.is_visible(me, ent, targpos) {
                        target = Some(Target { pos: targpos });
                        break;
                    }
                }

                target
            } else {
                let mut targpos = Vector { x: 0., y: 0., z: 0. };
                sdk::CBaseEntity_GetWorldSpaceCenter(ent, &mut targpos);
                if self.is_visible(me, ent, targpos) {
                    Some(Target { pos: targpos })
                } else {
                    None
                }
            }
        } else {
            None
        }  
    }
    unsafe fn is_visible(&self, me: *mut sdk::CBaseEntity, ent: *mut sdk::CBaseEntity, targpos: sdk::Vector) -> bool {
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
            tr.ent == ent || tr.fraction > 0.97 
    }
}
