package com.mozilla.toodle.rust;

import com.sun.jna.Callback;

public interface ItemsCallback extends Callback {
    void items(ItemSet.ByReference items);
}
