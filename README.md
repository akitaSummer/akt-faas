## Akt-Faas

### What is this

This is a faas demo.

Your Android Studio must have :
```
// go to Android Studio > Preferences > Appearance & Behaviour > Android SDK > SDK Tools. 

* Android SDK Tools 
* NDK (21<=)
* CMake
```

then run:
```
rustup target add aarch64-linux-android x86_64-apple-darwin x86_64-pc-windows-msvc x86_64-unknown-linux-gnu x86_64-apple-darwin aarch64-apple-darwin aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
```

###  How to build 
First, you must have deno.
Then, you can run
```
deno run -A --unstable ./mod.ts
curl http://localhost:3000/test1
```
to test.