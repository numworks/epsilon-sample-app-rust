.cpu cortex-m7
.fpu fpv5-sp-d16
/*
FIXME: What does this do??
.eabi_attribute 27, 1
.eabi_attribute 28, 1
.eabi_attribute 20, 1
.eabi_attribute 21, 1
.eabi_attribute 23, 3
.eabi_attribute 24, 1
.eabi_attribute 25, 1
.eabi_attribute 26, 1
.eabi_attribute 30, 4
.eabi_attribute 34, 1
.eabi_attribute 18, 4
*/

.thumb
.thumb_func
.syntax unified
.global eadk_backlight_brightness
eadk_backlight_brightness:
   push {r4, lr}
   svc 1
   mov r4, r0
   uxtb r0, r4
   pop {r4, pc}

.global eadk_backlight_set_brightness
eadk_backlight_set_brightness:
   svc 2
   bx lr

.global eadk_battery_is_charging
eadk_battery_is_charging:
   push {r4, lr}
   svc 3
   mov r4, r0
   uxtb r0, r4
   pop {r4, pc}

.global eadk_battery_level
eadk_battery_level:
   push {r4, lr}
   svc 4
   mov r4, r0
   uxtb r0, r4
   pop {r4, pc}

.global eadk_battery_voltage
eadk_battery_voltage:
  svc 5
  vmov ip, s0
  vmov s0, ip
  bx lr

.global eadk_display_pull_rect
eadk_display_pull_rect:
   sub sp, #8
   add r3, sp, #8
   stmdb r3, {r0, r1}
   svc 16
   add sp, #8
   bx lr

.global eadk_display_push_rect
eadk_display_push_rect:
  sub sp, #8
  add r3, sp, #8
  stmdb r3, {r0, r1}
  svc 17
  add sp, #8
  bx lr

.global eadk_display_push_rect_uniform
eadk_display_push_rect_uniform:
   sub sp, #8
   add r3, sp, #8
   stmdb r3, {r0, r1}
   svc 18
   add sp, #8
   bx lr

.global eadk_display_wait_for_vblank
eadk_display_wait_for_vblank:
   push {r4, lr}
   svc 19
   mov r4, r0
   uxtb r0, r4
   pop {r4, pc}

.global eadk_keyboard_pop_state
eadk_keyboard_pop_state:
  movs r2, #0
  movs r3, #0
  push {r0, r1, r4, lr}
  mov r4, sp
  strd r2, r3, [sp]
  mov r0, r4
  svc 31
  ldrd r0, r1, [r4]
  add sp, #8
  pop {r4, pc}

.global eadk_keyboard_scan
eadk_keyboard_scan:
  movs r2, #0
  movs r3, #0
  push {r0, r1, r4, lr}
  mov r4, sp
  strd r2, r3, [sp]
  mov r0, r4
  svc 32
  ldrd r0, r1, [r4]
  add sp, #8
  pop {r4, pc}

.global eadk_timing_millis
eadk_timing_millis:
  push {r0, r1, r4, lr}
  mov r4, sp
  svc 46
  str r0, [r4, #0]
  str r1, [r4, #4]
  movs r0, #0
  movs r1, #0
  add sp, #8
  pop {r4, pc}

.global eadk_timing_msleep
eadk_timing_msleep:
   svc 47
   bx lr

.global eadk_timing_usleep
eadk_timing_usleep:
  svc 48
  bx lr

.global eadk_random
eadk_random:
   push {r4, lr}
   svc 43
   mov r4, r0
   mov r0, r4
   pop {r4, pc}

.global eadk_usb_is_plugged
eadk_usb_is_plugged:
push {r4, lr}
   svc 50
   mov r4, r0
   uxtb r0, r4
   pop {r4, pc}
