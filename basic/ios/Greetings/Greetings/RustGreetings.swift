/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

class RustGreetings {
    func sayHello(to: String) -> String {
        let result = rust_greeting(to)

        let swift_result = String(cString: result!)

        rust_greeting_free(UnsafeMutablePointer<Int8>(mutating: result))

        return swift_result
        
    }
}
