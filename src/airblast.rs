use INTERFACES;
use OFFSETS;
use offsets::ptr_offset;
use libc;
use sdk::{self, Vector};
use std::mem;
use aimbot::Target;

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
        while self.current_entnum <= self.highest_entnum {
            let targ = unsafe {
                get_target(self.current_entnum)
            };

            if targ.is_some() {
                return targ;
            }

            self.current_entnum += 1;
        }
        None
    }
}

unsafe fn get_target(entnum: libc::c_int) -> Option<Target> {
    use std::ffi::CStr;
    let ent = sdk::CEntList_GetClientEntity(INTERFACES.entlist, entnum);
    if ent.is_null() {
        return None;
    }
    let dormant = sdk::CBaseEntity_IsDormant(ent); 
    if dormant { return None; }
    let class = sdk::CBaseEntity_GetClientClass(ent);
    let classname = CStr::from_ptr((*class).name); 
    let reflectable = match classname.to_bytes() { 
        b"CTFProjectile_Rocket" | b"CTFProjectile_Flare"
				| b"CTFProjectile_EnergyBall" | b"CTFProjectile_HealingBolt" 
				| b"CTFProjectile_Arrow" | b"CTFProjectile_SentryRocket" 
				| b"CTFProjectile_Throwable" | b"CTFThrowable" 
				| b"CTFProjectile_Cleaver"  | b"CTFProjectile_JarMilk" 
				| b"CTFProjectile_Jar" | b"CTFStunBall" 
				| b"CTFGrenadePipebombProjectile" | b"CTFBall_Ornament" => true,
                _ => false
    };
    if !reflectable {
        return None;
    }

    let mut origin = Vector { x: 0., y: 0., z: 0. };
    sdk::CBaseEntity_GetWorldSpaceCenter(ent, &mut origin);

    let me_idx = sdk::EngineClient_GetLocalPlayer(INTERFACES.engine);
    let me = sdk::CEntList_GetClientEntity(INTERFACES.entlist, me_idx);

    let myteam = *ptr_offset::<_, libc::c_int>(me, OFFSETS.m_iTeamNum);
    let friendly = *ptr_offset::<_, libc::c_int>(ent, OFFSETS.m_iTeamNum) == myteam;
    if friendly {
        return None;
    }

    let meorigin = sdk::CBaseEntity_GetAbsOrigin(me).clone();
    let eyes = meorigin + *ptr_offset::<_, Vector>(me, OFFSETS.m_vecViewOffset);
    let netchan = sdk::EngineClient_GetNetChannelInfo(INTERFACES.engine);
    let latency = sdk::INetChannelInfo_GetLatency(netchan, 0) + sdk::INetChannelInfo_GetLatency(netchan, 1);
    let mut vel = Vector { x: 0.0, y: 0.0, z: 0.0 };
    sdk::CBaseEntity_EstimateAbsVelocity(ent, &mut vel);
    let targpos = origin + (vel.scale(latency));

    let should_reflect = (targpos - eyes).length() <= 185.0;
    if should_reflect {
        Some(Target { pos: targpos })
    } else {
        None
    }
}
