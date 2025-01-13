extern int putchar(int c);

static int main(void)
{
    extern int x;

    static int y = 10;

    x = ~~((!(-(2 + 3 * 4) / 2 % 7 == 0) != 1) && (2 + 3 * 4 > 10) || (5 - 3 <= 1));

    ;

    int z;
    if (y)
    {
        if (y > 15)
            z = 1;
        else
        {
            z = 2;
            z = 4;
        }
        int y = 5;
    }

    switch (y)
    {
    case 10:
        y = 11;
    case 20:
    {
        y = 12;
        break;
    }
    default:
        y = 13;
    }

    int w = (z == 1) ? (z * 2) : (z * 3);

    while (z > 0)
    {
        z = z - 1;
    }

    for (int i = 0; i < w; i = i + 1)
    {
        z = z + 1;
    }

    do
    {
        z = z - 1;
    } while (z > 0);

    y *= 20;

    putchar(72);
    putchar(101);
    putchar(108);
    putchar(108);
    putchar(111);
    putchar(44);
    putchar(32);
    putchar(87);
    putchar(111);
    putchar(114);
    putchar(108);
    putchar(100);
    putchar(33);
    putchar(10);

    return y;
}