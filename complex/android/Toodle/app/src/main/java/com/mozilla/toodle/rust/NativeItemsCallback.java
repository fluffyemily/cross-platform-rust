package com.mozilla.toodle.rust;

import com.sun.jna.Callback;

public interface NativeItemsCallback extends Callback {
    void items(NativeItemSet.ByReference items);
}
