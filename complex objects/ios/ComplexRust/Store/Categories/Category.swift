//
//  Category.swift
//  ComplexRust
//
//  Created by Emily Toop on 02/10/2017.
//  Copyright © 2017 Mozilla. All rights reserved.
//

import Foundation

class Category: RustObject {
    private let raw: OpaquePointer

    required init(raw: OpaquePointer) {
        self.raw = raw
    }

    deinit {
        category_destroy(raw)
    }

    var id: Int {
        return category_get_id(raw)
    }

    var name: String {
        return String(cString: category_get_name(raw))
    }

    var items: [Item] {
        let items = category_get_items(raw)
        var items_list: [Item] = []
        for item in UnsafeBufferPointer(start: items, count: category_items_count(raw)) {
            items_list.append(Item(raw: item!))
        }
        return items_list
    }
}
