#include <stdlib.h>
#include <stdio.h>
#include <string.h>

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
int main()
{
    int width;
    scanf("%d", &width); fgetc(stdin);
    int height;
    scanf("%d", &height); fgetc(stdin);
    char text[256];
    fgets(text, 256, stdin);
	char font[1024][1024];
    for (int i = 0; i < height; ++i) {
        fgets(font[i], 1024, stdin);
        fprintf(stderr, "font line %s", font[i]);
    }
	int textLength = strlen(text) - 1;
	fprintf(stderr, "The text %s\n ", text);
	for (int i = 0; i < height; ++i) {
		for(int j = 0; j < textLength; ++j) {
			int offset;
			if(toupper(text[j]) < 65 || toupper(text[j]) > 90) {
				offset = 26 * width;
			} else {
				offset = (toupper(text[j]) - 'A') * width;
			}
			fprintf(stderr, "the offset %d\n", offset);
			for(int k = 0; k < width; ++k) {
				putchar(font[i][offset+k]);
			}
		}
		printf("\n");
	}
}
