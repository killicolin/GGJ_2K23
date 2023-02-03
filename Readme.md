 # Bevy-template
 ## Run Native
  ```
 cargo run --release
 ```
 
 ## Run Web
 ### Local run
  ```
 cargo build --release --target wasm32-unknown-unknown
 wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy_template.wasm 
\\change bevy_template
 npx serve .
  ```


 ### Make itch.io delivery
 ```
 cargo build --release --target wasm32-unknown-unknown
 wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy_template.wasm 
 \\change bevy_template
 ```
 then zip index.html, out and assets folder