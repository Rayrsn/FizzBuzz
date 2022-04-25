#include <stdio.h>

int main() {
    int countnum;

    printf("How many numbers to display: "); 

    scanf("%d", &countnum);

    for (int i = 1; i <= countnum; i++) {
        if (i % 3 == 0 && i % 5 == 0) {
            printf("FizzBuzz\n");
        } else if (i % 3 == 0) {
            printf("Fizz\n");
        } else if (i % 5 == 0) {
            printf("Buzz\n");
        } else {
            printf("%d\n", i);
        }
    }

    return 0;
}
