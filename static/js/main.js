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

Rust.stdweb_async.then(async function(stdweb_async) {
    console.log(await stdweb_async.rust_async());
});
