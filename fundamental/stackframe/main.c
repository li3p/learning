int swap(int *a, int *b)
{
   int c, d, e;
   d = 0xdd + *a;
   e = 0xee + *b;
   c = *a;
   *a = *b;
   *b = c;
   return c + 2 + d + e;
}

int main(void)
{
   int a;
   int b;
   int ret;
   int c;
   a = 16;
   b = 64;
   ret = 10;
   c = swap(&a, &b);
   ret = a - b + c;
   return ret;
}
