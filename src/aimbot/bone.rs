use sdk::{
    self,
    Vector,
    matrix3x4_t
};
use std;
use INTERFACES;
use libc;

pub unsafe fn get_all_bone_matrices(entity: *mut sdk::CBaseEntity) -> [matrix3x4_t; 128] {
    let mut bonetoworld = [std::mem::zeroed::<matrix3x4_t>(); 128];
    let _ok = sdk::CBaseEntity_SetupBones(entity, bonetoworld.as_mut_ptr(), 128, 0x100,
                                (*INTERFACES.globals).curtime);
    bonetoworld
}

#[allow(dead_code)]
pub unsafe fn get_all_bone_positions(entity: *mut sdk::CBaseEntity) -> [Vector; 128] {
    let bonetoworld = get_all_bone_matrices(entity);
    let mut positions = [std::mem::zeroed::<Vector>(); 128];
    for (bone, position) in bonetoworld.iter().zip(positions.iter_mut()) {
        *position = bone.transform_point(&Vector::zero());
    }
    positions
}
