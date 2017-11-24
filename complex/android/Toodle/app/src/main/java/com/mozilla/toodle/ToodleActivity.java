package com.mozilla.toodle;

import android.app.Activity;
import android.os.Bundle;
import android.support.v7.widget.LinearLayoutManager;
import android.support.v7.widget.RecyclerView;

public class ToodleActivity extends Activity {
    private RecyclerView listRecyclerView;
    private RecyclerView.Adapter listAdapter;
    private RecyclerView.LayoutManager layoutManager;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_toodle);

        listRecyclerView = findViewById(R.id.list_recycler_view);

        layoutManager = new LinearLayoutManager(this);
        listRecyclerView.setLayoutManager(layoutManager);

        listAdapter = new ListAdapter(new String[] {"One", "Two"});
        listRecyclerView.setAdapter(listAdapter);

    }

    @Override
    protected void onResume() {
        RustToodle.getInstance(this).resume();
        super.onResume();
    }

    @Override
    protected void onPause() {
        RustToodle.getInstance(this).pause();
        super.onPause();
    }
}
