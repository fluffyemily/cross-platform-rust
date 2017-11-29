package com.mozilla.toodle.rust;

import com.sun.jna.Structure;

import java.io.Closeable;
import java.io.IOException;
import java.util.Arrays;
import java.util.List;

public class UuidSet extends Structure implements Closeable {
    public static class ByReference extends UuidSet implements Structure.ByReference {
    }

    public static class ByValue extends UuidSet implements Structure.ByValue {
    }

    public Uuid.ByReference uuids;

    public int numberOfUuids;

    public List<Uuid> getUuids() {
        Uuid[] array = (Uuid[]) uuids.toArray(numberOfUuids);
        return Arrays.asList(array);
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("uuids", "numberOfUuids");
    }

    @Override
    public void close() throws IOException {
        // TODO
    }
}
