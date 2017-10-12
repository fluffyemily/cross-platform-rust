//
//  Categories.swift
//  ComplexRust
//
//  Created by Emily Toop on 02/10/2017.
//  Copyright © 2017 Mozilla. All rights reserved.
//

import Foundation


class Categories {

    static func allCategories() -> [Category] {
        let categories = get_all_categories()
        var allCategories: [Category] = []
        for val in UnsafeBufferPointer(start: categories, count: category_list_count(categories)) {
            allCategories.append(Category(raw: val!))
        }
        return allCategories
    }

}
