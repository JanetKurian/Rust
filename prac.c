#include <stdio.h>
#include <string.h>

void removeDuplicates(char *str)
{
    int seen[256] = {0};  // ASCII map
    int i, j = 0;

    for (i = 0; str[i] != '\0'; i++)
    {
        if (seen[(unsigned char)str[i]] == 0)
        {
            seen[(unsigned char)str[i]] = 1;
            str[j++] = str[i];  // keep character
        }
    }

    str[j] = '\0';  // terminate string
}

int main()
{
    char str[] = "abbada";

    removeDuplicates(str);

    printf("Result: %s\n", str);

    return 0;
}


//////
#include <stdio.h>
#include <string.h>

int main()
{
    char s[] = "janet";
    char c[] = "abcdefghijklmnopqrstuvwxyz";

    for(int i = 0; i < strlen(s); i++)
    {
        for(int j = 0; j < strlen(c); j++)
        {
            if(s[i] == c[j])
            {
                printf("%c", s[i]);
                break;
            }
        }
    }

    return 0;
}

/////
#include <stdio.h>
#include <stdint.h>

void bitshift(uint32_t value)
{
    uint32_t set_bit = value | (1 << 3);
    printf("set_bit = 0x%X\n", set_bit);

    uint32_t toggle_bit = value ^ (1 << 3);
    printf("toggle_bit = 0x%X\n", toggle_bit);

    uint32_t clear_bit = value & ~(1 << 4);
    printf("clear_bit = 0x%X\n", clear_bit);
}

int main()
{
    uint32_t Value = 0x195DD258;

    printf("initial = 0x%X\n", Value);

    bitshift(Value);

    return 0;
}

/////
