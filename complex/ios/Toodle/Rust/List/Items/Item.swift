/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

class Item: RustObject {
    var raw: OpaquePointer

    required init(raw: OpaquePointer) {
        self.raw = raw
    }

    func intoRaw() -> OpaquePointer {
        return self.raw
    }

    deinit {
        item_destroy(raw)
    }

    var uuid: String {
        return String(cString: item_get_uuid(raw))
    }

    var name: String {
        get {
            return String(cString: item_get_name(raw))
        }
        set {
            item_set_name(raw, newValue)
        }
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
                let timestamp = d.timeIntervalSince1970
                var date = Int64(timestamp)
                item_set_due_date(raw, AutoreleasingUnsafeMutablePointer<Int64>(&date))
            }
        }
    }

    var completionDate: Date? {
        get {
            guard let date = item_get_completion_date(raw) else {
                return nil
            }
            return Date(timeIntervalSince1970: Double(date.pointee))
        }
        set {
            if let d = newValue {
                let timestamp = d.timeIntervalSince1970
                var date = Int64(timestamp)
                item_set_completion_date(raw, AutoreleasingUnsafeMutablePointer<Int64>(&date))
            }
        }
    }

    fileprivate var _labels: [Label]?

    var labels: [Label] {
        get {
            if _labels == nil {
                let ls = item_get_labels(self.raw)
                _labels = []
                for index in 0..<item_labels_count(ls) {
                    let label = Label(raw: item_label_at(ls, index)!)
                    _labels?.append(label)
                }
            }

            return _labels!
        }
        set {
            _labels = nil
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
