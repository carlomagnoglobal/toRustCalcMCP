FILE I/O

Read and write files.

OPENING & CLOSING

  fd = fopen("file.txt", "r")  Open file (modes: "r"=read, "w"=write, "a"=append)
  fclose(fd)                    Close file

Example:

  fd = fopen("data.txt", "r")
  fgets(fd)                     # Read a line
  fclose(fd)

READING

  fgets(fd)                     Read one line (newline excluded)
  fgetc(fd)                     Read one character
  fread(fd, size)               Read up to size bytes
  fscan(fd, fmt)                Read formatted data (returns list)
  fscanf(fd, fmt, ...)          Read formatted with args

WRITING

  fputs(fd, str)                Write string
  fputc(fd, char)               Write character
  fwrite(fd, data)              Write data
  fprintf(fd, fmt, ...)         Formatted write

Example:

  fd = fopen("out.txt", "w")
  fprintf(fd, "Value: %d\n", 42)
  fclose(fd)

SEEKING & POSITION

  seek(fd, offset)              Move file pointer to offset
  tell(fd)                      Current position
  rewind(fd)                    Seek to start
  eof(fd)                       1 if at end of file

Example:

  seek(fd, 100)                 Move to byte 100
  tell(fd)                      = 100
  rewind(fd)                    Back to start

UTILITIES

  fflush(fd)                    Flush buffered data
  fileno(fd)                    File descriptor number
  fsize(filename)               File size in bytes
  exists(filename)              1 if file exists
  isdir(path)                   1 if directory
  remove(filename)              Delete file
  rename(old, new)              Rename file
  mkdir(path)                   Create directory

Example:

  exists("data.txt")            = 1 or 0
  fsize("data.txt")             = number of bytes
  remove("temp.txt")            Delete temp file