// This script calculates the n-th Fibonacci number using a really dumb algorithm
// to test the speed of the scripting engine.

const target = 30;

fn fib(n) {
    if n < 2 {
         n
    } else {
        fib(n-1) + fib(n-2)
    }
}

print("Ready... Go!");

let now = timestamp();

let result = fib(target);

print("Finished. Run time = " + now.elapsed + " seconds.");

print("Fibonacci number #" + target + " = " + result);

if result != 832_040 {
    print("The answer is WRONG! Should be 832,040!");
}