cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/conways_game_of_life.wasm
mv ./out/conways_game_of_life.js ./out/game.js
wasm-opt -Oz -o ./out/conways_game_of_life_bg.wasm ./out/conways_game_of_life_bg.wasm
rm ./out/conways_game_of_life.d.ts
rm ./out/conways_game_of_life_bg.wasm.d.ts
