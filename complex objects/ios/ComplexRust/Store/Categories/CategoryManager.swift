//
//  CategoryManager.swift
//  ComplexRust
//
//  Created by Emily Toop on 20/10/2017.
//  Copyright © 2017 Mozilla. All rights reserved.
//

import Foundation

class CategoryManager: RustObject {
    let raw: OpaquePointer

    required init(raw: OpaquePointer) {
        self.raw = raw
    }

    func allCategories() -> [Category] {
        let categories = get_all_categories(self.raw)
        var allCategories: [Category] = []
        for val in UnsafeBufferPointer(start: categories, count: category_list_count(categories)) {
            allCategories.append(Category(raw: val!))
        }
        return allCategories
    }

    func createCategory(withName name: String) -> Category {
        return Category(raw: create_category(self.raw, name))
    }
}
