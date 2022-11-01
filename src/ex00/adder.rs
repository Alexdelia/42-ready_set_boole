// add 2 u32 without using + operator
fn adder(a: u32, b: u32) -> u32 {
    let mut x = a;
    let mut carry = b;
    while carry != 0 {
        let tmp = x ^ carry;
        carry = (x & carry) << 1;
        x = tmp;
    }
    x
}

/*
    explanation
a = 1 = 0001 (binary)
b = 3 = 0011 (binary)

^ = XOR:
---------
1 ^ 1 = 0
1 ^ 0 = 1
0 ^ 1 = 1
0 ^ 0 = 0
---------
example:
0001	(1)
0011	(3)
----
0010	(2)

& = AND:
---------
1 & 1 = 1
1 & 0 = 0
0 & 1 = 0
0 & 0 = 0
---------
example:
0001	(1)
0011	(3)
----
0001	(1)

<< = shift left
(x << 1 == x * 2)
---------
1 << 1 = 10
0 << 1 = 00
11 << 1 = 110
10 << 1 = 100
00 << 1 = 000
---------
example:
0101	(5)
1010	(10)

x = 1
carry = 3

while carry != 0
    tmp = x ^ carry
    //  = 1 ^ 3 = 2
    //  = 0001 ^ 0011 = 0010
    carry = (x & carry) << 1
    //    = (1 & 3) << 1
    //    = (0001 & 0011) << 1
    //    = 0001 << 1
    //    = 0010 = 2
    x = tmp

*/
