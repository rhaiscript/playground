// This script tests string operations

print("hello");
print("this\nis \\ nice");      // escape sequences
print("40 hex is \x40");        // hex escape sequence
print("unicode fun: \u2764");   // Unicode escape sequence
print("more fun: \U0001F603");  // Unicode escape sequence
print("foo" + " " + "bar");     // string building using strings
print("foo" < "bar");           // string comparison
print("foo" >= "bar");          // string comparison
print("the answer is " + 42);   // string building using non-string types

let s = "\u2764" hello, world! \U0001F603"; // string variable
print("length=" + s.len);       // should be 17

s[s.len-3] = '?';               // change the string
print("Question: " + s);        // should print 'Question: hello, world?'
