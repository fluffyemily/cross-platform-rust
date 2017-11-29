/* -*- Mode: Java; c-basic-offset: 4; tab-width: 20; indent-tabs-mode: nil; -*-
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package com.mozilla.toodle.rust;

import android.util.Log;

public class ListManager extends RustObject {
    /* package-private */ ListManager(Toodle toodle) {
        rawPointer = JNA.INSTANCE.toodle_list(toodle.rawPointer);
    }

    public void createItem(String name, long dueDate) {
        JNA.INSTANCE.list_manager_create_item_direct(
                rawPointer,
                name,
                dueDate
        );
    }

    public void registerChangedItemsCallback(ItemsChangedCallback callback) {
        JNA.INSTANCE.list_manager_on_items_changed(callback);
    }

    public void getAllUuids(ItemUuidsCallback callback) {
        JNA.INSTANCE.list_manager_all_uuids(rawPointer, callback);
    }

    @Override
    public void close() {
        JNA.INSTANCE.toodle_list_destroy(rawPointer);
    }
}