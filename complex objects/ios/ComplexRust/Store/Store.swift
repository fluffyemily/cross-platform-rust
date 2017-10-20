//
//  Store.swift
//  ComplexRust
//
//  Created by Emily Toop on 17/10/2017.
//  Copyright © 2017 Mozilla. All rights reserved.
//

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
