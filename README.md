# What is this?

This is an exercise I was asked to do for a job interview, and I decided to do it in rust (which I had never touched before).

# Test Values

I used the following command to create the input.bin file:

```bash
echo -n -e '\x80' > input.bin
```

This command adds a byte of value '80' in hexidecimal (or '10000000' in binary) to the file.

The validity of the file can be checked using the following (on UNIX based systems):

```bash
xxd input.bin
```

The expected output for the left rotation is:

```00000000: 01```

indicating that the byte 0x01 is stored ('00000001' in binary).

The expected output for the right rotation is:

```00000000: 40```

indicating that the byte 0x40 is stored ('01000000' in binary).
