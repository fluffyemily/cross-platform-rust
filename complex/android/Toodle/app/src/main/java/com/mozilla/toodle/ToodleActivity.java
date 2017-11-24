package com.mozilla.toodle;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;

public class ToodleActivity extends AppCompatActivity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_toodle);
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
