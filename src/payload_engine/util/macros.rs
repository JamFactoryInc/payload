macro_rules! generate_descending_struct {
    ($name:ident; $leading_id:ident; $leading_type:ty, $($ids:ident; $types:ty),*) => {
        paste::item! {
            pub struct [<$name _ $leading_type>] <$leading_type, $($types),*> {
                $leading_id: $leading_type,
                $(
                    $ids: $types,
                )*
            }
        }

        generate_descending_struct!{$name; $($ids; $types),*}
    };
    ($name:ident; $leading_id:ident; $leading_type:ty) => {
        paste::item! {
            pub struct [<$name _ $leading_type>]<$leading_type> {
                $leading_id: $leading_type,
            }
        }
    };
}

macro_rules! generate_descending_fn {
    // largest (denoted by # before fn)
    (#fn $name:ident<$leading_targ:ident : Pattern,$($t:ident : Pattern),*>($info_arg:ident: $info_type:ty, $leading_arg:ident: $leading_type:ty,$($arg:ident: $arg_type:ty),*)) => {
        paste::item! {
            fn [<$name _ $leading_targ>]<$leading_targ : Pattern, $($t : Pattern),*>($info_arg: $info_type, $leading_arg: $leading_type, $($arg: $arg_type),*) -> *const usize {
                println!("Generated!");
                println!("{}", std::any::type_name::<$leading_targ>());
                $(
                    println!("{}", std::any::type_name::<$t>());
                )*
                Box::into_raw(Box::new(UberPattern::<$leading_targ, $($t),*> {
                    $leading_arg,
                    $(
                        $arg,
                    )*
                })) as *const usize
            }

            generate_descending_fn!{fn $name [<$name _ $leading_targ>]<$($t : Pattern),*>($info_arg: $info_type, $($arg: $arg_type),*)}
        }

    };
    (fn $name:ident $super_name:ident<$leading_targ:ident : Pattern,$($t:ident : Pattern),*>($info_arg:ident: $info_type:ty, $leading_arg:ident: $leading_type:ty,$($arg:ident: $arg_type:ty),*)) => {
        paste::item! {
            fn [<$name _ $leading_targ>]<$leading_targ : Pattern, $($t : Pattern),*>($info_arg: $info_type, $leading_arg: $leading_type, $($arg: $arg_type),*) -> *const usize {
                if $info_arg == 0 {
                    println!("Was 0");
                    $super_name($info_arg, $leading_arg, $($arg),*, 0u32)
                } else {
                    println!("Was 1");
                    $super_name($info_arg, $leading_arg, $($arg),*, "wow")
                }
            }

            generate_descending_fn!{fn $name [<$name _ $leading_targ>]<$($t : Pattern),*>($info_arg: $info_type, $($arg: $arg_type),*)}
        }

    };
    // root layer
    (fn $name:ident $super_name:ident<$leading_targ:ident : Pattern>($info_arg:ident: $info_type:ty, $leading_arg:ident: $leading_type:ty)) => {
        paste::item! {
            fn [<$name _ $leading_targ>]<$leading_targ : Pattern>($info_arg: $info_type, $leading_arg: $leading_type) -> *const usize {
                if $info_arg == 0 {
                    println!("Was 0");
                    $super_name($info_arg, $leading_arg, 0u32)
                } else {
                    println!("Was 1");
                    $super_name($info_arg, $leading_arg, "wow")
                }
            }
        }

        fn $name($info_arg: $info_type) -> *const usize {
            paste::item! {
                if $info_arg == 0 {
                    println!("Was 0");
                    [<$name _ $leading_targ>]($info_arg, 0u32)
                } else {
                    println!("Was 1");
                    [<$name _ $leading_targ>]($info_arg, "wow")
                }
            }
        }
    }
}

macro_rules! generate_descending_from_ids {
    ($($types:ident; $vars:ident),*) => {

        pub struct UberPattern <
        $(
            $types
        ),*> {
            $(
                $vars: $types,
            )*
        }

        generate_descending_fn! {
            #fn generated_fn<
            $(
                $types: Pattern
            ),*>(info: u32, $(
                $vars: $types
            ),*)
        }
    };
}

macro_rules! pair_args {
    ($($ids:ident),*) => {
        paste::item! {
            generate_descending_from_ids! {
                $(
                    $ids; [<$ids _var>]
                ),*
            }

            generate_descending_struct! {
                Group;
                $(
                    [<$ids _var>]; $ids
                ),*
            }
        }
    }
}

macro_rules! delete {
    ( 'internal_call $collector:tt # $something:tt $($t:tt)* ) => {
        delete!{ 'internal_call $collector $($t)* }
    };
    ( 'internal_call ($($collector:tt)*) $($stuff:ident),+ # $something:tt $($t:tt)* ) => {
        delete!{ 'internal_call ($($collector)* $($stuff),+)  $($t)* }
    };
    ('internal_call ($($collector:tt)*) $stuff:tt $($t:tt)* ) => {
        delete!{ 'internal_call ($($collector)* $stuff) $($t)* }
    };
    ('internal_call ($($stuff:tt)*)) => {
        $($stuff)*
    };
    ( $($t:tt)* ) => {
        delete!{ 'internal_call () $($t)* }
    };
}



macro_rules! sample_angle {
    ( (($($before:tt)*) ($($after:tt)*)) ($($delim:tt)?) ($d0:tt) (($($s0:tt)*) $($sample:tt)*) ) => {
        $($before)* <$($s0)*> $($after)*
    };
( (($($before:tt)*) ($($after:tt)*)) ($($delim:tt)?) ($d0:tt $d1:tt) (($($s0:tt)*) ($($s1:tt)*) $($sample:tt)*) ) => {
        $($before)* <$($s0)* $($delim)? $($s1)*> $($after)*
    };
( (($($before:tt)*) ($($after:tt)*)) ($($delim:tt)?) ($d0:tt $d1:tt $d2:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) $($sample:tt)*) ) => {
        $($before)* <$($s0)* $($delim)? $($s1)* $($delim)? $($s2)*> $($after)*
    };
( (($($before:tt)*) ($($after:tt)*)) ($($delim:tt)?) ($d0:tt $d1:tt $d2:tt $d3:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) $($sample:tt)*) ) => {
        $($before)* <$($s0)* $($delim)? $($s1)* $($delim)? $($s2)* $($delim)? $($s3)*> $($after)*
    };
( (($($before:tt)*) ($($after:tt)*)) ($($delim:tt)?) ($d0:tt $d1:tt $d2:tt $d3:tt $d4:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) ($($s4:tt)*) $($sample:tt)*) ) => {
        $($before)* <$($s0)* $($delim)? $($s1)* $($delim)? $($s2)* $($delim)? $($s3)* $($delim)? $($s4)*> $($after)*
    };
( (($($before:tt)*) ($($after:tt)*)) ($($delim:tt)?) ($d0:tt $d1:tt $d2:tt $d3:tt $d4:tt $d5:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) ($($s4:tt)*) ($($s5:tt)*) $($sample:tt)*) ) => {
        $($before)* <$($s0)* $($delim)? $($s1)* $($delim)? $($s2)* $($delim)? $($s3)* $($delim)? $($s4)* $($delim)? $($s5)*> $($after)*
    };
( (($($before:tt)*) ($($after:tt)*)) ($($delim:tt)?) ($d0:tt $d1:tt $d2:tt $d3:tt $d4:tt $d5:tt $d6:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) ($($s4:tt)*) ($($s5:tt)*) ($($s6:tt)*) $($sample:tt)*) ) => {
        $($before)* <$($s0)* $($delim)? $($s1)* $($delim)? $($s2)* $($delim)? $($s3)* $($delim)? $($s4)* $($delim)? $($s5)* $($delim)? $($s6)*> $($after)*
    };
( (($($before:tt)*) ($($after:tt)*)) ($($delim:tt)?) ($d0:tt $d1:tt $d2:tt $d3:tt $d4:tt $d5:tt $d6:tt $d7:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) ($($s4:tt)*) ($($s5:tt)*) ($($s6:tt)*) ($($s7:tt)*) $($sample:tt)*) ) => {
        $($before)* <$($s0)* $($delim)? $($s1)* $($delim)? $($s2)* $($delim)? $($s3)* $($delim)? $($s4)* $($delim)? $($s5)* $($delim)? $($s6)* $($delim)? $($s7)*> $($after)*
    };
( (($($before:tt)*) ($($after:tt)*)) ($($delim:tt)?) ($d0:tt $d1:tt $d2:tt $d3:tt $d4:tt $d5:tt $d6:tt $d7:tt $d8:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) ($($s4:tt)*) ($($s5:tt)*) ($($s6:tt)*) ($($s7:tt)*) ($($s8:tt)*) $($sample:tt)*) ) => {
        $($before)* <$($s0)* $($delim)? $($s1)* $($delim)? $($s2)* $($delim)? $($s3)* $($delim)? $($s4)* $($delim)? $($s5)* $($delim)? $($s6)* $($delim)? $($s7)* $($delim)? $($s8)*> $($after)*
    };
( (($($before:tt)*) ($($after:tt)*)) ($($delim:tt)?) ($d0:tt $d1:tt $d2:tt $d3:tt $d4:tt $d5:tt $d6:tt $d7:tt $d8:tt $d9:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) ($($s4:tt)*) ($($s5:tt)*) ($($s6:tt)*) ($($s7:tt)*) ($($s8:tt)*) ($($s9:tt)*) $($sample:tt)*) ) => {
        $($before)* <$($s0)* $($delim)? $($s1)* $($delim)? $($s2)* $($delim)? $($s3)* $($delim)? $($s4)* $($delim)? $($s5)* $($delim)? $($s6)* $($delim)? $($s7)* $($delim)? $($s8)* $($delim)? $($s9)*> $($after)*
    };
}

macro_rules! count {
    (@internal ($($order:literal)*) $one:tt) => {
        {(2 << (0 $(+ $order)*)) + $one }
    };
    (@internal ($($order:literal)*) $one:tt $two:tt) => {
        {2 << (0 $(+ $order)*) }
    };
    (@internal ($($order:literal)*) $one:tt $two:tt $three:tt $four:tt $($remainder:tt)*) => {
        count!(@internal ($($order)* 2) ($($segments)* ($one $two $three $four $five $six $seven $eight $nine $ten)) $($remainder)*)
    };
    (@internal ($($order:literal)*) $one:tt $two:tt $($remainder:tt)*) => {
        count!(@internal ($($order)* 1) ($($segments)* ($one $two $three $four $five $six $seven $eight $nine $ten)) $($remainder)*)
    };
    ($($remainder:tt)*) => {
        count!(@internal () $($remainder)*)
    };
}

macro_rules! to_struct {
    ($name:ident () $($types:tt)+) => {
        struct $name <$($types),+> {
            $($types : $types),+
        }
    };
    ($name:ident ($constraint:tt) $($types:tt)+) => {
        struct $name <$($types : $constraint),+> {
            $($types : $types),+
        }
    };
}

macro_rules! to_struct_body {
    ($name:ident () ($body_generator:ident) $($types:tt)+) => {
        struct $name <$($types),+> {
            $($types : $types),+
        }
    };
}

macro_rules! to_trait_impl {
    ($name:ident $struct_name:ident ($constraint:tt) ($body_generator:ident) $($types:tt)+) => {
        impl<$( $types : $constraint),+> $name for $struct_name <$($types),+> {

            $body_generator! { $($types)+ }

        }
    };
    ($name:ident $struct_name:ident () ($body_generator:ident) $($types:tt)+) => {
        impl<$($types),+> $name for $struct_name <$($types),+> {

            $body_generator! { $($types)+ }

        }
    };
}

macro_rules! sample_macro {
( ($name:ident) ($($args:tt)*) ($d0:tt) (($($s0:tt)*) $($sample:tt)*) ) => { $name ! ($($args)* $($s0)*) };
( ($name:ident) ($($args:tt)*) ($d0:tt $d1:tt) (($($s0:tt)*) ($($s1:tt)*) $($sample:tt)*) ) => { $name ! ($($args)* $($s0)* $($s1)*) };
( ($name:ident) ($($args:tt)*) ($d0:tt $d1:tt $d2:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) $($sample:tt)*) ) => { $name ! ($($args)* $($s0)* $($s1)* $($s2)*) };
( ($name:ident) ($($args:tt)*) ($d0:tt $d1:tt $d2:tt $d3:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) $($sample:tt)*) ) => { $name ! ($($args)* $($s0)* $($s1)* $($s2)* $($s3)*) };
( ($name:ident) ($($args:tt)*) ($d0:tt $d1:tt $d2:tt $d3:tt $d4:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) ($($s4:tt)*) $($sample:tt)*) ) => { $name ! ($($args)* $($s0)* $($s1)* $($s2)* $($s3)* $($s4)*) };
( ($name:ident) ($($args:tt)*) ($d0:tt $d1:tt $d2:tt $d3:tt $d4:tt $d5:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) ($($s4:tt)*) ($($s5:tt)*) $($sample:tt)*) ) => { $name ! ($($args)* $($s0)* $($s1)* $($s2)* $($s3)* $($s4)* $($s5)*) };
( ($name:ident) ($($args:tt)*) ($d0:tt $d1:tt $d2:tt $d3:tt $d4:tt $d5:tt $d6:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) ($($s4:tt)*) ($($s5:tt)*) ($($s6:tt)*) $($sample:tt)*) ) => { $name ! ($($args)* $($s0)* $($s1)* $($s2)* $($s3)* $($s4)* $($s5)* $($s6)*) };
( ($name:ident) ($($args:tt)*) ($d0:tt $d1:tt $d2:tt $d3:tt $d4:tt $d5:tt $d6:tt $d7:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) ($($s4:tt)*) ($($s5:tt)*) ($($s6:tt)*) ($($s7:tt)*) $($sample:tt)*) ) => { $name ! ($($args)* $($s0)* $($s1)* $($s2)* $($s3)* $($s4)* $($s5)* $($s6)* $($s7)*) };
( ($name:ident) ($($args:tt)*) ($d0:tt $d1:tt $d2:tt $d3:tt $d4:tt $d5:tt $d6:tt $d7:tt $d8:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) ($($s4:tt)*) ($($s5:tt)*) ($($s6:tt)*) ($($s7:tt)*) ($($s8:tt)*) $($sample:tt)*) ) => { $name ! ($($args)* $($s0)* $($s1)* $($s2)* $($s3)* $($s4)* $($s5)* $($s6)* $($s7)* $($s8)*) };
( ($name:ident) ($($args:tt)*) ($d0:tt $d1:tt $d2:tt $d3:tt $d4:tt $d5:tt $d6:tt $d7:tt $d8:tt $d9:tt) (($($s0:tt)*) ($($s1:tt)*) ($($s2:tt)*) ($($s3:tt)*) ($($s4:tt)*) ($($s5:tt)*) ($($s6:tt)*) ($($s7:tt)*) ($($s8:tt)*) ($($s9:tt)*) $($sample:tt)*) ) => { $name ! ($($args)* $($s0)* $($s1)* $($s2)* $($s3)* $($s4)* $($s5)* $($s6)* $($s7)* $($s8)* $($s9)*) };

}
pub(crate) use sample_macro;
pub(crate) use to_trait_impl;
pub(crate) use to_struct;
pub(crate) use delete;