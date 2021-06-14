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
  svc #4
  bx lr

.global eadk_backlight_set_brightness
eadk_backlight_set_brightness:
  push {r4, lr}
  svc #1
  mov r4, r0
  uxtb r0, r4
  pop {r4, pc}

.global eadk_display_push_rect_uniform
eadk_display_push_rect_uniform:
  sub sp, sp, #8
  add r3, sp, #8
  stmdb r3, {r0, r1}
  svc #22
  add sp, sp, #8
  bx lr
