use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(inline_js = r#"
export function spawn_worker() {
    const code = `
        console.log("hello from worker");
        console.warn("worker warning message");
        console.error("worker error message");
    `;
    const blob = new Blob([code], { type: 'application/javascript' });
    new Worker(URL.createObjectURL(blob));
}

export function spawn_worker_with_error() {
    const code = `
        console.log("worker about to throw");
        throw new Error("intentional worker error");
    `;
    const blob = new Blob([code], { type: 'application/javascript' });
    new Worker(URL.createObjectURL(blob));
}

export function spawn_worker_with_rejection() {
    const code = `
        console.log("worker about to reject");
        Promise.reject(new Error("intentional unhandled rejection"));
    `;
    const blob = new Blob([code], { type: 'application/javascript' });
    new Worker(URL.createObjectURL(blob));
}

export function spawn_worker_with_trusted_url() {
    if (typeof trustedTypes === 'undefined') {
        console.log("trustedTypes not supported, skipping TrustedScriptURL test");
        return;
    }

    const code = `
        console.log("hello from worker with TrustedScriptURL");
    `;
    const blob = new Blob([code], { type: 'application/javascript' });
    const blobUrl = URL.createObjectURL(blob);

    // Create a TrustedTypePolicy that allows our blob URLs
    const policy = trustedTypes.createPolicy('worker-policy', {
        createScriptURL: (url) => url
    });
    const trustedUrl = policy.createScriptURL(blobUrl);
    new Worker(trustedUrl);
}
"#)]
extern "C" {
    fn spawn_worker();
    fn spawn_worker_with_error();
    fn spawn_worker_with_rejection();
    fn spawn_worker_with_trusted_url();
}

fn main() {
    log("spawning worker with console logs");
    spawn_worker();

    log("spawning worker that throws an error");
    spawn_worker_with_error();

    log("spawning worker with unhandled rejection");
    spawn_worker_with_rejection();

    log("spawning worker with TrustedScriptURL");
    spawn_worker_with_trusted_url();
}
