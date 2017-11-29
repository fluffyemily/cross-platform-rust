package com.mozilla.toodle.rust;

import com.sun.jna.Callback;

public interface ItemUuidsCallback extends Callback {
    void uuids(UuidSet.ByReference uuids);
}
