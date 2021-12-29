#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        unsafe {
            let b: $base = std::mem::zeroed();
            (&b.$field as *const _ as isize) - (&b as *const _ as isize)
        }
    }};
}

///Currently for the Application.rs. for DRY
#[macro_export]
macro_rules! lazy_struct{
    {
        $(#[$outer:meta])?
        $v0:vis struct $i0:ident {
            $($v1:vis $i1:ident: $t1:ty,)*
            $(
                -$v2:vis $i2:ident: $t2:ty,
                $($v3:vis $i3:ident: $t3:ty,)*
            )+
        }
    }=>{
        $(#[$outer])?
        $v0 struct $i0{
            $($v1 $i1: $t1,)*
            $(
                $v2 $i2: $crate::utils::LazyManual<$t2>,
                $($v3 $i3: $t3,)*
            )+
        }
    }
}

///for using ? repition as kinda conditional expression.
#[macro_export]
macro_rules! macro_branch_expr{
    {$t:tt, $e:expr}=>{$t};
    {, $e:expr}=>{$e};
}

///Same as lazy_struct
#[macro_export]
macro_rules! lazy_construct{
    {
        $i0:ident {
            $(
                $i1:ident $(:$e1:expr)?,
            )+
        }
    }=>{
        $i0 {
            $(
                $i1: macro_branch_expr!{$($e1)?, $crate::utils::LazyManual::new()},
            )+
        }
    };
}

/**I can't understand why.
If reference become generics that for parameters of closure,
the closure wants argument live longer than closure itself.
When reference explicitly set as parameters of closure,
now the closure doesn't care how long argmuent lives.


I have figured out it is related with for<'r> Fn(&'a T).
But there's no way to identify type parameter with closures's arguments type parameter.


This macro kinda alternative for all tries to generalize closure*/
#[macro_export]
macro_rules! box_as {
    ($i:ident, $T:ty) => {
        Box::new($i) as Box<$T>
    };
}
