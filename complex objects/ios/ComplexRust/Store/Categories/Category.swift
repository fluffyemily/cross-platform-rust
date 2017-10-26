/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

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

    fileprivate var _items: [Item] = []

    var items: [Item] {
        if self.refreshItems {
            self._items = self.fetchItems()
            self.refreshItems = false
        }

        return self._items
    }

    fileprivate var refreshItems = true

    func add_item(item: Item) {
        category_add_item(self.raw, item.raw)
        self.refreshItems = true
    }

    fileprivate func fetchItems() -> [Item] {
        let items = category_get_items(self.raw)
        var items_list: [Item] = []
        for index in 0..<category_items_count(self.raw) {
            items_list.append(Item(raw: category_item_at(items, index)!))
        }
        return items_list
    }
}
