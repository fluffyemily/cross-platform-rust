package com.mozilla.toodle.rust;

import com.sun.jna.Structure;

import java.io.Closeable;
import java.io.IOException;
import java.util.Arrays;
import java.util.List;

public class Uuid extends Structure implements Closeable {
    public static class ByReference extends Uuid implements Structure.ByReference {
    }

    public static class ByValue extends Uuid implements Structure.ByValue {
    }

    public String uuid;

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("uuid");
    }

    @Override
    public void close() throws IOException {
        // TODO
    }
}
