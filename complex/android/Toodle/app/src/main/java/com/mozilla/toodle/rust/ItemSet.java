package com.mozilla.toodle.rust;

import com.sun.jna.Structure;

import java.io.Closeable;
import java.io.IOException;
import java.util.Arrays;
import java.util.List;


public class ItemSet extends Structure implements Closeable {
    public static class ByReference extends ItemSet implements Structure.ByReference {
    }

    public static class ByValue extends ItemSet implements Structure.ByValue {
    }

    public Item.ByReference items;

    public int numberOfItems;

    public List<Item> getItems() {
        Item[] array = (Item[]) items.toArray(numberOfItems);
        return Arrays.asList(array);
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("items", "numberOfItems");
    }

    @Override
    public void close() throws IOException {
        // TODO
    }
}
