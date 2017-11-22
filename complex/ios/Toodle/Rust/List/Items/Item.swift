/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

class Item: RustObject {
    let raw: OpaquePointer

    required init(raw: OpaquePointer) {
        self.raw = raw
    }

    init() {
        self.raw = item_new()
    }

    deinit {
        item_destroy(raw)
    }

    var id: Int? {
        return item_get_id(raw)?.pointee
    }

    var description: String {
        get {
            return String(cString: item_get_description(raw))
        }
        set {
            item_set_description(raw, newValue)
        }
    }

    var createdAt: Date {
        return Date(timeIntervalSinceReferenceDate: Double(item_get_created_at(raw)))
    }

    var dueDate: Date? {
        get {
            guard let date = item_get_due_date(raw) else {
                return nil
            }
            return Date(timeIntervalSince1970: Double(date.pointee))
        }
        set {
            if let d = newValue {
                item_set_due_date(raw, Int64(d.timeIntervalSince1970))
            }
        }
    }

    var isComplete: Bool {
        get {
            return item_get_is_complete(raw) != 0
        }
        set {
            item_set_is_complete(raw, newValue ? 1 : 0)
        }
    }

    func dueDateAsString() -> String? {
        guard let dueDate = self.dueDate else {
            return nil
        }
        let dateFormatter = DateFormatter()
        dateFormatter.dateFormat = "yyyy-MM-dd'T'HH:mm:ss.SSSZ"
        return dateFormatter.string(from: dueDate)
    }
}
