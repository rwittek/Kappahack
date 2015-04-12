use {INTERFACES, OFFSETS};
use offsets::ptr_offset;
use sdk;
use sdk::Vector;
pub use self::target::{Target, Targets};

mod target;

#[allow(dead_code)]
fn targets() -> Targets {
    unimplemented!()
}

#[allow(dead_code)]
unsafe fn aim(targ: Target) {
    let me_idx = sdk::EngineClient_GetLocalPlayer(INTERFACES.engine); 
    let me = sdk::CEntList_GetClientEntity(INTERFACES.entlist, me_idx);
    let meorigin = sdk::CBaseEntity_GetAbsOrigin(me).clone();
    let eyes = meorigin + *ptr_offset::<_, Vector>(me, OFFSETS.m_vecViewOffset);

    let aimray = eyes - targ.pos;

    let aimangle = aimray.to_angle();

    sdk::EngineClient_SetViewAngles(INTERFACES.engine, &aimangle);
}
