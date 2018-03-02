#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate futures_await as futures;
#[macro_use]
extern crate stdweb;

use futures::prelude::*;
use stdweb::PromiseFuture;
use stdweb::js_export;
use stdweb::unstable::TryInto;

/// rust_async should return a Promise that returns a retun value of a Promise of js_async defined in main.js.
#[js_export]
#[async]
fn rust_async() -> Result<i32, stdweb::error::Error> {
    let future: PromiseFuture<SomeType> = js!( return await js_async(); ).try_into().unwrap();
    PromiseFuture::spawn(
        future
            .map(|value| {
                // I want to return variable value to JS as Promise.
            })
            .map_err(|e| console!(error, e)),
    );
    // Since spawn returns (), there are no ways to return a Promise-like something...
}
