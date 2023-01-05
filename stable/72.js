(this.webpackJsonp=this.webpackJsonp||[]).push([[72],{53:function(n,e,t){"use strict";t.r(e),e.default='//! This script illustrates how to put doc-comments on functions.\n\n/// The function `foo`, which prints `hello, world!` and a magic number,\n/// accepts three parameters.\n///\n/// # Parameters\n///\n/// * `x` - `i64`\n/// * `y` - `string`\n/// * `z` - `bool`\n///\n/// # Notes\n///\n/// This is a doc-comment.  It can be obtained with the `metadata` feature.\n///\n/// An example is the `rhai-doc` app.\n///\n/// # Example\n///\n/// ```rhai\n/// let x = foo(42, "hello", true);\n///\n/// print(x);     // prints 47\n/// ```\nfn foo(x, y, z) {\n   print(`hello, world! ${if z { x + y.len() } else { x } }`);\n}\n\nfoo(39, "bar", true);\n'}}]);