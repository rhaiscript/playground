// This script simulates object-oriented programming (OOP) techniques
// using function pointers (Fn) and object maps.

// Define object
let obj1 = #{
    _data: 42,                      // data field
    get_data: Fn("getData"),        // property getter
    action: Fn("action"),           // method
    update: Fn("update1")           // property setter
};

fn getData() {
    this._data
}
fn action() {
    print("Data=" + this._data);
}
fn update1(x) {
    this._data = x;
    this.action();
}

if obj1.get_data() > 0 {            // property access
    obj1.update(123);               // call method
} else {
    print("we have a problem here");
}

// Define another object based on the first object
let obj2 = #{
    _data: 0,                       // data field - new value
    update: Fn("update2")           // property setter - another function
};
obj2.fill_with(obj1);               // add all other fields from obj1

fn update2(x) {
    this._data = x * 2;
    this.action();
}

if obj2.get_data() > 0 {            // property access
    obj2.update(0);                 // call method
} else {
    obj2.update(42);                // call method
}
