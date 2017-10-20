
class LoginManager: RustObject {
    let raw: OpaquePointer

    required init(raw: OpaquePointer) {
        self.raw = raw
    }

    func validateLogin(withUsername username: String, andPassword password: String) -> LoginStatus {
        return LoginStatus(rawValue: validate_login(self.raw, username, password)) ?? .invalid
    }

    func createLogin(withUsername username: String, andPassword password: String) -> Login? {
        return Login(raw: create_login(self.raw, username, password))
    }
}
