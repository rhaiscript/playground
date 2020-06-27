const MAX = 1_000_000;

print("Iterating an array with " + MAX + " items...");

print("Ready... Go!");

let now = timestamp();

let list = [];

for i in range(0, MAX) {
    list.push(i);
}

let sum = 0;

for i in list {
    sum += i;
}

print("Sum = " + sum);
print("Finished. Run time = " + now.elapsed + " seconds.");
