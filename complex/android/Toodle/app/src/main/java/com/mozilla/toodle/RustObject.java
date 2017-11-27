package com.mozilla.toodle;

abstract class RustObject {
    Long rawPointer;

    void refresh() {}

    void destroy() {
        rawPointer = null;
    }
}
