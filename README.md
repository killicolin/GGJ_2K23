 # GCJ-2k33
 ## Run Native
  ```
 cargo run --release
 ```

 ## Run Web
 ### Local run
  ```
 cargo build --release --target wasm32-unknown-unknown
 wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/ggj_2k33.wasm
\\change ggj_2k33
 npx serve .
  ```


 ### Make itch.io delivery
 ```
 cargo build --release --target wasm32-unknown-unknown
 wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/ggj_2k33.wasm
 \\change ggj_2k33
 ```
 then zip index.html, out and assets folder
