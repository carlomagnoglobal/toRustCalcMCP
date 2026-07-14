# Memory & Stack Management

13 functions.

| Name | Signature | Description |
|------|-----------|-------------|
| `blk` | `blk(size)` | allocate memory block |
| `blkcpy` | `blkcpy(dest,src,size)` | copy memory block |
| `blkfree` | `blkfree(id)` | free memory block |
| `blocks` | `blocks()` | get number of allocated blocks |
| `free` | `free()` | free all allocated memory |
| `freeglobals` | `freeglobals()` | free all global variables |
| `push` | `push(val)` | push value onto evaluation stack |
| `pop` | `pop()` | pop value from evaluation stack |
| `depth` | `depth()` | get evaluation stack depth |
| `blksize` | `blksize(id)` | get size of memory block |
| `peek` | `peek(id,offset)` | read byte from memory block at offset |
| `poke` | `poke(id,offset,val)` | write byte to memory block at offset |
| `memread` | `memread(id,offset,size)` | read bytes from block as string |
