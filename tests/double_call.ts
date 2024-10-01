function fun() {
    return function() {
        return 1
    };
}

fun();

// this does not work, because of parser
// fun()();