#ifndef EADK_H
#define EADK_H

#include <stdint.h>

// Backlight
uint8_t eadk_backlight_brightness();
void eadk_backlight_set_brightness(uint8_t brightness);

// Battery
bool eadk_battery_is_charging();
uint8_t eadk_battery_level();
float eadk_battery_voltage();

// Display
typedef uint16_t eadk_color;
typedef struct {
  uint16_t x;
  uint16_t y;
  uint16_t width;
  uint16_t height;
} eadk_rect;
void eadk_display_push_rect(eadk_rect rect, const eadk_color * pixels);
void eadk_display_push_rect_uniform(eadk_rect rect, eadk_color color);
void eadk_display_pull_rect(eadk_rect rect, eadk_color * pixels);
bool eadk_display_wait_for_vblank();

// Keyboard
typedef enum {
  LEFT=0,  UP=1,     DOWN=2,    RIGHT=3,            OK=4,              BACK=5,
  HOME=6,  /*B2=7,*/ ONOFF=8,   /*B4=9,             B5=10,             B6=11, */
  SHIFT=12,ALPHA=13, XNT=14,    BAR=15,             TOOLBOX=16,        BACKSPACE=17,
  EXP=18,  LN=19,    LOG=20,    IMAGINARY=21,       COMMA=22,          POWER=23,
  SINE=24, COSINE=25,TANGENT=26,PI=27,              SQRT=28,           SQUARE=29,
  SEVEN=30,EIGHT=31, NINE=32,   LEFTPARENTHESIS=33, RIGHTPARENTHESIS=34,// F6=35,
  FOUR=36, FIVE=37,  SIX=38,    MULTIPLICATION=39,  DIVISION=40,        // G6=41,
  ONE=42,  TWO=43,   THREE=44,  PLUS=45,            MINUS=46,           // H6=47,
  ZERO=48, DOT=49,   EE=50,     ANS=51,             EXE=52,             // I6=53,
  NONE = 54
} eadk_key;

typedef uint64_t eadk_keyboard_state;
eadk_keyboard_state eadk_keyboard_scan();
eadk_keyboard_state eadk_keyboard_pop_state();

// Timing
void eadk_timing_usleep(uint32_t us);
void eadk_timing_msleep(uint32_t ms);
uint64_t eadk_timing_millis();

// USB
bool eadk_usb_is_plugged();

// Misc
uint32_t eadk_random();

#endif
