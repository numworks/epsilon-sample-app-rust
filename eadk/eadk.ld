MEMORY {
  /* The maximal size of the external app depends on how many apps you already
   * stored. */
  FLASH (rx) : ORIGIN = 0, LENGTH = 64K
}

SECTIONS {
  .app_info ORIGIN(FLASH) : {
    LONG(0xDEC0BEBA);
    LONG(0); /* API level 0 */
    LONG(eadk_app_name); /* There is an "app_name" symbol that points to a C-string */
    LONG(_eadk_app_icon_end - _eadk_app_icon_start);
    LONG(_eadk_app_icon_start);
    LONG(eadk_main);
    LONG(_eadk_app_end);
    LONG(0xDEC0BEBA);
  } >FLASH

  .text : {
    . = ALIGN(4);
    KEEP(*(*.eadk_main));
    *(.text)
    *(.text.*)
  } >FLASH

  .rodata : {
    KEEP(*(*.eadk_app_name));
    *(.rodata)
    *(.rodata.*)
    INCLUDE "target/icon.ld";
    _eadk_app_end = .;
  } >FLASH


  /DISCARD/ : {
    *(.bss)
    *(.bss.*)
    *(.data)
    *(.data.*)
    *(.ARM.exidx .ARM.exidx.*);
  }

}

