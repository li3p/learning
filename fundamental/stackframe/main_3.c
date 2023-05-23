int swap(int *a, int *b, int arr[5])
{
    int c, d, e;
    // int arr[4] = {0x111, 0x222, 0x333, 0x444};
    d = 0xdd + *a;
    e = 0xee + *b;
    c = *a;
    *a = *b;
    *b = c;
    return c + 2 + d + e - arr[3] + arr[1];
}

int swap0(int *a, int *b)
{
    int c, d, e;
    int arr0[5] = {0x88, 0x11, 0x22, 0x33, 0x44};
    d = 0xdd + *a;
    e = 0xee + *b;
    c = *a;
    *a = *b;
    *b = c;
    int f = swap(&arr0[0], &arr0[1], arr0);
    return f + c + 2 + d + e + arr0[1] + arr0[3] - arr0[4];
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
    c = swap0(&a, &b);
    ret = a - b + c;
    return ret;
}
