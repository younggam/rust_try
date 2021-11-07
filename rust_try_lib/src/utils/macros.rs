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
        #[$outer:meta]
        $vis_struct:vis struct $custom:ident {
            $($v:vis $i:ident: $t:ty),+ $(,)?;
            $($vis_field:vis $identifier:ident: $type:ty),* $(,)?
        }
    }=>{
        #[$outer]
        $vis_struct struct $custom{
            $($v $i: $t),+,
            $($vis_field $identifier: $crate::utils::LazyManual<$type>),*
        }
    }
}

///Same as lazy_struct
#[macro_export]
macro_rules! lazy_construct{
    {
        $self:ident {
            $($i:ident: $($e:expr)?),+ $(,)?;
            $($identifier:ident),* $(,)?
        }
    }=>{
        $self {
            $($i: $($e)?),+,
            $($identifier: $crate::utils::LazyManual::new()),*
        }
    }
}
