const warp = import('../wasm/warp_engine');

var test;

warp.then(module => {
    module.eval_test('./script.js')
})