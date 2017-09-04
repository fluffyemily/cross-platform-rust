package com.mozilla.greetings;

/**
 * Created by emilytoop on 01/09/2017.
 */

public class RustGreetings {

    private static native String greeting(final String pattern);

    public String sayHello(String to) {
        return greeting(to);
    }
}
