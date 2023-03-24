//! Noop implementation, just for the sake of having a pseudo-syntax.

#![allow(dead_code)]
#![allow(unused_variables)]

macro_rules! setup {
    (fn $name:ident({$($arg:ident: $type:ty),*}) $(-> $ret:ty)? { $body:stmt }) => {
        fn $name($($arg: $type),*) $(-> $ret)? {
            $body
        }
    };
}

macro_rules! destruct {
    ($($name:ident: $type:ty),*) => {
        $(let $name: $type = $name;)*
    };
}

macro_rules! call {
    ($callee:expr, {$($_:ident: $expr:expr),*}) => {
        $callee($($expr),*)
    };
}

setup!(fn do_something({red: u8, green: u8, blue: u8}) {
    destruct!{red: u8, green: u8, blue: u8}
});

pub fn test() {
    call!(do_something, {red: 1, green: 2, blue: 3});
}