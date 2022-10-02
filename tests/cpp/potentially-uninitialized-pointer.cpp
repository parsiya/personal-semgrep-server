int main () {

    // ok: potentially-uninitialized-pointer
    char* buf[10];
    // initialize the buffer.
    buf = "Semgrep";
    func(&buf);

    // ruleid: potentially-uninitialized-pointer
    char* buf[10];
    // do stuff here but not with buf.
    func(&buf);
}
