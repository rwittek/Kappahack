use libc;
use std::ops::{
    Add,
        Sub
};

pub enum EngineClient {}

pub enum CHLClient {}

pub enum CEngineTrace {}

pub enum CEntList {}

pub enum ITraceFilter {}

pub enum CBaseEntity {}

pub enum DebugOverlay {}

pub enum CInput {}

#[allow(dead_code)]
extern "C" {
    pub static GLOBAL_TRACEFILTER_PTR: *mut ITraceFilter;

    pub fn CBaseEntity_IsDormant(_this: *mut CBaseEntity) -> bool;
    pub fn CBaseEntity_GetAbsOrigin(_this: *mut CBaseEntity) -> &Vector;
    pub fn CBaseEntity_GetAbsAngles(_this: *mut CBaseEntity) -> &QAngle;
    pub fn CBaseEntity_GetRenderBounds(_this: *mut CBaseEntity, mins: &mut Vector, maxes: &mut Vector);

    pub fn CBaseEntity_GetIndex(_this: *mut CBaseEntity) -> libc::c_int;
    pub fn CBaseEntity_GetRefEHandle(_this: *mut CBaseEntity) -> &libc::c_int;

    pub fn CEntList_GetClientEntity(_this: *mut CEntList, entnum: libc::c_int) -> *mut CBaseEntity;

    pub fn CEngineTrace_TraceRay(_this: *mut CEngineTrace,
                                ray: &Ray_t,
                                fMask: libc::c_uint,
                                pTraceFilter: *mut ITraceFilter,
                                pTrace: *mut trace_t) -> bool;

    
    pub fn CTraceFilterSkipEntity_SetHandle(_this: *mut ITraceFilter, handle: libc::c_int);

    pub fn DebugOverlay_AddLineOverlay(_this: *mut DebugOverlay,
                                      origin: &Vector,
                                      dest: &Vector,
                                      r: libc::c_int,
                                      g: libc::c_int,
                                      b: libc::c_int,
                                      noDepthTest: bool,
                                      duration: libc::c_float
                                      );

    pub fn EngineClient_GetLocalPlayer(_this: *mut EngineClient) -> libc::c_int;
    pub fn EngineClient_GetViewAngles(_this: *mut EngineClient, va: &mut QAngle);
    pub fn EngineClient_SetViewAngles(_this: *mut EngineClient, va: &QAngle);
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct QAngle {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32
}
impl QAngle {
    pub fn to_vectors(self) -> (Vector, Vector, Vector) {
        use std::num::{Float};

        let (sy, cy) = self.yaw.to_radians().sin_cos(); 

        let (sp, cp) = self.pitch.to_radians().sin_cos(); 

        let (sr, cr) = self.roll.to_radians().sin_cos(); 

        (
            Vector {
                x: cp*cy,
                y: cp*sy,
                z: -sp
            },
            Vector {
                x: -1.0*sr*sp*cy+-1.0*cr*-sy,
                y: -1.0*sr*sp*sy+-1.0*cr*cy,
                z: -1.0*sr*cp
            },
            Vector {
                x: cr*sp*cy+-sr*-sy,
                y: cr*sp*sy+-sr*cy,
                z: cr*cp
            }
        )
    }
    pub fn to_vector(self) -> Vector {
        let (forward, _right, _up) = self.to_vectors();
        forward
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector {
    pub fn zero() -> Vector {
        Vector { x: 0., y: 0., z: 0. }
    }
    pub fn to_aligned(self) -> VectorAligned {
        VectorAligned {
            x: self.x,
            y: self.y,
            z: self.z,
            _pad: [0xDE,0xAD,0xBE,0xEF]
        }
    }
    pub fn length_sqr(&self) -> f32 {
        (self.x * self.x)
            + (self.y * self.y)
            + (self.z * self.z)
    }
    pub fn scale(self, s: f32) -> Vector {
        Vector {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s
        }
    }
    pub fn to_angle(self) -> QAngle {
        use std::num::{Float};
        if self.x == 0.0 && self.y == 0.0 {
            QAngle { pitch: 0.0, yaw: 0.0, roll: 0.0 }
        }
        else
        {
		    let mut yaw = self.y.atan2(self.x).to_degrees();

            if yaw > 180.0 {
                yaw -= 360.0;
            }

		    let tmp = self.x * self.x + self.y * self.y;

	        let pitch = self.z.atan2(tmp).to_degrees();
            QAngle { pitch: pitch, yaw: yaw, roll: 0.0 }
        }
	}

}
impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}
    
impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VectorAligned {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    _pad: [u8; 4]
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct Ray_t {
    m_Start: VectorAligned,
    m_Delta: VectorAligned,
    m_StartOffset: VectorAligned,
    m_Extents: VectorAligned,
    m_IsRay: bool,
    m_IsSwept: bool
}
impl Ray_t {
    pub fn new(start: Vector, end: Vector) -> Ray_t {
        let delta = end - start;
        let is_swept = delta.length_sqr() > 0.0;

        Ray_t {
            m_Start: start.to_aligned(),
            m_Delta: delta.to_aligned(),
            m_StartOffset: Vector::zero().to_aligned(),
            m_Extents: Vector::zero().to_aligned(),
            m_IsRay: true,
            m_IsSwept: is_swept
        }
    }
}

#[repr(C)]
#[allow(dead_code, non_snake_case, non_camel_case_types)]
pub struct csurface_t {
	name: *const libc::c_char,
	surface_props: libc::c_short,
	flags: u16
}
#[repr(C)]
#[allow(dead_code, non_snake_case, non_camel_case_types)]
pub struct cplane_t {
	normal: Vector,
	float: libc::c_float,
	type_: u8,
	signbits: u8,
	pad: [u8; 2]
}
#[repr(C)]
#[allow(dead_code, non_snake_case, non_camel_case_types)]
pub struct trace_t {
    startpos: Vector,
    endpos: Vector,
	plane: cplane_t,

	fraction: libc::c_float,
	contents: i32,
	dispFlags: libc::c_ushort,

	pub allsolid: bool,
	pub startsolid: bool,

	fractionleftsolid: libc::c_float,
	surface: csurface_t,
	pub hitgroup: i32,
	pub physicsbone: libc::c_short,
    worldsurfaceindex: libc::c_ushort,
	pub ent: *mut CBaseEntity,
	pub hitbox: i32
}

pub struct CVerifiedUserCmd {
    pub m_cmd: CUserCmd,
    pub m_crc: libc::c_uint
}

#[derive(Clone, Copy)]
pub struct CUserCmd {
	vtable_ptr: *const i32,
	pub command_number: i32,
	pub tick_count: i32,
	
	pub viewangles: QAngle,  

	pub forwardmove: f32,
	pub sidemove: f32,
	pub upmove: f32,     
	pub buttons: i32,	
	// Impulse command issued.
	pub impulse: u8,   
	pub weaponselect: i32,	
	pub weaponsubtype: i32,

	pub random_seed: i32,

	pub mousedx: u16,
	pub mousedy: u16,

	pub hasbeenpredicted: bool
}
