/* global Rust */

function sleep(ms) {
    return new Promise(function(res, rej) {
        setTimeout(res, ms);
    });
}

async function js_async() {
    await sleep(1000); // sleep is a just example of async function.
    return 1;
}

Rust.stdweb_async.then(function(stdweb_async) {
    return new Promise(function(res, rej) {
        window.resolve = res;
        stdweb_async.rust_async();
    });
}).then(function(value) {
    console.log(value);
});
