# TEST VALUES

I used the following command to create the input.bin file:

```bash
echo -n -e '\x80' > input.bin
```

The validity of the file can be checked using the following (on UNIX based systems):

```bash
xxd input.bin
```
