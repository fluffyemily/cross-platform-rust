/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import UIKit

protocol LoginViewControllerDelegate {
    func accountCreationSuccess(withUsername username: String, andPassword password: String)
}

private let LoggedInKey = "LoggedIn"

class LoginViewController: UIViewController {

    lazy var newLoginVC: NewLoginViewController = {
        let newVC = NewLoginViewController()
        newVC.delegate = self
        return newVC
    }()

    lazy var dbStore: ToodleLib = {
        return ToodleLib.sharedInstance
    }()

    var messageLabel: UILabel = {
        let label = UILabel()
        label.lineBreakMode = .byWordWrapping
        label.textAlignment = .center
        label.textColor = .red
        label.font = UIFont.boldSystemFont(ofSize: 20)
        return label
    }()

    var usernameTextField: UITextField = {
        let textField = UITextField()
        textField.placeholder = "username"
        textField.autocapitalizationType = .none
        textField.borderStyle = .roundedRect
        return textField
    }()

    var passwordTextField: UITextField = {
        let textField = UITextField()
        textField.placeholder = "password"
        textField.isSecureTextEntry = true
        textField.autocapitalizationType = .none
        textField.borderStyle = .roundedRect
        return textField
    }()

    var signInButton: UIButton = {
        let button = UIButton(type: .custom)
        button.setTitle("Sign in", for: .normal)
        button.setTitleColor(.blue, for: .normal)
        button.addTarget(self, action: #selector(validateLogin), for: .touchUpInside)
        return button
    }()

    var signUpButton: UIButton = {
        let button = UIButton(type: .custom)
        button.setTitle("Sign up", for: .normal)
        button.setTitleColor(.blue, for: .normal)
        button.addTarget(self, action: #selector(openSignUp), for: .touchUpInside)
        return button
    }()

    override func viewDidLoad() {
        super.viewDidLoad()

        var constraints = [NSLayoutConstraint]()

        view.addSubview(usernameTextField)
        constraints += [usernameTextField.topAnchor.constraint(equalTo: view.topAnchor, constant: 140),
                        usernameTextField.centerXAnchor.constraint(equalTo: view.centerXAnchor),
                        usernameTextField.widthAnchor.constraint(equalToConstant: 200)]

        view.addSubview(passwordTextField)
        constraints += [passwordTextField.topAnchor.constraint(equalTo: usernameTextField.bottomAnchor, constant: 20),
                        passwordTextField.centerXAnchor.constraint(equalTo: view.centerXAnchor),
                        passwordTextField.widthAnchor.constraint(equalToConstant: 200)]

        view.addSubview(signUpButton)
        constraints += [signUpButton.topAnchor.constraint(equalTo: passwordTextField.bottomAnchor, constant: 20),
                        signUpButton.centerXAnchor.constraint(equalTo: view.centerXAnchor, constant: -40),
                        signUpButton.widthAnchor.constraint(equalToConstant: 100)]

        view.addSubview(signInButton)
        constraints += [signInButton.topAnchor.constraint(equalTo: passwordTextField.bottomAnchor, constant: 20),
                        signInButton.centerXAnchor.constraint(equalTo: view.centerXAnchor, constant: 40),
                        signInButton.widthAnchor.constraint(equalToConstant: 100)]

        view.addSubview(messageLabel)
        constraints += [messageLabel.topAnchor.constraint(equalTo: signInButton.bottomAnchor, constant: 40),
                        messageLabel.centerXAnchor.constraint(equalTo: view.centerXAnchor),
                        messageLabel.widthAnchor.constraint(equalTo: view.widthAnchor)]

        NSLayoutConstraint.activate(constraints, translatesAutoresizingMaskIntoConstraints: false)

        if UserDefaults.standard.bool(forKey: LoggedInKey) {
            self.openToDoList()
        }
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }

    @objc fileprivate func validateLogin() {
        print("Signing in...")

        if let username = self.usernameTextField.text,
            let password = self.passwordTextField.text {
            self.signIn(withUsername: username, andPassword: password)
        }
    }

    fileprivate func signIn(withUsername username: String, andPassword password: String) {
        switch self.dbStore.logins.validateLogin(withUsername: username, andPassword: password) {
        case .valid:
            // remember successful sign in so we don't have to do it every time
            UserDefaults.standard.set(true, forKey: LoggedInKey)
            self.openToDoList()
        case .incorrectPassword:
            self.messageLabel.text = "Incorrect password for username"
        case .unknownUsername:
            self.messageLabel.text = "That username does not exist"
        case .invalid:
            self.messageLabel.text = "The provided login details are not recognized"
        }
    }

    @objc fileprivate func openSignUp() {
        self.newLoginVC.usernameTextField.text = self.usernameTextField.text
        self.newLoginVC.passwordTextField.text = self.passwordTextField.text
        self.present(self.newLoginVC, animated: true, completion: nil)
    }

    fileprivate func openToDoList() {
        let categoryVC = ToDoListCategoryViewController()
        categoryVC.dbStore = self.dbStore
        self.navigationController?.pushViewController(categoryVC, animated: true)
    }
}

extension LoginViewController: LoginViewControllerDelegate {
    func accountCreationSuccess(withUsername username: String, andPassword password: String) {
        self.signIn(withUsername: username, andPassword: password)
    }
}

