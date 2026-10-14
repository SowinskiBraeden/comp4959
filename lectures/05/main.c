#include <stdio.h>

#define CHECK(pred) printf("%s ... %s\n", pred ? "passed" : "FAILED", #pred)

int main(void) {
	CHECK(1 + 1 == 2);
	CHECK(1 + 2 == 4);
}
