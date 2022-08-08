ENTRY(_eadk_main)

SECTIONS {
  .text : {
    KEEP(*(.text._eadk_main))
    *(.text)
    *(.text.*)
  }

  .rodata.eadk_app_api_level : {
    KEEP(*(*.eadk_app_api_level));
  }

  .rodata.eadk_app_name : {
    KEEP(*(*.eadk_app_name));
  }

  .rodata.eadk_app_icon : {
    KEEP(*(*.eadk_app_icon));
  }

  .rodata : {
    *(.rodata)
    *(.rodata.*)
  }

  .init_array : {
    KEEP (*(.init_array*))
  }

  .data : {
    *(.data)
    *(.data.*)
  }

  .bss : {
    *(.bss)
    *(.bss.*)
  }

  /DISCARD/ : {
    *(.ARM.exidx .ARM.exidx.*);
  }
}
