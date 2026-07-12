# General example

Full language syntax description.

**Types**
```Rust
Bascics:
- Int
- Float
- Bool
- String

Arrays: [type]
Nullable: type?
```

**Variable declaration**
```kt
// Constant variables
a: Int = 4
b := 4 // Auto-type: int

// Muttable variables
c!: Int = 4;
d! := 4 // Auto-type: int
```

**function declaration**
```rust
// Function with default arguments
pub fn name(arg1: Int, arg2: Float) -> Int { ... }

// Bind arguments type
pub fn name(arg1, arg2: Float) -> Int { ... }
```

**Package**
```rust
pkg org::kiquar::some      // Single in start file

use std::math              // e.g., math::sqrt(9)
use std::math::*           // e.g., sqrt(49)
use std::math::{sqrt, abs} // e.g., abs(sqrt(7))
```

**Class**
```kt
// Extendable class
pub class B! { ... }
// Extends B class
pub class A: B { ... }

// abstract class
class Z? {
    pub fn doSome()
}

// Static methods
class K {
    pub fn ^getName() -> String {
        return "K"
    }
}

name := K::getName()

// Interface (Trait)
pub trait Drawable {
    pub fn getName() -> String
}

// Overrides
use std::io::println
class Picture: Z, Drawable {
    pub fn getName!() {
        return "Picture"
    }
    
    pub fn doSome!() {
        println("Drawing...")
    }
}

// Fields and constructor
class J {
    name: String?
    startAge := 0
    health! := 0
    pub character!: Player? = DefaultPlayer()
    
    pub new(name: String) {
        self.name = name
    }
}

myJ := J("Jorge")
myJ.character = OtherCharacter()

// Inside class can use `Self` instead of name of class
```

**Conditional operator**
```rust
use std::io::*

if a < 0 {
    println("A is less zero")
} else if a > 0 {
    println("A is greater zero")
} else {
    println("A is zero")
}
```

**Loops**
```rust
use std::io::*

// While loop
a! := 0
while a < 10 {
    a += 1
}

// from 0 to 9
for i in 0..10 {
    println(f"I is {i}")
}
// Including value
for i in 0..=10 {
    println(f"I is {i}")
}
// Including value with step
for i in 0..=10:2 {
    println(f"I is {i}")
}
```

