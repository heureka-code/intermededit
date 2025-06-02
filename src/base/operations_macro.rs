macro_rules! impl_operation_specific {
    ($module: ident, $(#[doc = $trait_doc:expr])* $vis: vis trait $trait: ident $(: ($($bounds: path),*))? {
        $(($($func: tt)*))+
    }) => {
        mod $module {
            pub trait SealedToInsertUpdateDelete {}
        }
        impl $module::SealedToInsertUpdateDelete for $crate::base::operations::Insert {}
        impl $module::SealedToInsertUpdateDelete for $crate::base::operations::Delete {}
        impl $module::SealedToInsertUpdateDelete for $crate::base::operations::Replace {}

        $(#[doc = $trait_doc])*
        $vis trait $trait
            where Self: $module::SealedToInsertUpdateDelete,
                $($(Self: $bounds),*)?
        {
            $($crate::base::operations::impl_operation_specific!{singsignature: $($func)*})*
        }
        impl $trait for $crate::base::operations::Insert {
            $($crate::base::operations::impl_operation_specific!{singinsert: $($func)*})*
        }
        impl $trait for $crate::base::operations::Delete {
            $($crate::base::operations::impl_operation_specific!{singdelete: $($func)*})*
        }
        impl $trait for $crate::base::operations::Replace {
            $($crate::base::operations::impl_operation_specific!{singreplace: $($func)*})*
        }
    };

    (singsignature: $(#[doc = $gen_doc: expr])* fn $name: ident$(<$($generics: tt)*>)?($($param: ident: $type: ty),*$(,)?) -> $return: ty,
            $(#[doc = $ins_doc: expr])* insert: $ins: block, $(#[doc = $del_doc: expr])* delete: $del: block, $(#[doc = $subs_doc: expr])* replace: $subs: block $(,)?) => {
        $(#[doc = $gen_doc])*
        fn $name$(<$($generics)*>)?($($param: $type),*) -> $return;
    };

    (singinsert: $(#[doc = $gen_doc: expr])* fn $name: ident$(<$($generics: tt)*>)?($($param: ident: $type: ty),*$(,)?) -> $return: ty,
            $(#[doc = $ins_doc: expr])* insert: $ins: block, $(#[doc = $del_doc: expr])* delete: $del: block, $(#[doc = $subs_doc: expr])* replace: $subs: block $(,)?) => {
        $crate::base::operations::impl_operation_specific!(docs: $(#[doc = $gen_doc])* : $(#[doc = $ins_doc])* :
            fn $name$(<$($generics)*>)?($($param: $type),*) -> $return { $ins }
        );
    };

    (singdelete: $(#[doc = $gen_doc: expr])* fn $name: ident$(<$($generics: tt)*>)?($($param: ident: $type: ty),*$(,)?) -> $return: ty,
            $(#[doc = $ins_doc: expr])* insert: $ins: block, $(#[doc = $del_doc: expr])* delete: $del: block, $(#[doc = $subs_doc: expr])* replace: $subs: block $(,)?) => {
        $crate::base::operations::impl_operation_specific!(docs: $(#[doc = $gen_doc])* : $(#[doc = $del_doc])* :
            fn $name$(<$($generics)*>)?($($param: $type),*) -> $return { $del }
        );
    };

    (singreplace: $(#[doc = $gen_doc: expr])* fn $name: ident$(<$($generics: tt)*>)?($($param: ident: $type: ty),*$(,)?) -> $return: ty,
            $(#[doc = $ins_doc: expr])* insert: $ins: block, $(#[doc = $del_doc: expr])* delete: $del: block, $(#[doc = $subs_doc: expr])* replace: $subs: block $(,)?) => {
        $crate::base::operations::impl_operation_specific!(docs: $(#[doc = $gen_doc])* : $(#[doc = $subs_doc])* :
            fn $name$(<$($generics)*>)?($($param: $type),*) -> $return { $subs }
        );
    };
    (docs: $(#[doc = $general: tt])+ : $(#[doc = $specific: tt])+ : $item: item) => {
        $(#[doc = $general])*
        #[doc = "\n"]
        $(#[doc = $specific])*
        $item
    };
    (docs: $(#[doc = $general: tt])* : $(#[doc = $specific: tt])* : $item: item) => {
        $(#[doc = $general])*
        $(#[doc = $specific])*
        $item
    };
}
pub(crate) use impl_operation_specific;
