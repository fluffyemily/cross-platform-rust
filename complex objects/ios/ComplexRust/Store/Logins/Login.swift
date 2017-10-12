//
//  Logins.swift
//  ComplexRust
//
//  Created by Emily Toop on 25/09/2017.
//  Copyright © 2017 Mozilla. All rights reserved.
//

import Foundation

enum LoginStatus: Int {
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
        return LoginStatus(rawValue: login_is_valid(raw)) ?? .invalid
    }
}
