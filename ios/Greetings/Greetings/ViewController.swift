/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import UIKit

class ViewController: UIViewController {

    lazy var label: UILabel = {
        let label = UILabel()
        label.text = "What's your name?"
        label.textAlignment = .center
        return label
    }()

    var textField: UITextField = {
        let textField = UITextField()
        textField.autocapitalizationType = .words
        textField.borderStyle = .roundedRect
        return textField
    }()

    var greetMeButton: UIButton = {
        let button = UIButton(type: .custom)
        button.setTitle("Greet me!", for: .normal)
        button.setTitleColor(.blue, for: .normal)
        button.addTarget(self, action: #selector(performGreeting), for: .touchUpInside)
        return button
    }()

    var startAgainButton: UIButton = {
        let button = UIButton(type: .custom)
        button.setTitle("Start again", for: .normal)
        button.setTitleColor(.blue, for: .normal)
        button.addTarget(self, action: #selector(startAgain), for: .touchUpInside)
        button.isHidden = true
        return button
    }()


    lazy var responseLabel: UILabel = {
        let label = UILabel()
        label.textAlignment = .center
        label.textColor = .magenta
        label.font = UIFont.systemFont(ofSize: 40)
        return label
    }()

    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view, typically from a nib.

        var constraints = [NSLayoutConstraint]()

        view.addSubview(label)
        constraints += [label.topAnchor.constraint(equalTo: view.topAnchor, constant: 40),
                                        label.centerXAnchor.constraint(equalTo: view.centerXAnchor)]

        view.addSubview(textField)
        constraints += [textField.topAnchor.constraint(equalTo: label.bottomAnchor, constant: 20),
                        textField.centerXAnchor.constraint(equalTo: view.centerXAnchor),
                        textField.widthAnchor.constraint(equalToConstant: 200)]

        view.addSubview(greetMeButton)
        constraints += [greetMeButton.topAnchor.constraint(equalTo: textField.bottomAnchor, constant: 20),
                        greetMeButton.centerXAnchor.constraint(equalTo: view.centerXAnchor),
                        greetMeButton.widthAnchor.constraint(equalToConstant: 100)]

        view.addSubview(responseLabel)
        constraints += [responseLabel.centerYAnchor.constraint(equalTo: view.centerYAnchor),
                        responseLabel.centerXAnchor.constraint(equalTo: view.centerXAnchor),
                        responseLabel.widthAnchor.constraint(equalTo: view.widthAnchor)]

        view.addSubview(startAgainButton)
        constraints += [startAgainButton.bottomAnchor.constraint(equalTo: view.bottomAnchor, constant: -20),
                        startAgainButton.centerXAnchor.constraint(equalTo: view.centerXAnchor),
                        startAgainButton.widthAnchor.constraint(equalToConstant: 100)]


        NSLayoutConstraint.activate(constraints, translatesAutoresizingMaskIntoConstraints: false)

    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }

    @objc fileprivate func performGreeting() {
        textField.resignFirstResponder()
        label.isHidden = true
        textField.isHidden = true
        greetMeButton.isHidden = true
        startAgainButton.isHidden = false
        let greetee = textField.text?.isEmpty ?? true ? "there" : textField.text!

        let rustGreetings = RustGreetings()
        responseLabel.text = rustGreetings.sayHello(to: greetee)
    }

    @objc fileprivate func startAgain() {
        label.isHidden = false
        textField.text = nil
        textField.isHidden = false
        greetMeButton.isHidden = false
        startAgainButton.isHidden = true
        responseLabel.text = nil
    }


}

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

