/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

class LoginManager: RustObject {
    let raw: OpaquePointer

    required init(raw: OpaquePointer) {
        self.raw = raw
    }

    func validateLogin(withUsername username: String, andPassword password: String) -> LoginStatus {
        return LoginStatus(rawValue: validate_login(self.raw, username, password).rawValue) ?? .invalid
    }

    func createLogin(withUsername username: String, andPassword password: String) -> Login? {
        return Login(raw: create_login(self.raw, username, password))
    }
}
