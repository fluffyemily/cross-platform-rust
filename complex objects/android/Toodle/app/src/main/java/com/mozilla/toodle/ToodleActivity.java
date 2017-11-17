package com.mozilla.toodle;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.widget.TextView;

public class ToodleActivity extends AppCompatActivity {
    RustToodle toodle;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_toodle);
    }

    @Override
    protected void onResume() {
        toodle.resume();
        super.onResume();
    }

    @Override
    protected void onPause() {
        toodle.pause();
        super.onPause();
    }
}
