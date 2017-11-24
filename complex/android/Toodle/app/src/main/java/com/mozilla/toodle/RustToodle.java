package com.mozilla.toodle;

import android.content.Context;

class RustToodle {
    static {
        System.loadLibrary("toodle");
    }

    private static final String DB_NAME = "toodle.db";

    private Long toodlePtr;
    private final String dbPath;
    final LoginManager loginManager;

    private static RustToodle instance;

    private RustToodle(String dbPath) {
        this.dbPath = dbPath;
        this.loginManager = new LoginManager(this);
    }

    void resume() {
        toodlePtr = newToodle(dbPath);
        loginManager.refresh();
    }

    void pause() {
        toodleDestroy(toodlePtr);
        toodlePtr = null;
    }

    static RustToodle getInstance(final Context context) {
        if (instance == null) {
            instance = new RustToodle(context.getDatabasePath(DB_NAME).getAbsolutePath());
        }
        return instance;
    }

    static class LoginManager {
        private Long loginManagerPtr;
        private final RustToodle toodle;

        LoginManager(RustToodle toodle) {
            this.toodle = toodle;
        }

        private void refresh() {
            loginManagerPtr = loginManager(toodle.toodlePtr);
        }

        Object validate(String username, String password) {
            return validateLogin(loginManagerPtr, username, password);
        }
    }

    private static native long newToodle(final String dbPath);
    private static native void toodleDestroy(final long toodlePtr);

    private static native long loginManager(final long toodlePtr);
    private static native long validateLogin(final long toodlePtr, final String username, final String password);

    private static native void toodleList(final long toodlePtr);
    private static native void newCategory(final long toodlePtr, final String name);
}