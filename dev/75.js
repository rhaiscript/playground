(this.webpackJsonp=this.webpackJsonp||[]).push([[75],{55:function(n,i,s){"use strict";s.r(i),i.default='// This script runs for-loops\n\nconst MAX = 1_000_000;\n\nprint(`Iterating an array with ${MAX} items...`);\n\nprint("Ready... Go!");\n\nlet now = timestamp();\n\nlet list = [];\n\n// Loop over range\nfor i in 0..MAX {\n    list.push(i);\n}\n\nprint(`Time = ${now.elapsed} seconds...`);\n\nlet sum = 0;\n\n// Loop over array\nfor i in list {\n    sum += i;\n}\n\nprint(`Sum = ${sum}`);\nprint(`Finished. Total run time = ${now.elapsed} seconds.`);\n'}}]);