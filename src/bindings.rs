// Generated by `wit-bindgen` 0.24.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod foo {
    #[allow(dead_code)]
    pub mod a {
        #[allow(dead_code, clippy::all)]
        pub mod a_child {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
        }
    }
    #[allow(dead_code)]
    pub mod utils {
        #[allow(dead_code, clippy::all)]
        pub mod utils {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct SomeResource {
                handle: _rt::Resource<SomeResource>,
            }

            impl SomeResource {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for SomeResource {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "foo:utils/utils")]
                        extern "C" {
                            #[link_name = "[resource-drop]some-resource"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            impl SomeResource {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new() -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "foo:utils/utils")]
                        extern "C" {
                            #[link_name = "[constructor]some-resource"]
                            fn wit_import() -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import() -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import();
                        SomeResource::from_handle(ret as u32)
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod foo {
        #[allow(dead_code)]
        pub mod a {
            #[allow(dead_code, clippy::all)]
            pub mod a {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                #[doc(hidden)]

                macro_rules! __export_foo_a_a_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _: () = {};
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_foo_a_a_cabi;
            }
        }
        #[allow(dead_code)]
        pub mod b {
            #[allow(dead_code, clippy::all)]
            pub mod b {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                #[doc(hidden)]

                macro_rules! __export_foo_b_b_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _: () = {};
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_foo_b_b_cabi;
            }
        }
    }
}
mod _rt {

    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        // NB: This would ideally be `u32` but it is not. The fact that this has
        // interior mutability is not exposed in the API of this type except for the
        // `take_handle` method which is supposed to in theory be private.
        //
        // This represents, almost all the time, a valid handle value. When it's
        // invalid it's stored as `u32::MAX`.
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }

    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }

    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }

        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }

        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }

    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource")
                .field("handle", &self.handle)
                .finish()
        }
    }

    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    // If this handle was "taken" then don't do anything in the
                    // destructor.
                    u32::MAX => {}

                    // ... but otherwise do actually destroy it with the imported
                    // component model intrinsic as defined through `T`.
                    other => T::drop(other),
                }
            }
        }
    }
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_example_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::foo::a::a::__export_foo_a_a_cabi!($ty with_types_in $($path_to_types_root)*::exports::foo::a::a);
  $($path_to_types_root)*::exports::foo::b::b::__export_foo_b_b_cabi!($ty with_types_in $($path_to_types_root)*::exports::foo::b::b);
  )
}
#[doc(inline)]
pub(crate) use __export_example_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.24.0:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 529] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x93\x03\x01A\x02\x01\
A\x0a\x01B\x04\x04\0\x0dsome-resource\x03\x01\x01i\0\x01@\0\0\x01\x04\0\x1a[cons\
tructor]some-resource\x01\x02\x03\x01\x0ffoo:utils/utils\x05\0\x02\x03\0\0\x0dso\
me-resource\x01B\x05\x02\x03\x02\x01\x01\x04\0\x0dsome-resource\x03\0\0\x01i\x01\
\x01q\x01\x03bar\x01\x02\0\x04\0\x0fa-child-variant\x03\0\x03\x03\x01\x0dfoo:a/a\
-child\x05\x02\x02\x03\0\x01\x0fa-child-variant\x01B\x07\x02\x03\x02\x01\x01\x04\
\0\x0dsome-resource\x03\0\0\x02\x03\x02\x01\x03\x04\0\x0fa-child-variant\x03\0\x02\
\x01i\x01\x01q\x02\x03bar\x01\x04\0\x03baz\x01\x03\0\x04\0\x09a-variant\x03\0\x05\
\x04\x01\x07foo:a/a\x05\x04\x01B\x05\x02\x03\x02\x01\x01\x04\0\x0dsome-resource\x03\
\0\0\x01i\x01\x01q\x01\x03bar\x01\x02\0\x04\0\x09b-variant\x03\0\x03\x04\x01\x07\
foo:b/b\x05\x05\x04\x01\x1ccomponent:transitive/example\x04\0\x0b\x0d\x01\0\x07e\
xample\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.202\
.0\x10wit-bindgen-rust\x060.24.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
