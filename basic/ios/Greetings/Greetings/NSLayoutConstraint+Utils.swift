//
//  NSLayoutConstraint+Utils.swift
//  Greetings
//
//  Created by Emily Toop on 28/09/2017.
//  Copyright © 2017 Mozilla. All rights reserved.
//

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
