/* -*- Mode: Java; c-basic-offset: 4; tab-width: 20; indent-tabs-mode: nil; -*-
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package com.mozilla.toodle;

import android.content.Context;

import com.mozilla.toodle.rust.ListManager;
import com.mozilla.toodle.rust.Toodle;

import java.util.Date;

class Item {
    private final Context context;

    private String name;
    private Long dueDate;

    Item(Context context) {
        this.context = context;
    }

    Item name(final String name) {
        this.name = name;
        return this;
    }

    Item dueDate(final int year, final int month, final int date) {
        // TODO pretty sure this is wrong, somehow.
        final Date dd = new Date(year, month, date);
        dueDate = dd.getTime();
        return this;
    }

    void save() {
        try (final Toodle toodle = new Toodle(context)) {
            try (final ListManager listManager = toodle.getListManager()) {
                listManager.createItem(name, dueDate);
            }
        }
    }
}