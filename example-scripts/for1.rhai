// This script runs for-loops

let arr = [1, 2, 3, 4];

for a in arr {
    for b in [10, 20] {
        print(a + "," + b);
    }

    if a == 3 { break; }
}
//print(a);                 // <- if you uncomment this line, the script will fail to run
                            //    because 'a' is not defined here

for i in range(0, 5) {      // runs through a range from 0 to 4
    print(i);
}
