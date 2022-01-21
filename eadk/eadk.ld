INCLUDE "target/device_information.ld";

MEMORY {
  /* The maximal size of the external app depends on how many apps you already
   * stored. */
  FLASH (rx) : ORIGIN = EXTERNAL_APPS_FLASH_START, LENGTH = EXTERNAL_APPS_RAM_END - EXTERNAL_APPS_RAM_START
  RAM (rwx) : ORIGIN = EXTERNAL_APPS_RAM_START, LENGTH = EXTERNAL_APPS_RAM_END - EXTERNAL_APPS_RAM_START
}

SECTIONS {
  .app_info ORIGIN(FLASH) : {
    /* External apps header expects relative addresses. We subtract the
     * EXTERNAL_APPS_FLASH_START from all absolute addresses. */
    LONG(0xDEC0BEBA);
    LONG(0); /* API level 0 */
    LONG(eadk_app_name - EXTERNAL_APPS_FLASH_START); /* There is an "app_name" symbol that points to a C-string */
    LONG(_eadk_app_icon_end - _eadk_app_icon_start);
    LONG(_eadk_app_icon_start - EXTERNAL_APPS_FLASH_START);
    LONG(eadk_start - EXTERNAL_APPS_FLASH_START);
    LONG(_eadk_app_end - EXTERNAL_APPS_FLASH_START);
    LONG(0xDEC0BEBA);
  } >FLASH

  .text : {
    . = ALIGN(4);
    KEEP(*(*.eadk_start));
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

  .init_array : {
    . = ALIGN(4);
    _init_array_start = .;
    KEEP (*(.init_array*))
    _init_array_end = .;
  } >FLASH

  .data : {
    . = ALIGN(4);
    _data_section_start_flash = LOADADDR(.data);
    _data_section_start_ram = .;
    *(.data)
    *(.data.*)
    _data_section_end_ram = .;
  } >RAM AT >FLASH

  .bss : {
    . = ALIGN(4);
    _bss_section_start_ram = .;
    *(.bss)
    *(.bss.*)
    _bss_section_end_ram = .;
  } >RAM

  /*.heap : {
    _heap_start = .;
    . = (ORIGIN(RAM) + LENGTH(RAM));
    _heap_end = .;
  } >RAM*/

  /DISCARD/ : {
    *(.ARM.exidx .ARM.exidx.*);
  }

}

