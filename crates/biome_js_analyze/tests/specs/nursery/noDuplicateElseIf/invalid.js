if (isSomeThing(x)) {
    foo();
} else if (isSomeThing(x)) {
    bar();
}

if (a) {
    foo();
} else if (b) {
    bar();
} else if (c && d) {
    baz();
} else if (c && d) {
    quux();
} else {
    quuux();
}

if (n === 1) {
    foo();
} else if (n === 2) {
    bar();
} else if (n === 3) {
    baz();
} else if (n === 2) {
    quux();
} else if (n === 5) {
    quuux();
}

// && and || operators
if (a || b) {
    foo();
} else if (a) {
    bar();
}

if (a) {
    foo();
} else if (b) {
    bar();
} else if (a || b) {
    baz();
}

if (a) {
    foo();
} else if (a && b) {
    bar();
}

if (a && b) {
    foo();
} else if (a && b && c) {
    bar();
}

if (a || b) {
    foo();
} else if (b && c) {
    bar();
}

if (a) {
    foo();
} else if (b && c) {
    bar();
} else if (d && (c && e && b || a)) {
    baz();
}
