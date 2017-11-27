package com.mozilla.toodle;

import android.os.Bundle;
import android.support.design.widget.FloatingActionButton;
import android.support.design.widget.Snackbar;
import android.support.v7.app.AppCompatActivity;
import android.support.v7.widget.Toolbar;
import android.view.View;
import android.widget.Button;
import android.widget.DatePicker;
import android.widget.EditText;
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

                final RustToodle.Item item = new RustToodle.Item(RustToodle.getInstance(getApplicationContext()).listManager);

                item
                        .name(name)
                        .dueDate(yearDue, monthDue, dayDue)
                        .save();

                finish();
            }
        });
    }

}
