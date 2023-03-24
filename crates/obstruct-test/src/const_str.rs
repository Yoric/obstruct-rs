#![allow(dead_code)]
#![allow(unused_variables)]

macro_rules! setup {
    (fn $name:ident({$($arg:ident: $type:ty),*}) $(-> $ret:ty)? { $body:stmt }) => {
        // Outer function for inlining purposes.
        fn $name($($arg: $type),*) $(-> $ret)? {
            fn $name($($arg: $type),*) $(-> $ret)? {
                $body
            }
            $name($($arg),*)
        }
    };
}

macro_rules! destruct {
    ($($name:ident: $type:ty),*) => {
        $(let $name: $type = $name;)*
    };
}

setup!(fn do_something({red: u8, green: u8, blue: u8}) {
    destruct!{red: u8, green: u8, blue: u8}
});

trait Field<T> {
    const NAME: &'static str;
    fn take(self) -> T;
}
macro_rules! call {
    ($callee:ident, $($field:ident = $value:expr),*) => {
        $callee(($(
            {
                #[allow(non_camel_case_types)]
                struct $field<T>(T);
                impl<T> Field<T> for $field<T> {
                    const NAME: &'static str = stringify!($field);
                    fn take(self) -> T {
                        self.0
                    }
                }
                $field($value)
            }
        ),*))
    };
}

#[inline(always)]
fn do_something_outer<A, B, C>(temporary_name: (A, B, C)) // FIXME: Might be useful to implement some kind of reordering.
    where A: Field<u8, NAME="red">,
          B: Field<u8, NAME="green">,
          C: Field<u8, NAME="blue">,
{
    let (red, green, blue) = temporary_name;
    let red = red.take();
    let green = green.take();
    let blue = blue.take();
    do_something_inner(red, green, blue);
}

fn do_something_inner(_red: u8, _green: u8, _blue: u8) {
    unimplemented!()
}

// FIXME: We could have `do_something` desugar to an outer function (for inlining purposes) and an inner function (with regular arguments).

pub fn test() {
    call!(do_something_outer, red = 0, green = 1, blue = 2);
    //call!(do_something_outer, red = 0, green = 1);
    //call!(do_something_outer, red = 0, green = 1, blue = 2, alpha = 3);
    //call!(do_something_outer, red = 0, green = 1, blue = 0.1);

    //call!(do_something_outer, red = 0, green = 1, yellow = 2);
        // type mismatch resolving `<yellow<u8> as Field<u8>>::NAME == "blue"`
        //  expected constant `"blue"`
        //  found constant `"yellow"`
}

