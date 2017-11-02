/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

class Store: RustObject {

    class var sharedInstance: Store {
        struct Static {
            static let instance: Store = Store()
        }
        return Static.instance
    }

    var raw: OpaquePointer

    required init(raw: OpaquePointer) {
        self.raw = raw
    }

    convenience init() {
        let paths = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
        let documentsURL = paths[0]
        let storeURI = documentsURL.appendingPathComponent("todolist.db", isDirectory: false).absoluteString
        self.init(raw: new_store(storeURI))
    }

    var logins: LoginManager {
        return LoginManager(raw: store_logins(self.raw));
    }

    var categories: CategoryManager {
        return CategoryManager(raw: store_categories(self.raw));
    }
}

class Singleton {
}
