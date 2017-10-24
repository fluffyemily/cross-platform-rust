/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import UIKit

extension NSLayoutConstraint {
    open class func activate(_ constraints: [NSLayoutConstraint], translatesAutoresizingMaskIntoConstraints: Bool) {
        for constraint in constraints {
            if let view = constraint.firstItem as? UIView {
                view.translatesAutoresizingMaskIntoConstraints = translatesAutoresizingMaskIntoConstraints
            }
        }
        NSLayoutConstraint.activate(constraints)
    }
}
