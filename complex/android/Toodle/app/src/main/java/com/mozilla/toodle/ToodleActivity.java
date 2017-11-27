package com.mozilla.toodle;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;
import android.support.design.widget.FloatingActionButton;
import android.support.v7.widget.LinearLayoutManager;
import android.support.v7.widget.RecyclerView;
import android.view.View;

public class ToodleActivity extends Activity {
    private RecyclerView listRecyclerView;
    private RecyclerView.Adapter listAdapter;
    private RecyclerView.LayoutManager layoutManager;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_toodle);

        listRecyclerView = findViewById(R.id.listItems);

        layoutManager = new LinearLayoutManager(this);
        listRecyclerView.setLayoutManager(layoutManager);

        listAdapter = new ListAdapter(new String[] {"One", "Two"});
        listRecyclerView.setAdapter(listAdapter);

        final FloatingActionButton newItemBtn = findViewById(R.id.newItem);
        newItemBtn.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                startActivity(new Intent(ToodleActivity.this, NewItemActivity.class));
            }
        });
    }

    @Override
    protected void onResume() {
        RustToodle.getInstance(this).refresh();
        super.onResume();
    }

    @Override
    protected void onPause() {
        RustToodle.getInstance(this).destroy();
        super.onPause();
    }
}
