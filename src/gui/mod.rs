#![allow(dead_code)]
use libc;
use INTERFACES;
use sdk;

lazy_static! {
    pub static ref GUI_MANAGER: GuiManager = GuiManager::new();
}

#[repr(packed)]
pub struct Color { pub r: u8, pub g: u8, pub b: u8, pub a: u8 }

pub struct GuiManager {
    font: libc::c_uint, // fucking seriously?
}
impl GuiManager {
    fn new() -> GuiManager {
        unsafe { 
            assert!(!INTERFACES.surface.is_null());
            let font = sdk::ISurface_CreateFont(INTERFACES.surface);
            let fontstr = ::std::ffi::CString::new("Comic Sans").unwrap();
            sdk::ISurface_SetFontGlyphSet(INTERFACES.surface, &font, fontstr.as_ptr(), 12, 600, 0, 0, 0x200);

            GuiManager {
                font: font
            }
        }
    }

    pub fn draw_text(&self, x: libc::c_int, y: libc::c_int, color: &Color, text: &str) {
        unsafe {
            use libc::c_int;
            sdk::ISurface_DrawSetTextPos(INTERFACES.surface, x, y);
            sdk::ISurface_DrawSetTextFont(INTERFACES.surface, self.font);
            sdk::ISurface_DrawSetTextColor(INTERFACES.surface, color.r as c_int,
                                           color.g as c_int, color.b as c_int,
                                           color.a as c_int);

            let mut textbuf = [0u16; 512];
            let mut length = 0;
            for (dest, src) in textbuf.iter_mut().zip(text.utf16_units()) {
                length += 1;
                *dest = src;
            }
            sdk::ISurface_DrawPrintText(INTERFACES.surface, textbuf.as_ptr(), length as libc::c_int);
        }
    }
}

