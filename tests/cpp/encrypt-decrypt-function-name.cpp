int main() {

    // ok: encrypt-decrypt-function-name
    randomfunc();

    // ruleid: encrypt-decrypt-function-name
    decryptme();

    int param = 1;
    // ruleid: encrypt-decrypt-function-name
    int res = thisencrypts(param);

    // ruleid: encrypt-decrypt-function-name
    thisencryptfunctionisbad();

    // ok: encrypt-decrypt-function-name
    int res2 = safefunction();

    return 0;
}
