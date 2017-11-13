/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

class ListManager: RustObject {
    let raw: OpaquePointer

    required init(raw: OpaquePointer) {
        self.raw = raw
    }

    func allCategories() -> [Category] {
        let categories = get_all_categories(self.raw)
        var allCategories: [Category] = []
        for index in 0..<category_list_count(categories) {
            let category = Category(raw: category_list_item_at(categories, index))
            allCategories.append(category)
        }
        return allCategories
    }

    func createCategory(withName name: String) -> Category {
        return Category(raw: category_new(self.raw, name))
    }

    func add(item: Item, toCategory category: Category) {
        category_manager_create_item(raw, item.raw, category.id)
    }

    func update(item: Item) throws {
        category_manager_update_item(raw, item.raw)
    }
}
