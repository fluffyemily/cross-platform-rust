package com.mozilla.toodle.rust;

import com.sun.jna.Structure;

import java.io.Closeable;
import java.io.IOException;
import java.util.Arrays;
import java.util.List;

public class Item extends Structure implements Closeable {
    public static class ByReference extends Item implements Structure.ByReference {
    }

    public static class ByValue extends Item implements Structure.ByValue {
    }

    public String uuid;
    public String itemName;
//    public Long dueDate;
//    public Long completionDate;

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("uuid", "itemName");//, "dueDate", "completionDate");
    }

    @Override
    public void close() throws IOException {
        // TODO
    }
}
