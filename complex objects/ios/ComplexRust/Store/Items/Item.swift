//
//  Items.swift
//  ComplexRust
//
//  Created by Emily Toop on 12/10/2017.
//  Copyright © 2017 Mozilla. All rights reserved.
//

import Foundation

class Item: RustObject {
    private let raw: OpaquePointer

    required init(raw: OpaquePointer) {
        self.raw = raw
    }

    init() {
        self.raw = item_new();
    }

    deinit {
        item_destroy(raw)
    }
}
