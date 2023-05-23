```bash
$ gcc -g -o main main.c
$ objdump -d main > main.dump
$ gcc -Wall -S -o main.s main.c
```