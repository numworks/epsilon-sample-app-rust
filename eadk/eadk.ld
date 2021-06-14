MEMORY {
  FLASH (rx) : ORIGIN = 0, LENGTH = 64K
}

SECTIONS {
  .app_info ORIGIN(FLASH) : {
    _app_start = .;
    LONG(0xDEC0BEBA);
    LONG(0); // API level 0
    LONG(app_name); // There is an "app_name" symbol that points to a C-string
    LONG(_app_icon_end - _app_icon_start);
    LONG(_app_icon_start);
    LONG(main & 0xFFFFFFFE); // This address shall be odd to indicate a Thumb jump
// FIXME: Remove & 0xFFFFFFFE
    LONG(_app_end - _app_start);
    LONG(0xDEC0BEBA);
  } >FLASH

  .text : {
    . = ALIGN(4);
    *(.text)
    *(.text.*)
  } >FLASH

  .rodata : {
    *(.rodata)
    *(.rodata.*)
    INCLUDE "icon.ld";
    _app_end = .;
  } >FLASH


  /DISCARD/ : {
    *(.bss)
    *(.bss.*)
    *(.data)
    *(.data.*)
    *(.ARM.exidx .ARM.exidx.*);
  }

}

