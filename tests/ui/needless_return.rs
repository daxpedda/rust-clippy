// run-rustfix

#![allow(unused)]
#![allow(
    clippy::if_same_then_else,
    clippy::single_match,
    clippy::branches_sharing_code,
    clippy::needless_bool
)]
#![warn(clippy::needless_return)]

macro_rules! the_answer {
    () => {
        42
    };
}

fn test_end_of_fn() -> bool {
    if true {
        // no error!
        return true;
    }
    return true;
}

fn test_no_semicolon() -> bool {
    return true;
}

fn test_if_block() -> bool {
    if true {
        return true;
    } else {
        return false;
    }
}

fn test_match(x: bool) -> bool {
    match x {
        true => return false,
        false => {
            return true;
        },
    }
}

fn test_closure() {
    let _ = || {
        return true;
    };
    let _ = || return true;
}

fn test_macro_call() -> i32 {
    return the_answer!();
}

fn test_void_fun() {
    return;
}

fn test_void_if_fun(b: bool) {
    if b {
        return;
    } else {
        return;
    }
}

fn test_void_match(x: u32) {
    match x {
        0 => (),
        _ => return,
    }
}

fn read_line() -> String {
    use std::io::BufRead;
    let stdin = ::std::io::stdin();
    return stdin.lock().lines().next().unwrap().unwrap();
}

fn borrows_but_not_last(value: bool) -> String {
    if value {
        use std::io::BufRead;
        let stdin = ::std::io::stdin();
        let _a = stdin.lock().lines().next().unwrap().unwrap();
        return String::from("test");
    } else {
        return String::new();
    }
}

macro_rules! needed_return {
    ($e:expr) => {
        if $e > 3 {
            return;
        }
    };
}

fn test_return_in_macro() {
    // This will return and the macro below won't be executed. Removing the `return` from the macro
    // will change semantics.
    needed_return!(10);
    needed_return!(0);
}

mod issue6501 {
    fn foo(bar: Result<(), ()>) {
        bar.unwrap_or_else(|_| return)
    }

    fn test_closure() {
        let _ = || {
            return;
        };
        let _ = || return;
    }

    struct Foo;
    #[allow(clippy::unnecessary_lazy_evaluations)]
    fn bar(res: Result<Foo, u8>) -> Foo {
        res.unwrap_or_else(|_| return Foo)
    }
}

fn main() {
    let _ = test_end_of_fn();
    let _ = test_no_semicolon();
    let _ = test_if_block();
    let _ = test_match(true);
    test_closure();
}
