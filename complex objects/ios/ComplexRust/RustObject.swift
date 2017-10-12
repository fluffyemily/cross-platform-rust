//
//  RustObject.swift
//  ComplexRust
//
//  Created by Emily Toop on 12/10/2017.
//  Copyright © 2017 Mozilla. All rights reserved.
//

import Foundation

protocol RustObject {
    init(raw: OpaquePointer)
}
