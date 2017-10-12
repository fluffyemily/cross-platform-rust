//
//  Store.swift
//  ComplexRust
//
//  Created by Emily Toop on 17/10/2017.
//  Copyright © 2017 Mozilla. All rights reserved.
//

import Foundation

class Store: RustObject {
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

    func validateLogin(withUsername username: String, andPassword password: String) -> LoginStatus {
        return LoginStatus(rawValue: validate_login(raw, username, password)) ?? .invalid
    }

    func createLogin(withUsername username: String, andPassword password: String) -> Login {
        return Login(raw: create_login(raw, username, password)!)
    }
}
