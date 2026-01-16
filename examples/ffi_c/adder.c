#include <stdlib.h>

int add(int a, int b) {
    return a + b;
}

typedef struct Point { int x; int y; } Point;

int point_manhattan(Point p) {
    return abs(p.x) + abs(p.y);
}
