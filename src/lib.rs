#![no_std]
#![forbid(unsafe_code)]
// reasonable clippy categories
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
// reasonable clippy::restriction lints
#![warn(
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::default_numeric_fallback,
    clippy::else_if_without_else,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::exit,
    clippy::expect_used,
    clippy::filetype_is_file,
    clippy::float_arithmetic,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::modulo_arithmetic,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::pattern_type_mismatch,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_to_string,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm
)]
// reasonable rustc lints
#![warn(
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
// reasonable rustdoc lints
#![warn(
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    rustdoc::invalid_html_tags
)]

//! [![crates.io]](https://crates.io/crates/into_ext)
//! [![github]](https://github.com/steffahn/into_ext)
//! [![MIT / Apache 2.0 licensed]](https://github.com/steffahn/into_ext#License)
//! [![unsafe forbidden]](https://github.com/rust-secure-code/safety-dance/)
//!
//! [github]: https://img.shields.io/badge/github-steffahn/into__ext-yellowgreen.svg
//! [crates.io]: https://img.shields.io/crates/v/into_ext.svg
//! [MIT / Apache 2.0 licensed]: https://img.shields.io/crates/l/into_ext.svg
//! [docs.rs]: https://docs.rs/into_ext/badge.svg
//! [unsafe forbidden]: https://img.shields.io/badge/unsafe-forbidden-success.svg
//!
//! This crate offers an extension trait [`IntoExt`] for the [`Into`] trait from the standard
//! library, offering a method [`.into_::<T>()`][into_] to specify the target type of conversion.
//!
//! See [the documentation of `IntoExt::into_`][into_] for more details.
//!
//! [into_]: IntoExt::into_

/// Extension trait for the [`Into`] trait, offering a method `.into_::<T>()` to specify the target
/// type of conversion.
pub trait IntoExt<T_>: Into<T_> {
    /// Calling `foo.into()` using the standard library's [`Into`] trait can lead to ambiguities.
    /// Some current workarounds to specify the target type `T` are to use `T::from(foo)`, or
    /// `Into::<T>::into(foo)`. Both of these alternatives are interfering badly with postfix method
    /// syntax; the former also doesn't support types that have a `S: Into<T>` but no `T: From<S>`
    /// implementation.
    ///
    /// With `IntoExt`, you can simply write `foo.into_::<T>()`.
    ///
    /// The underscore at the end of the method name is to avoid collision with `Into::into`,
    /// and to indicate that the method is followed by additional information
    /// (i.e. a type parameter).
    ///
    /// # Example
    /// ```
    /// use into_ext::IntoExt;
    ///
    /// // here’s a big `u32` value, called ‘x’
    /// let x: u32 = u32::MAX;
    ///
    /// // now, let’s get x + 10 as an `u64` (without using the `as` operator)
    /// let y = x.into_::<u64>() + 10;
    /// ```
    /// whereas, e.g. the following wouldn't have worked
    /// ```compile_fail
    /// let x: u32 = u32::MAX;
    /// let y: u32 = x.into() + 10_u32;
    /// ```
    fn into_<T>(self) -> T
    where
        T: TypeIsEqual<To = T_>,
    {
        #[allow(clippy::missing_docs_in_private_items, clippy::missing_const_for_fn)]
        fn helper<T>(val: <T as TypeIsEqual>::To) -> T {
            val
        }
        helper(self.into())
    }
}

impl<S, T_> IntoExt<T_> for S where S: Into<T_> {}

/// Helper trait for type equality, necessary to make [`IntoExt::into_`] work.
///
/// Generically implemented so that `T: TypeIsEqual<To = T>` holds for all types.
///
/// # Example
///
/// If you _have_ an `S: TypeIsEqual<To = T>` bound, this trait can be used by using a generic
/// function with arguments or return values of type `S` and `<S as TypeIsEqual>::To`.
///
/// ```
/// # use into_ext::TypeIsEqual;
/// fn convert_one_way<S, T>(s: S) -> T
/// where
///     S: TypeIsEqual<To = T>,
/// {
///     fn helper<S>(s: S) -> <S as TypeIsEqual>::To {
///         s
///     }
///     helper(s)
/// }
///
/// fn convert_other_way<S, T>(t: T) -> S
/// where
///     S: TypeIsEqual<To = T>,
/// {
///     fn helper<S>(t: <S as TypeIsEqual>::To) -> S {
///         t
///     }
///     helper(t)
/// }
/// ```
/// The same works if the corresponding types are _part_ of a larger type, e.g. you can convert
/// `&mut [Option<S>]` to `&mut [Option<T>]` and things like that.
pub trait TypeIsEqual {
    /// Two types being equal -- e.g. `S == T` -- is written as `S: TypeIsEqual<To = T>` with
    /// this trait.
    type To: ?Sized;
}
impl<T: ?Sized> TypeIsEqual for T {
    type To = Self;
}
