package com.mozilla.toodle;

import android.content.Context;

import java.util.Date;

class RustToodle extends RustObject {
    static {
        System.loadLibrary("toodle");
    }

    private static final String DB_NAME = "toodle.db";

    private final String dbPath;

    final ListManager listManager;

    private static RustToodle instance;

    private RustToodle(String dbPath) {
        this.dbPath = dbPath;
        this.listManager = new ListManager(this);
    }

    @Override
    void refresh() {
        rawPointer = newToodle(dbPath);
        listManager.refresh();
    }

    @Override
    void destroy() {
        toodleDestroy(rawPointer);
        super.destroy();
    }

    static RustToodle getInstance(final Context context) {
        if (instance == null) {
            instance = new RustToodle(context.getDatabasePath(DB_NAME).getAbsolutePath());
        }
        return instance;
    }

    static class ListManager extends RustObject {
        private final RustToodle toodle;

        ListManager(RustToodle toodle) {
            this.toodle = toodle;
        }

        @Override
        void refresh() {
            rawPointer = listManager(toodle.rawPointer);
        }
    }

    static class Item extends RustObject {
        private final ListManager listManager;

        Item(ListManager listManager) {
            this.listManager = listManager;
            this.rawPointer = newItem();
        }

        Item name(final String name) {
            itemSetName(rawPointer, name);
            return this;
        }

        Item dueDate(final int year, final int month, final int date) {
            final Date dueDate = new Date(year, month, date);
            final long timestamp = dueDate.getTime();
            itemSetDueDate(rawPointer, timestamp);
            return this;
        }

        void save() {
            itemCreate(listManager.rawPointer, rawPointer);
        }
    }

    private static native long newToodle(final String dbPath);
    private static native void toodleDestroy(final long toodlePtr);

    private static native long listManager(final long toodlePtr);
    private static native long newItem();
    private static native long itemSetName(final long itemPtr, final String name);
    private static native long itemSetDueDate(final long itemPtr, final long dueDate);
    private static native void itemCreate(final long listManagerPtr, final long itemPtr);
}