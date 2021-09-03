#![no_std]
#![forbid(unsafe_code)]
// reasonable clippy categories
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
// reasonable clippy::restriction lints
#![warn(
    clippy::clone_on_ref_ptr,
    clippy::float_arithmetic,
    clippy::float_cmp_const,
    clippy::integer_arithmetic,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::multiple_inherent_impl,
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::shadow_unrelated,
    clippy::string_add,
    clippy::unimplemented,
    clippy::use_debug
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
    rustdoc::missing_doc_code_examples,
    rustdoc::private_doc_tests,
    rustdoc::invalid_html_tags
)]

//! [![crates.io]](https://crates.io/crates/into_ext)
//! [![github]](https://github.com/steffahn/into_ext)
//! [![MIT / Apache 2.0 licensed]](https://github.com/steffahn/into_ext#License)
//! [![unsafe forbidden]](https://github.com/rust-secure-code/safety-dance/)
//!
//! Extension trait for the `Into` trait, offering a method `.into_::<T>()` to specify the target
//! type of conversion.
//!
//! [github]: https://img.shields.io/badge/github-steffahn/into__ext-yellowgreen.svg
//! [crates.io]: https://img.shields.io/crates/v/into_ext.svg
//! [MIT / Apache 2.0 licensed]: https://img.shields.io/crates/l/into_ext.svg
//! [docs.rs]: https://docs.rs/into_ext/badge.svg
//! [unsafe forbidden]: https://img.shields.io/badge/unsafe-forbidden-success.svg

/// Extension trait for the `Into` trait, offering a method `.into_::<T>()` to specify the target
/// type of conversion.
trait IntoExt<T0>: Into<T0> {
    /// Calling `foo.into()` using the standard librarie's `Into` trait can lead to ambiguities.
    /// Current workarounds to specify the target type `T` include using `T::from(foo)` or
    /// `Into::<T>::into(foo)`. Both are interfering badly with postfix method syntax, the former
    /// also doesn't support types that have a `S: Into<T>` but no `T: From<S>` implementation.
    ///
    /// ```
    /// let x: u32 = u32::MAX;
    /// // not let’s get x + 10 as an `u64`
    /// let y = x.into_::<u64> + 10;
    /// ```
    fn into_<T>(self) -> T {
        todo!()
    }
}
