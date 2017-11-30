package com.mozilla.toodle.rust;

import com.sun.jna.Structure;

import java.io.Closeable;
import java.util.Arrays;
import java.util.List;


public class NativeItemSet extends Structure implements Closeable {
    public static class ByReference extends NativeItemSet implements Structure.ByReference {
    }

    public static class ByValue extends NativeItemSet implements Structure.ByValue {
    }

    public NativeItem.ByReference items;
    public int numberOfItems;

    public List<NativeItem> getItems() {
        final NativeItem[] array = (NativeItem[]) items.toArray(numberOfItems);
        return Arrays.asList(array);
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("items", "numberOfItems");
    }

    @Override
    public void close() {
        final NativeItem[] nativeItems = (NativeItem[]) items.toArray(numberOfItems);
        for (NativeItem nativeItem : nativeItems) {
            nativeItem.close();
        }
    }
}
