int main() {

    // ok: arrays-out-of-bounds-access
    int arr[100];
    arr[10] = 10;

    // ruleid: arrays-out-of-bounds-access
    int arr2[100];
    arr2[100] = 100;

    // ok: arrays-out-of-bounds-access   
    int arr3[100];
    int nem = arr3[10];

    // ruleid: arrays-out-of-bounds-access
    int arr4[100];
    int nem = arr4[120];

    return 0;
}