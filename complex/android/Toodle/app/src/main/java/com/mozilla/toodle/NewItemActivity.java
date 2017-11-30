/* -*- Mode: Java; c-basic-offset: 4; tab-width: 20; indent-tabs-mode: nil; -*-
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package com.mozilla.toodle;

import android.os.Bundle;
import android.support.v7.app.AppCompatActivity;
import android.view.View;
import android.widget.Button;
import android.widget.DatePicker;
import android.widget.TextView;

public class NewItemActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_new_item);

        final TextView itemName = findViewById(R.id.itemName);
        final DatePicker itemDueDate = findViewById(R.id.itemDueDate);
        final Button addBtn = findViewById(R.id.itemAdd);

        addBtn.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                final String name = itemName.getText().toString();
                final int yearDue = itemDueDate.getYear();
                final int monthDue = itemDueDate.getMonth();
                final int dayDue = itemDueDate.getDayOfMonth();

                final Item item = new Item();

                item
                        .name(name)
                        .dueDate(yearDue, monthDue, dayDue)
                        .create(getApplicationContext());

                finish();
            }
        });
    }

}
