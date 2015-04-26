use {INTERFACES, OFFSETS};
use offsets::ptr_offset;
use sdk;
use sdk::Vector;
pub use self::target::{Target, Targets};

mod target;

pub fn targets() -> Targets {
    Targets::new()
}

pub unsafe fn aim(targ: Target, cmd: &mut sdk::CUserCmd) {
    let me_idx = sdk::EngineClient_GetLocalPlayer(INTERFACES.engine); 
    let me = sdk::CEntList_GetClientEntity(INTERFACES.entlist, me_idx);
    let meorigin = sdk::CBaseEntity_GetAbsOrigin(me).clone();
    let eyes = meorigin + *ptr_offset::<_, Vector>(me, OFFSETS.m_vecViewOffset);

    let aimray = targ.pos - eyes;

    let aimangle = aimray.to_angle();

    cmd.viewangles = aimangle;
}
