

## Resources

1. Read all of this: <https://www.redblobgames.com/grids/hexagons/>
2. Then read all of this: <https://www.redblobgames.com/grids/hexagons/implementation.html>
3. Goto 1


### WASM Setup

1. Added `[lib]`: `crate-type = ["cydlib"]`
2. Added dependency: `wasm-bindgen = "^0.2"`
3. Previously installed wasm-pack: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
4. Build initial WASM: `wasm-pack build`
5. Build initial web app: `npm init wasm-app rust-hex-app`
