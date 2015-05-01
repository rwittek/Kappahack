use sdk::CBaseEntity;
use sdk;
use INTERFACES;
use OFFSETS;
use offsets::ptr_offset;
use libc;
pub fn get_active_weapon(player: *mut CBaseEntity) -> *mut CBaseEntity {
    unsafe {
        let handle = *ptr_offset::<_, libc::c_int>(player, OFFSETS.m_hActiveWeapon);
        sdk::CEntList_GetClientEntityFromHandle(INTERFACES.entlist, handle)
    }
}
