/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import UIKit

class NewLoginViewController: UIViewController {

    var delegate: LoginViewControllerDelegate?

    lazy var newAccountLabel: UILabel = {
        let label = UILabel()
        label.text = "Provide a username and password to create a new account"
        label.textAlignment = .center
        label.lineBreakMode = .byWordWrapping
        label.numberOfLines = 0
        return label
    }()

    var usernameTextField: UITextField = {
        let textField = UITextField()
        textField.placeholder = "username"
        textField.autocapitalizationType = .words
        textField.borderStyle = .roundedRect
        return textField
    }()

    var passwordTextField: UITextField = {
        let textField = UITextField()
        textField.placeholder = "password"
        textField.isSecureTextEntry = true
        textField.autocapitalizationType = .words
        textField.borderStyle = .roundedRect
        return textField
    }()

    var passwordConfirmationTextField: UITextField = {
        let textField = UITextField()
        textField.placeholder = "confirm password"
        textField.isSecureTextEntry = true
        textField.autocapitalizationType = .words
        textField.borderStyle = .roundedRect
        return textField
    }()

    var signInButton: UIButton = {
        let button = UIButton(type: .custom)
        button.setTitle("Create account", for: .normal)
        button.setTitleColor(.blue, for: .normal)
        button.addTarget(self, action: #selector(createAccount), for: .touchUpInside)
        return button
    }()

    var cancelButton: UIButton = {
        let button = UIButton(type: .custom)
        button.setTitle("Cancel", for: .normal)
        button.setTitleColor(.blue, for: .normal)
        button.addTarget(self, action: #selector(cancel), for: .touchUpInside)
        return button
    }()
    
    override func viewDidLoad() {
        super.viewDidLoad()

        self.view.backgroundColor = UIColor.white

        // Do any additional setup after loading the view.
        var constraints = [NSLayoutConstraint]()

        view.addSubview(newAccountLabel)
        constraints += [newAccountLabel.topAnchor.constraint(equalTo: view.topAnchor, constant: 100),
                        newAccountLabel.widthAnchor.constraint(equalTo: view.widthAnchor, constant: -50),
                        newAccountLabel.centerXAnchor.constraint(equalTo: view.centerXAnchor)]

        view.addSubview(usernameTextField)
        constraints += [usernameTextField.topAnchor.constraint(equalTo: newAccountLabel.bottomAnchor, constant: 40),
                        usernameTextField.centerXAnchor.constraint(equalTo: view.centerXAnchor),
                        usernameTextField.widthAnchor.constraint(equalToConstant: 200)]

        view.addSubview(passwordTextField)
        constraints += [passwordTextField.topAnchor.constraint(equalTo: usernameTextField.bottomAnchor, constant: 20),
                        passwordTextField.centerXAnchor.constraint(equalTo: view.centerXAnchor),
                        passwordTextField.widthAnchor.constraint(equalToConstant: 200)]

        view.addSubview(passwordConfirmationTextField)
        constraints += [passwordConfirmationTextField.topAnchor.constraint(equalTo: passwordTextField.bottomAnchor, constant: 20),
                        passwordConfirmationTextField.centerXAnchor.constraint(equalTo: view.centerXAnchor),
                        passwordConfirmationTextField.widthAnchor.constraint(equalToConstant: 200)]

        view.addSubview(cancelButton)
        constraints += [cancelButton.topAnchor.constraint(equalTo: passwordConfirmationTextField.bottomAnchor, constant: 20),
                        cancelButton.rightAnchor.constraint(equalTo: view.centerXAnchor, constant: -40),
                        cancelButton.widthAnchor.constraint(equalToConstant: 100)]

        view.addSubview(signInButton)
        constraints += [signInButton.topAnchor.constraint(equalTo: cancelButton.topAnchor),
                        signInButton.leftAnchor.constraint(equalTo: view.centerXAnchor, constant: 40),
                        signInButton.widthAnchor.constraint(equalToConstant: 140)]

        NSLayoutConstraint.activate(constraints, translatesAutoresizingMaskIntoConstraints: false)
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
    
    @objc fileprivate func createAccount() {
        print("Creating account")
        guard let username = self.usernameTextField.text,
            let password = self.passwordTextField.text,
            let confirmedPassword = self.passwordConfirmationTextField.text else {
                return print("Empty username & password field")
        }
        guard password == confirmedPassword else {
            return print("Password's do not match")
        }
        guard let _ = ToodleLib.sharedInstance.logins.createLogin(withUsername: username, andPassword: password) else {
            return print("failed to create login")
        }
        self.delegate?.accountCreationSuccess(withUsername: username, andPassword: password)
        self.dismiss(animated: true, completion: nil)
    }

    @objc fileprivate func cancel() {
        self.dismiss(animated: true, completion: nil)
    }

}
