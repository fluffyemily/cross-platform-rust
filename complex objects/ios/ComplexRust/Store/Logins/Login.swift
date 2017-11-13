/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

enum LoginStatus: UInt32 {
    case valid = 0
    case unknownUsername = 1
    case incorrectPassword = 2
    case invalid = 3
}

class Login: RustObject {
    private let raw: OpaquePointer

    required init(raw: OpaquePointer) {
        self.raw = raw
    }

    deinit {
        login_destroy(raw)
    }

    var id: Int {
        return login_get_id(raw)
    }

    var username: String {
        return String(cString: login_get_username(raw)!)
    }

    var password: String {
        return String(cString: login_get_password(raw)!)
    }

    var guid: String {
        get {
            return String(cString: login_get_guid(raw)!)
        }

        set {
            login_set_guid(raw, UnsafeMutablePointer<Int8>(mutating: newValue))
        }
    }

    var timeCreated: Int {
        return login_get_time_created(raw)
    }

    var timeLastUsed: Int {
        return login_get_time_last_used(raw)
    }

    var timePasswordChanged: Int {
        return login_get_time_password_changed(raw)
    }

    var timesUsed: Int {
        return login_get_times_used(raw)
    }

    var isValid: LoginStatus {
        let loginStatus = login_is_valid(raw)
        return LoginStatus(rawValue: loginStatus.rawValue) ?? .invalid
    }
}
