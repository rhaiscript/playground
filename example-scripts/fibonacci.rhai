// This script calculates the n-th Fibonacci number using a really dumb algorithm
// to test the speed of the scripting engine.

const target = 28;

fn fib(n) {
    if n < 2 {
         n
    } else {
        fib(n-1) + fib(n-2)
    }
}

print("Running Fibonacci(28) x 5 times...");
print("Ready... Go!");

let result;
let now = timestamp();

for n in range(0, 5) {
	result = fib(target);
}

print(`Finished. Run time = ${now.elapsed} seconds.`);

print(`Fibonacci number #${target} = ${result}`);

if result != 317_811 {
    print("The answer is WRONG! Should be 317,811!");
}
