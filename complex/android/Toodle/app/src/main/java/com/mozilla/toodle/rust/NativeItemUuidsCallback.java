package com.mozilla.toodle.rust;

import com.sun.jna.Callback;

public interface NativeItemUuidsCallback extends Callback {
    void uuids(UuidSet.ByReference uuids);
}
