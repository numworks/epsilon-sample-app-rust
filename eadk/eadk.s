.syntax unified
.cpu cortex-m7
.fpu fpv5-sp-d16
.thumb
.thumb_func

@@@ Backlight

@ u8 eadk_backlight_brightness()
.global eadk_backlight_brightness
eadk_backlight_brightness:
   push {r4, lr}
   svc 1
   mov r4, r0
   uxtb r0, r4
   pop {r4, pc}

@ void eadk_backlight_set_brightness(u8 brightness)
.global eadk_backlight_set_brightness
eadk_backlight_set_brightness:
   svc 2
   bx lr

@ u8 eadk_battery_is_charging()
.global eadk_battery_is_charging
eadk_battery_is_charging:
   push {r4, lr}
   svc 3
   mov r4, r0
   uxtb r0, r4
   pop {r4, pc}

@ u8 eadk_battery_level()
.global eadk_battery_level
eadk_battery_level:
   push {r4, lr}
   svc 4
   mov r4, r0
   uxtb r0, r4
   pop {r4, pc}

@ f32 eadk_battery_voltage()
.global eadk_battery_voltage
eadk_battery_voltage:
  svc 5
  vmov ip, s0
  vmov s0, ip
  bx lr

@@@ Display

@ point = u16 x, u16 y
@ rect = u16 x, u16 y, u16 width, u16 heigt

@ void eadk_display_push_rect(rect r, const u16 * pixels)
.global eadk_display_pull_rect
eadk_display_pull_rect:
   sub sp, #8
   add r3, sp, #8
   stmdb r3, {r0, r1}
   svc 18
   add sp, #8
   bx lr

@ void eadk_display_push_rect_uniform(rect r, u16 color)
.global eadk_display_push_rect
eadk_display_push_rect:
  sub sp, #8
  add r3, sp, #8
  stmdb r3, {r0, r1}
  svc 19
  add sp, #8
  bx lr

@ void eadk_display_pull_rect(rect r, u16 * pixels)
.global eadk_display_push_rect_uniform
eadk_display_push_rect_uniform:
   sub sp, #8
   add r3, sp, #8
   stmdb r3, {r0, r1}
   svc 20
   add sp, #8
   bx lr

@ u8 eadk_display_wait_for_vblank()
.global eadk_display_wait_for_vblank
eadk_display_wait_for_vblank:
   push {r4, lr}
   svc 21
   mov r4, r0
   uxtb r0, r4
   pop {r4, pc}

@ void eadk_display_draw_string(u32 text, point p, u8 large_font, u16 text_color, u16 background_color)
.global eadk_display_draw_string
eadk_display_draw_string:
   push {r0, r1, r4, r5}
   ldrh r5, [sp, #16]
   str r1, [sp, #4]
   mov r4, pc
   lsrs r4, r4, #22
   str r5, [sp, #16]
   lsls r4, r4, #22
   add r4, r4, #65536
   ldr r4, [r4, #40]
   mov ip, r4
   add sp, sp, #8
   pop {r4, r5}
   bx ip

@ Keyboard

@ Left  =  0
@ Up  =  1
@ Down = 2
@ Right = 3
@ Ok = 4
@ Back = 5
@ Home  =  6
@ Shift = 12
@ Alpha = 13
@ Xnt = 14
@ Bar = 15
@ Toolbox = 16
@ Backspace = 17
@ Exp = 18
@ Ln = 19
@ Log = 20
@ Imaginary = 21
@ Comma = 22
@ Power = 23
@ Sine = 24
@ Cosine = 25
@ Tangent = 26
@ Pi = 27
@ Sqrt = 28
@ Square = 29
@ Seven = 30
@ Eight = 31
@ Nine = 32
@ LeftParenthesis = 33
@ RightParenthesis = 34
@ Four = 36
@ Five = 37
@ Six = 38
@ Multiplication = 39
@ Division = 40
@ One = 42
@ Two = 43
@ Three = 44
@ Plus = 45
@ Minus = 46
@ Zero = 48
@ Dot = 49
@ EE = 50
@ Ans = 51
@ Exe = 52

@ u64 eadk_keyboard_state eadk_keyboard_scan()
.global eadk_keyboard_scan
eadk_keyboard_scan:
  push {r0, r1, r4, r5, lr}
  movs r2, #0
  movs r3, #0
  mov r4, r0
  mov r5, sp
  strd r2, r3, [sp]
  mov r0, r5
  svc 34
  ldrd r2, r3, [r5]
  mov r0, r4
  strd r2, r3, [r4]
  add sp, #8
  pop {r4, r5, pc}

@@@ Timing

@ u64 eadk_timing_millis()
.global eadk_timing_millis
eadk_timing_millis:
  movs r2, #0
  movs r3, #0
  push {r0, r1, r4, lr}
  mov r4, sp
  strd r2, r3, [sp]
  svc 48
  str r0, [r4]
  str r1, [r4, #4]
  ldrd r0, r1, [r4]
  add sp, #8
  pop {r4, pc}

@ void eadk_timing_msleep(u32 ms)
.global eadk_timing_msleep
eadk_timing_msleep:
   svc 49
   bx lr

@ void eadk_timing_usleep(u32 us)
.global eadk_timing_usleep
eadk_timing_usleep:
  svc 50
  bx lr

@@@ Misc

@ u32 eadk_random()
.global eadk_random
eadk_random:
   push {r4, lr}
   svc 45
   mov r4, r0
   mov r0, r4
   pop {r4, pc}
