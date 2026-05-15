unsafe extern "C" {
    unsafe fn _pbl_display_width()  -> i16;
    unsafe fn _pbl_display_height() -> i16;
    unsafe fn _pbl_is_rect()        -> u8;
    unsafe fn _pbl_is_round()       -> u8;
    unsafe fn _pbl_is_color()       -> u8;
}

pub fn display_width()  -> i16  { unsafe { _pbl_display_width() } }
pub fn display_height() -> i16  { unsafe { _pbl_display_height() } }
pub fn is_rect()        -> bool { unsafe { _pbl_is_rect() != 0 } }
pub fn is_round()       -> bool { unsafe { _pbl_is_round() != 0 } }
pub fn is_color()       -> bool { unsafe { _pbl_is_color() != 0 } }
