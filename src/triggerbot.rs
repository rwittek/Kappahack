use {INTERFACES, OFFSETS};
use libc;
use offsets::ptr_offset;

use sdk::{self, Ray_t, QAngle, trace_t, Vector};
use std::mem;

const TRIGGER_MASK: libc::c_uint = 0x4200400B; 

pub unsafe fn should_trigger(me: *mut sdk::CBaseEntity, eyes: Vector, angles: QAngle) -> bool {
    let ray = Ray_t::new(eyes, eyes + (angles.to_vector().scale(8192.0)));
    let mut tr = mem::zeroed::<trace_t>();
    let myteam = *ptr_offset::<_, libc::c_int>(me, OFFSETS.m_iTeamNum);

    sdk::CTraceFilterSkipEntity_SetHandle(sdk::GLOBAL_TRACEFILTER_PTR, *sdk::CBaseEntity_GetRefEHandle(me));
 
    sdk::CEngineTrace_TraceRay(INTERFACES.trace,
                               &ray,
                               TRIGGER_MASK,
                               sdk::GLOBAL_TRACEFILTER_PTR,
                               &mut tr);

    if tr.ent.is_null() {
        false
    } else {
        let index = sdk::CBaseEntity_GetIndex(tr.ent);
        if index < 1 || index > 32 { return false; }

        let friendly = *ptr_offset::<_, libc::c_int>(tr.ent, OFFSETS.m_iTeamNum) == myteam;
       !friendly
    }
}
