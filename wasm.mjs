argv.debug = argv.debug == null ? 1 : +argv.debug;
console.log('debug: ', argv.debug)
if (argv.debug == 1) {
    await $`cargo build --target wasm32-unknown-unknown`
    await $`wasm-bindgen target/wasm32-unknown-unknown/debug/bevy-tank-game.wasm --out-dir wasm --web --out-name index`.catch(e => {
        console.log("请按照wasm-bindgen"); throw e;
    })
} else {
    await $`cargo build --target wasm32-unknown-unknown --release`
    await $`wasm-bindgen target/wasm32-unknown-unknown/release/bevy-tank-game.wasm --out-dir wasm --web --out-name index`.catch(e => {
        console.log("请按照wasm-bindgen"); throw e;
    })
}

await $`http-server .`