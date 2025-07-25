# Utopia Examples

## Multi-Language Example

```utopia
@lang python {
def py_add(a, b):
    return a + b
}

@lang c {
int c_mul(int a, int b) {
    return a * b;
}
}

@lang javascript {
function js_sub(a, b) {
    return a - b;
}
}

@lang main {
    let x = 5;
    let y = 3;
    let sum = python::py_add(x, y);
    let product = c::c_mul(x, y);
    let diff = javascript::js_sub(x, y);
    println("Sum:", sum);
    println("Product:", product);
    println("Difference:", diff);
}
```

## Control Flow and Functions

```utopia
@lang main {
    let score = 85;
    if (score >= 90) {
        println("Grade: A");
    } else if (score >= 80) {
        println("Grade: B");
    } else {
        println("Grade: C or below");
    }
}
```

## Data Processing Example

```utopia
@lang python {
def process(data):
    return [x * 2 for x in data]
}

@lang main {
    let arr = [1, 2, 3];
    let result = python::process(arr);
    println(result);
}
```

See [docs/language-reference.md](language-reference.md) for more details on language blocks and cross-language calls. 