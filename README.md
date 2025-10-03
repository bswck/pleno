*At this point we already know enough to understand the basic steps used by every linker.*

## Useful stuff

- `nm` for listing symbols!

  Consider this simple program from Anthony Sottile for generating UUIDs in C.
  
  ```c
  #include <uuid/uuid.h>
  #include <stdio.h>

  int main() {
      uuid_t u;
      uuid_generate(u);
      char uuid_str[UUID_STR_LEN];
      uuid_unparse(u, uuid_str);
      printf("%s\n", uuid_str);
      return 0;
  }
  ```

  If `-l uuid` is not supplied to `gcc`, `ld` fails with an undefined reference:

  ```
  /usr/bin/ld: /tmp/ccSq2ZeM.o: in function `main':
  t.c:(.text+0x10): undefined reference to `uuid_generate'
  collect2: error: ld returned 1 exit status
  ```

  After successful compilation and linking, the nm output of the binary is:

  ```
  00000000004012a0 r __abi_tag
  000000000040300c B __bss_start
  000000000040300c b completed.0
  0000000000403008 D __data_start
  0000000000403008 W data_start
  00000000004003c0 t deregister_tm_clones
  00000000004003b0 T _dl_relocate_static_pie
  0000000000400430 t __do_global_dtors_aux
  0000000000402df0 d __do_global_dtors_aux_fini_array_entry
  0000000000401198 R __dso_handle
  0000000000402df8 d _DYNAMIC
  000000000040300c D _edata
  0000000000403010 B _end
  0000000000400484 T _fini
  0000000000400460 t frame_dummy
  0000000000402de8 d __frame_dummy_init_array_entry
  0000000000401258 r __FRAME_END__
  0000000000402fe8 d _GLOBAL_OFFSET_TABLE_
                   w __gmon_start__
  00000000004011a0 r __GNU_EH_FRAME_HDR
  000000000040033c T _init
  0000000000401190 R _IO_stdin_used
                   U __libc_start_main@GLIBC_2.34
  0000000000400466 T main
  00000000004003f0 t register_tm_clones
  0000000000400380 T _start
  0000000000403010 D __TMC_END__
                   U uuid_generate@UUID_1.0
  ```

## Checklist

- [ ] Read the input object files. Determine the length and type of the contents. Read the symbols.
- [ ] Build a symbol table containing all the symbols, linking undefined symbols to their definitions.
- [ ] Decide where all the contents should go in the output executable file, which means deciding where they should go in memory when the program runs.
- [ ] Read the contents data and the relocations. Apply the relocations to the contents. Write the result to the output file.
- [ ] Optionally write out the complete symbol table with the final values of the symbols.
More tomorrow.


(“Linkers part 2”, Ian Lance Taylor)