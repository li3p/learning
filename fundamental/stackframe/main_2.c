int swap(int *a, int *b)
{
   int c, d, e;
   int arr[10] = {0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99};
   d = 0xdd + *a;
   e = 0xee + *b;
   c = *a;
   *a = *b;
   *b = c;
   return c + 2 + d + e + arr[4] + arr[9] - arr[3];
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
