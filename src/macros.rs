pub(crate) use paste::paste;

macro_rules! with_location_kind {
    {$(
        $(#[$($attrs:tt)+])*
        $vis:vis enum $name:ident {
            $($body:tt)*
        }
    )*} => {
        $crate::macros::paste! {
            $(
                $(#[$($attrs)+])*
                $vis struct $name {
                    pub location: $crate::file::Location,
                    pub kind: [<$name Kind>],
                }

                $(#[$($attrs)+])*
                $vis enum [<$name Kind>] {
                    $($body)*
                }
            )*
        }
    };
}
pub(crate) use with_location_kind;
