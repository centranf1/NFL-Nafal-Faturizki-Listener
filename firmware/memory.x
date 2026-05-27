/* nRF5340 Application Core Memory Layout
 * 
 * nRF5340 DK:
 * - Flash: 1MB (0x00000000 - 0x000FFFFF)
 * - RAM:   512KB (0x20000000 - 0x2007FFFF)
 */

MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 512K
}

SECTIONS
{
  .text :
  {
    KEEP(*(.vector_table))
    *(.text .text.*)
    *(.rodata .rodata.*)
  } > FLASH

  .bss :
  {
    *(.bss .bss.*)
    *(COMMON)
  } > RAM AT > FLASH

  .data :
  {
    *(.data .data.*)
  } > RAM AT > FLASH
}
