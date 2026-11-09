Link
====

build.rs generates link.ld in OUT_DIR at build time and passes
-Tlink.ld to the linker. Nothing is stored in the tree.

Memory
------

========  =========================  ==============================
Region    Origin                     Length
========  =========================  ==============================
FLASH     0x00000000                 64K (lpc1115), 32K (lpc1114)
RAM       0x10000000                 8K
========  =========================  ==============================

Sections
--------

- .isr_vector at flash origin, exactly 0xC0 bytes: 16 core and
  32 IRQ slots. The link fails on any other size.
- .text and .rodata follow.
- .data loads from flash, runs from RAM. _sidata, _sdata, _edata
  mark it.
- .bss in RAM, bounded by _sbss and _ebss.
- _estack is the end of RAM minus 32 bytes. The boot ROM IAP
  routines use the top 32 bytes.
- The link fails if bss plus 0x200 bytes of minimum stack
  exceeds _estack.
- Entry point is hal_boot_reset.
- .ARM.exidx and .ARM.extab are discarded.

Vector checksum
---------------

The boot ROM accepts an image only if the sum of vector words 0
to 7 is zero. Word 7 holds the correction. The linker cannot
compute it. It is applied after objcopy, or by the programmer
at flash time. Decided in the play.sh step.

Rebuild
-------

The script reruns when build.rs or the selected feature changes.