(this.webpackJsonp=this.webpackJsonp||[]).push([[78],{59:function(n,a,e){"use strict";e.r(a),a.default="//! This script defines a function with two parameters and local variables.\n\nlet a = 3;\n\nfn add(a, b) {\n    a = 42;             // notice that 'a' is passed by value\n    a + b;              // notice that the last value is returned even if terminated by a semicolon\n}\n\nlet result = add(a, 4);\n\nprint(`add(a, 4) should be 46: ${result}`);\n\nprint(`a should still be 3: ${a}`);     // prints 3: 'a' is never changed\n"}}]);