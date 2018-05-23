package com.mozilla.greetings;

public class RustGreetings {

    public String sayHello(String to) {
        return JNA.INSTANCE.rust_greeting(to);
    }
}
