# basic
Example project showing a rust library running on multiple platforms.

Current version shows a simple rust library that passes only primatives running on an iOS and Android apps only. Web based examples are coming soon.

First, we have to [install Xcode](https://itunes.apple.com/us/app/xcode/id497799835?ls=1&amp;mt=12) and then set up Xcode build tools. If you already have the build tools installed and they are up to date, you can skip this step.

```
xcode-select --install
```

Next, we need to ensure that Rust is installed and that we can cross compile to the iOS architectures. For this we will be using [rustup](https://www.rustup.rs/). If you already have rustup installed, you can skip this step. Rustup installs Rust from the official release channels and enables you to easily switch between different release versions. It will be useful to you for all your future Rust development, not just here.
```
curl https://sh.rustup.rs -sSf | sh
```

Add the iOS architectures to rustup so we can use them during cross compilation.
```
rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios
```

When you installed Rust, it also installed cargo, which is a package manager similar to pip, gems etc. Now we will use cargo to install `cargo-lipo`. This is a cargo subcommand which automatically creates a universal library for use with iOS. Without this crate, cross compiling Rust to work on iOS is infinitely harder.
```
cargo install cargo-lipo
```

We need to build our library against the iOS architectures using `cargo-lipo`. The built artifacts of will be placed in `cargo/target/`. The universal iOS library that we are interested in can be found in `cargo/target/universal/release/libgreetings.a`.

```
cd cargo
cargo lipo --release
```

Open `ios/Greetings/Greetings.xcodeproj` in Xcode. Select the Greetings project from the project navigator, and then ensure the Greetings target is selected. Open the `General` tab. Scroll down to the `Linked Frameworks and Libraries` section. Import your `libgreetings.a` library by either dragging it in from finder, or clicking the + at the bottom of the list, clicking 'Add other...' and navigating to `cargo/target/universal/release/`. Select `libgreetings.a` and then click `Open`.

You should now be able to build and run your iOS app.
