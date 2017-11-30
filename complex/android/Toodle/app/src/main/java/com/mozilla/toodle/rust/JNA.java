/* -*- Mode: Java; c-basic-offset: 4; tab-width: 20; indent-tabs-mode: nil; -*-
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package com.mozilla.toodle.rust;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.NativeLibrary;
import com.sun.jna.Pointer;

public interface JNA extends Library {
    String JNA_LIBRARY_NAME = "toodle";
    NativeLibrary JNA_NATIVE_LIB = NativeLibrary.getInstance(JNA_LIBRARY_NAME);

    JNA INSTANCE = (JNA) Native.loadLibrary(JNA_LIBRARY_NAME, JNA.class);

    Pointer new_toodle(String dbPath);
    void toodle_destroy(Pointer toodle);

    Pointer toodle_list(Pointer toodle);
    void toodle_list_destroy(Pointer listManager);

    void list_manager_create_item_direct(Pointer listManager, String name, long dueDate);
    void list_manager_on_items_changed(NativeItemsChangedCallback callback);
    void list_manager_all_uuids(Pointer listManager, NativeItemUuidsCallback callback);
    void list_manager_all_items(Pointer listManager, NativeItemsCallback callback);
    void item_jna_destroy(Pointer item);

    // TODO...
    // void a_item_set_name(String uuid, String name);
    // void a_item_set_due_date(String uuid, long dueDate);
    // get items
    // get labels..?
}
