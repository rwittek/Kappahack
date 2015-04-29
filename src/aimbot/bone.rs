use sdk::{
    self,
    Vector,
    matrix3x4_t
};
use std;
use INTERFACES;
use libc;

pub unsafe fn get_bone_position(entity: *mut sdk::CBaseEntity, bone: libc::c_int) -> Vector {
    let mut bonetoworld = [std::mem::zeroed::<matrix3x4_t>(); 128];
    let ok = sdk::CBaseEntity_SetupBones(entity, bonetoworld.as_mut_ptr(), 128, 0x100,
                                (*INTERFACES.globals).curtime);
//    assert!(ok);
    bonetoworld[bone as usize].transform_point(&Vector::zero())
}
