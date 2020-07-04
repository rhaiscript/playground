rhai-playground
===

This is an attempt to make a playground for the [Rhai scripting language][rhai]
using WebAssembly.

The `master` branch is automatically built and deployed [here][playground-unstable].

[rhai]: https://github.com/jonathandturner/rhai
[playground-unstable]: https://alvinhochun.github.io/rhai-playground-unstable/


Embedding the Rhai playground
---

It is still a work-in-progress, but it is possible to embed the playground for
use on another web page.

[View demo on CodePen](https://codepen.io/alvinhochun/pen/LYGQEaW)

```html
<style>
iframe.rhaiPlayground {
    width: 100%;
    height: 400px;
    border: 0;
}
</style>

<pre><code class="language-rhai">fn hello_rhai(msg) {
    print("Hello world! " + msg);
}
hello_rhai("Embed the Rhai Playground to run Rhai code!");
</code></pre>

<script>
const ORIGIN = "https://alvinhochun.github.io";
const PATH = "/rhai-playground-unstable/";
let nextPlaygroundIdx = 0;
function loadPlayground(el) {
    const id = "" + nextPlaygroundIdx++;
    const iframe = document.createElement("iframe");
    iframe.style = el.style;
    iframe.className = el.className;
    iframe.classList.add("rhaiPlayground");
    iframe.src = ORIGIN + PATH + "#embed-" + id;
    el.replaceWith(iframe);
    const code = el.innerText;
    const onMessage = ev => {
        if (
            ev.data.from === "rhai-playground" &&
            ev.data.req === "embed-loaded" &&
            ev.data.id === id
        ) {
            iframe.contentWindow.postMessage(
                {
                    to: "rhai-playground",
                    req: "embed-init",
                    code,
                },
                ORIGIN,
            );
            // window.removeEventListener("message", onMessage);
        }
    }
    window.addEventListener("message", onMessage);
}

document.querySelectorAll("code.language-rhai").forEach(el => {
    loadPlayground(el);
});
</script>
```


## How to install

```sh
npm install
```

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

## How to run unit tests

```sh
# Runs tests in Firefox
npm test -- --firefox

# Runs tests in Chrome
npm test -- --chrome

# Runs tests in Safari
npm test -- --safari
```

## What does each file do?

* `Cargo.toml` contains the standard Rust metadata. You put your Rust dependencies in here. You must change this file with your details (name, description, version, authors, categories)

* `package.json` contains the standard npm metadata. You put your JavaScript dependencies in here. You must change this file with your details (author, name, version)

* `webpack.config.js` contains the Webpack configuration. You shouldn't need to change this, unless you have very special needs.

* The `js` folder contains your JavaScript code (`index.js` is used to hook everything into Webpack, you don't need to change it).

* The `src` folder contains your Rust code.

* The `static` folder contains any files that you want copied as-is into the final build. It contains an `index.html` file which loads the `index.js` file.

* The `tests` folder contains your Rust unit tests.
