int main() {

    // ok: encode-decode-function-name
    randomfunc();

    // ruleid: encode-decode-function-name
    encodeme();

    int param = 1;
    // ruleid: encode-decode-function-name
    int res = decodeme(param);

    // ruleid: encode-decode-function-name
    thisencodefunctionisbad();

    // ok: encode-decode-function-name
    int res2 = safefunction();

    return 0;
}
