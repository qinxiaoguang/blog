# 调用该方法的时候，可能会卡住，原因是Cargo.toml里的wasm-bindgen的版本和安装的不一致
# 解决方法是使用cargo install wasm-bindgen-cli命令再安装一次，会提示对应版本，将Caogo.toml里的版本填写一致即可
wasm-pack build --target no-modules

rm pkg/.gitignore
