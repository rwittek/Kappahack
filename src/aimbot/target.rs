use INTERFACES;
use libc;
use sdk;
use sdk::Vector;

pub struct Target {
    pub pos: Vector
}

#[allow(dead_code)]
pub struct Targets {
    current_entnum: libc::c_int,
}
impl Targets {
    #[allow(dead_code)]
    pub fn new() -> Targets {
        Targets {
            current_entnum: 1
        }
    }

}

impl Iterator for Targets {
    type Item = Target;
    fn next(&mut self) -> Option<Target> {
        self.current_entnum += 1;
        while self.current_entnum < 65 {
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
    let ent = sdk::CEntList_GetClientEntity(INTERFACES.entlist, entnum);
    if ent.is_null() {
        return None;
    }

    let origin = sdk::CBaseEntity_GetAbsOrigin(ent);

    Some(Target {
        pos: *origin 
    })
}
