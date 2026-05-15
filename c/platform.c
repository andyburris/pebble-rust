#include <pebble.h>

int16_t _pbl_display_width(void)  { return PBL_DISPLAY_WIDTH; }
int16_t _pbl_display_height(void) { return PBL_DISPLAY_HEIGHT; }
uint8_t _pbl_is_rect(void)        { return PBL_IF_RECT_ELSE(1, 0); }
uint8_t _pbl_is_round(void)       { return PBL_IF_ROUND_ELSE(1, 0); }
uint8_t _pbl_is_color(void)       { return PBL_IF_COLOR_ELSE(1, 0); }
