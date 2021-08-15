# WebAssembly初体验

## hello-wasm
[MDN教程](https://developer.mozilla.org/zh-CN/docs/WebAssembly/Rust_to_wasm)

### 安装准备
- 安装Rust
- 安装wasm-pack 
```
cargo install wasm-pack
```
- 安装node.js并注册账户
```powershell
> npm adduser
Username: $username
Password:
Email: (this IS public) $you@example.com
```

### 构建 WebAssembly npm 包

#### 新建项目
```
cargo new --lib hello-wasm

+-- Cargo.toml
+-- src
    +-- lib.rs
```

#### lib.rs
```rust
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

#### Cargo.toml
```toml
[package]
name = "hello-wasm"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A sample project with wasm-pack"
license = "MIT/Apache-2.0"
repository = "https://github.com/yourgithubusername/hello-wasm"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

#### 构建发布
```shell
# 构建
wasm-pack build --scope $mynpmusername
# 发布
cd pkg
npm publish --access=public
```

### 在webpack项目使用

#### 目录
```
+-- site
    +-- package.json
    +-- webpack.config.js
    +-- index.html
```

#### package.json
```json
{
  "scripts": {
    "serve": "webpack-dev-server"
  },
  "dependencies": {
    "@mynpmusername/hello-wasm": "^0.1.0"
  },
  "devDependencies": {
    "webpack": "^4.25.1",
    "webpack-cli": "^3.1.2",
    "webpack-dev-server": "^3.1.10"
  }
}

``` 

#### webpack.config.js
```js
const path = require('path');
module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development"
};

```

#### index.html
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>hello-wasm example</title>
  </head>
  <body>
    <script src="./index.js"></script>
  </body>
</html>
```

#### index.js
```js
const js = import("./node_modules/@yournpmusername/hello-wasm/hello_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
});

```

#### 启动
```shell
$ npm install
$ npm run serve
```

## 待续...
