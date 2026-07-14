# File I/O

24 functions.

| Name | Signature | Description |
|------|-----------|-------------|
| `fopen` | `fopen(filename,mode)` | open file (mode: 'r', 'w', 'a') |
| `fclose` | `fclose(fd)` | close file |
| `fgets` | `fgets(fd)` | read line from file |
| `fgetc` | `fgetc(fd)` | read character from file |
| `fputs` | `fputs(fd,str)` | write string to file |
| `fputc` | `fputc(fd,ch)` | write character to file |
| `seek` | `seek(fd,offset)` | seek to position in file |
| `tell` | `tell(fd)` | get current position in file |
| `eof` | `eof(fd)` | check if at end-of-file |
| `remove` | `remove(filename)` | delete file |
| `rename` | `rename(old,new)` | rename file |
| `fflush` | `fflush(fd)` | flush file buffer |
| `rewind` | `rewind(fd)` | rewind file to beginning |
| `fileno` | `fileno(fd)` | get file descriptor number |
| `fread` | `fread(fd,size)` | read bytes from file |
| `fwrite` | `fwrite(fd,data)` | write bytes to file |
| `fseek` | `fseek(fd,offset,whence)` | seek with whence (0=SET, 1=CUR, 2=END) |
| `fprintf` | `fprintf(fd,...)` | formatted write to file |
| `fscan` | `fscan(fd,fmt)` | read formatted data from file (returns list) |
| `fscanf` | `fscanf(fd,fmt,...)` | read formatted data with arguments (returns list) |
| `fsize` | `fsize(filename)` | get file size in bytes |
| `exists` | `exists(filename)` | check if file exists (returns 1 or 0) |
| `isdir` | `isdir(path)` | check if path is directory (returns 1 or 0) |
| `mkdir` | `mkdir(path)` | create directory (returns 0 on success) |
