int main() {

    int oneDim[10];
    int multipleDims[10][20][30][40] = {0};

    // ruleid: arrays-passed-to-functions
    int ret = func(oneDim);

    // ruleid: arrays-passed-to-functions
    int ret2 = func(30, multipleDims, 20, 40);

    // ok: arrays-passed-to-functions
    int* ptr1 = (int*) malloc(sizeof(int)*2);
    // ok: arrays-passed-to-functions
    free(ptr1);

    // ok: arrays-passed-to-functions
    int* ptr2 = (int*) calloc(2, sizeof(int));

    // ok: arrays-passed-to-functions
    int* ptr3 = (int*) realloc(ptr2, sizeof(int));

    // ruleid: arrays-passed-to-functions
    func(10, ptr3, 20);
    // ruleid:arrays-passed-to-functions
    func2(20, 30, ptr3);

    delete(ptr3);

	// ok: arrays-passed-to-functions
    int var = 10;

    static const char TEXT_TYPE[] = "TextType";
    // ok: arrays-passed-to-functions
    mTextType = Process(TEXT_TYPE);

    // ok: arrays-passed-to-functions
    char c[7];
    char name[] = "Parsia";
    memcpy(c, name, 7);
}