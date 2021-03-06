#include <stdlib.h>
#include <stdio.h>

typedef enum { NONE, STANDARD, ADMIN } Privilege;

void read_buf_from_socket(char *buf, int size) {
	scanf("%d", buf);
}

void admin_only_func() {
	printf("admin_only_func called!!!\n");
}

void trigger(int *p1, char *p2) {
	// ...

	read_buf_from_socket(p2, 4);

	if (*p1 == ADMIN) {
		admin_only_func();
	}

	// ...
}

void vulnerability_cause_UAF() {
	// ...

	int *p1 = malloc(4);
	*p1 = NONE; // = get_privilege()

	if (*p1 == NONE) {
		free(p1);
		// missing return
	}

	char *p2 = malloc(4);
	trigger(p1, p2);

	// ...
}

int main() {
	vulnerability_cause_UAF();
	return 0;
}
