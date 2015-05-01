#![allow(dead_code)]
use INTERFACES;
use OFFSETS;
use offsets::ptr_offset;
use sdk;
use sdk::Vector;

pub fn ideal_angle_delta(hspeed: f32,
                         maxspeed: f32,
                         airaccelerate: f32) -> Option<f32> {
    let term = (30.0 - (airaccelerate * maxspeed / 66.0)) / hspeed;
    if term < 1.0 && term > -1.0 {
        Some(term.acos())
    } else {
        None
    }
}


pub unsafe fn autostrafe(cmd: &mut sdk::CUserCmd) {
    let me_idx = sdk::EngineClient_GetLocalPlayer(INTERFACES.engine);
    let me = sdk::CEntList_GetClientEntity(INTERFACES.entlist, me_idx);

    let velocity = *ptr_offset::<_, Vector>(me, OFFSETS.m_vecVelocity); 
    let hvelocity = Vector { x: velocity.x, y: velocity.y, z: 0.0 };
    let hspeed = hvelocity.length();
    if hspeed.abs() < 0.1 {
        return;
    }

    let idealangdelta_rad = if let Some(delta) = ideal_angle_delta(hspeed, 300.0, 10.0) { delta } else { return; };
    let idealangdelta_deg = idealangdelta_rad.to_degrees() % 360.0;

    let fwd = cmd.viewangles.to_vector();
    let fwd = Vector { z: 0.0, ..fwd }.normalize();

    let slip = fwd.dot(&hvelocity);
    if slip.abs() < 30.0 {
        return;
    } else {
        let lookyaw_deg = cmd.viewangles.yaw % 360.0;

        let wishmoveyaw_local_deg = Vector { x: cmd.forwardmove, y: cmd.sidemove, z: 0.0 }.normalize().to_angle().yaw % 360.0; 
        let wishmoveyaw_deg = (lookyaw_deg + wishmoveyaw_local_deg) % 360.0;

        let velyaw_deg = hvelocity.normalize().to_angle().yaw % 360.0;

        let factor = if velyaw_deg - wishmoveyaw_deg > 0.0 {
            1.0
        } else {
            -1.0
        };
        let finalyaw_deg = (velyaw_deg + factor * idealangdelta_deg ) % 360.0;
        let finalyaw_rad = finalyaw_deg.to_radians();


        let fm = (cmd.forwardmove * cmd.forwardmove + cmd.sidemove * cmd.sidemove).sqrt();
        cmd.forwardmove = fm * finalyaw_rad.cos();
        cmd.sidemove = fm * finalyaw_rad.sin();
    }
}
