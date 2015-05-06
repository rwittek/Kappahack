use sdk;
use libc;
use offsets::ptr_offset;
use INTERFACES;
use OFFSETS;

pub unsafe fn predict_local_command(me: *mut sdk::CBaseEntity, cmd: &sdk::CUserCmd) {
    if !sdk::MOVEHELPER.is_null() {
        let mut tmpcmd = *cmd;
        tmpcmd.buttons &= !(
            1<<0 | 1<<11); 

        let oldflags = *ptr_offset::<_, libc::c_int>(me, OFFSETS.m_fFlags);

        sdk::IPrediction_RunCommand(INTERFACES.prediction,
                                    me,
                                    &tmpcmd as *const _,
                                    sdk::MOVEHELPER);

        *ptr_offset::<_, libc::c_int>(me, OFFSETS.m_fFlags) = oldflags;
    }
}
