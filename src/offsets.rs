#[allow(non_snake_case)]
pub struct Offsets {
    pub m_vecViewOffset: usize,
    pub m_iTeamNum: usize,
    pub m_lifeState: usize
}
unsafe impl Sync for Offsets {}
unsafe impl Send for Offsets {} 


impl Offsets {
    pub fn load(&mut self) {
        *self = Offsets {
            m_vecViewOffset: 0xFC,
            m_iTeamNum: 0x0B0,
            m_lifeState: 0x0A5
        }
    }
}

pub fn ptr_offset<T, Res>(x: *mut T, offset: usize) -> *const Res {
    (((x as usize) + offset) as *const Res)
}

pub static mut OFFSETS: Offsets = Offsets {
    m_vecViewOffset: 0,
    m_iTeamNum: 0,
    m_lifeState: 0
};
