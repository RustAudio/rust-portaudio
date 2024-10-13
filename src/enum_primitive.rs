// Copyright (c) 2015 Anders Kaseorg <andersk@mit.edu>

// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// “Software”), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:

// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

// Changes :
// Use only one path in enum_from_primitive,
// so our variants can be documented.

/// Helper macro for internal use by `enum_from_primitive!`.
macro_rules! enum_from_primitive_impl_ty {
    ($meth:ident, $ty:ty, $name:ident, $( $variant:ident )*) => {
        #[allow(non_upper_case_globals, unused)]
        fn $meth(n: $ty) -> $crate::Option<Self> {
            $( if n == $name::$variant as $ty {
                $crate::Option::Some($name::$variant)
            } else )* {
                $crate::Option::None
            }
        }
    };
}

/// Helper macro for internal use by `enum_from_primitive!`.
macro_rules! enum_from_primitive_impl {
    ($name:ident, $( $variant:ident )*) => {
        impl $crate::FromPrimitive for $name {
            enum_from_primitive_impl_ty! { from_i64, i64, $name, $( $variant )* }
            enum_from_primitive_impl_ty! { from_u64, u64, $name, $( $variant )* }
        }
    };
}

/// Wrap this macro around an `enum` declaration to get an
/// automatically generated implementation of `num::FromPrimitive`.
macro_rules! enum_from_primitive {
    (
        $( #[$enum_attr:meta] )*
            pub enum $name:ident {
                $( #[$variant_attr:meta]
                     $variant:ident = $discriminator:expr ),*,
            }
    ) => {
        $( #[$enum_attr] )*
            pub enum $name {
                $( #[$variant_attr]
                     $variant = $discriminator ),*,
            }
        enum_from_primitive_impl! { $name, $( $variant )+ }
    };

}
