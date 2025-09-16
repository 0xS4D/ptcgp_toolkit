#![allow(unused_qualifications)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(clippy::useless_transmute)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use std::os::raw::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[repr(C)]
    #[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct __BindgenBitfieldUnit<Storage> {
        storage: Storage,
    }
    impl<Storage> __BindgenBitfieldUnit<Storage> {
        #[inline]
        pub const fn new(storage: Storage) -> Self {
            Self { storage }
        }
    }
    impl<Storage> __BindgenBitfieldUnit<Storage>
    where
        Storage: AsRef<[u8]> + AsMut<[u8]>,
    {
        #[inline]
        fn extract_bit(byte: u8, index: usize) -> bool {
            let bit_index = if cfg!(target_endian = "big") {
                7 - (index % 8)
            } else {
                index % 8
            };
            let mask = 1 << bit_index;
            byte & mask == mask
        }
        #[inline]
        pub fn get_bit(&self, index: usize) -> bool {
            debug_assert!(index / 8 < self.storage.as_ref().len());
            let byte_index = index / 8;
            let byte = self.storage.as_ref()[byte_index];
            Self::extract_bit(byte, index)
        }
        #[inline]
        pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
            debug_assert!(index / 8 < core::mem::size_of::<Storage>());
            let byte_index = index / 8;
            let byte = unsafe {
                *(core::ptr::addr_of!((*this).storage) as *const u8).offset(byte_index as isize)
            };
            Self::extract_bit(byte, index)
        }
        #[inline]
        fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
            let bit_index = if cfg!(target_endian = "big") {
                7 - (index % 8)
            } else {
                index % 8
            };
            let mask = 1 << bit_index;
            if val {
                byte | mask
            } else {
                byte & !mask
            }
        }
        #[inline]
        pub fn set_bit(&mut self, index: usize, val: bool) {
            debug_assert!(index / 8 < self.storage.as_ref().len());
            let byte_index = index / 8;
            let byte = &mut self.storage.as_mut()[byte_index];
            *byte = Self::change_bit(*byte, index, val);
        }
        #[inline]
        pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
            debug_assert!(index / 8 < core::mem::size_of::<Storage>());
            let byte_index = index / 8;
            let byte = unsafe {
                (core::ptr::addr_of_mut!((*this).storage) as *mut u8).offset(byte_index as isize)
            };
            unsafe { *byte = Self::change_bit(*byte, index, val) };
        }
        #[inline]
        pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
            debug_assert!(bit_width <= 64);
            debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
            debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
            let mut val = 0;
            for i in 0..(bit_width as usize) {
                if self.get_bit(i + bit_offset) {
                    let index = if cfg!(target_endian = "big") {
                        bit_width as usize - 1 - i
                    } else {
                        i
                    };
                    val |= 1 << index;
                }
            }
            val
        }
        #[inline]
        pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
            debug_assert!(bit_width <= 64);
            debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
            debug_assert!(
                (bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>()
            );
            let mut val = 0;
            for i in 0..(bit_width as usize) {
                if unsafe { Self::raw_get_bit(this, i + bit_offset) } {
                    let index = if cfg!(target_endian = "big") {
                        bit_width as usize - 1 - i
                    } else {
                        i
                    };
                    val |= 1 << index;
                }
            }
            val
        }
        #[inline]
        pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
            debug_assert!(bit_width <= 64);
            debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
            debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
            for i in 0..(bit_width as usize) {
                let mask = 1 << i;
                let val_bit_is_set = val & mask == mask;
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                self.set_bit(index + bit_offset, val_bit_is_set);
            }
        }
        #[inline]
        pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
            debug_assert!(bit_width <= 64);
            debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
            debug_assert!(
                (bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>()
            );
            for i in 0..(bit_width as usize) {
                let mask = 1 << i;
                let val_bit_is_set = val & mask == mask;
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                unsafe { Self::raw_set_bit(this, index + bit_offset, val_bit_is_set) };
            }
        }
    }
    #[allow(unused_imports)]
    use self::super::root;
    pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
    pub const _SAL_VERSION: u32 = 20;
    pub const __SAL_H_VERSION: u32 = 180000000;
    pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
    pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
    pub const _CRT_PACKING: u32 = 8;
    pub const _HAS_EXCEPTIONS: u32 = 1;
    pub const NULL: u32 = 0;
    pub const _HAS_CXX17: u32 = 0;
    pub const _HAS_CXX20: u32 = 0;
    pub const _HAS_CXX23: u32 = 0;
    pub const _HAS_CXX26: u32 = 0;
    pub const _HAS_NODISCARD: u32 = 1;
    pub const WCHAR_MIN: u32 = 0;
    pub const WCHAR_MAX: u32 = 65535;
    pub const WINT_MIN: u32 = 0;
    pub const WINT_MAX: u32 = 65535;
    pub const _ARM_WINAPI_PARTITION_DESKTOP_SDK_AVAILABLE: u32 = 1;
    pub const _CRT_BUILD_DESKTOP_APP: u32 = 1;
    pub const _ARGMAX: u32 = 100;
    pub const _CRT_INT_MAX: u32 = 2147483647;
    pub const _CRT_FUNCTIONS_REQUIRED: u32 = 1;
    pub const _CRT_HAS_CXX17: u32 = 0;
    pub const _CRT_HAS_C11: u32 = 0;
    pub const _CRT_INTERNAL_NONSTDC_NAMES: u32 = 1;
    pub const __STDC_SECURE_LIB__: u32 = 200411;
    pub const __GOT_SECURE_LIB__: u32 = 200411;
    pub const __STDC_WANT_SECURE_LIB__: u32 = 1;
    pub const _SECURECRT_FILL_BUFFER_PATTERN: u32 = 254;
    pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES: u32 = 0;
    pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_COUNT: u32 = 0;
    pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES: u32 = 1;
    pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_MEMORY: u32 = 0;
    pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES_MEMORY: u32 = 0;
    pub type va_list = *mut ::std::os::raw::c_char;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct __vcrt_va_list_is_reference {
        pub _address: u8,
    }
    pub const __the_value: __vcrt_va_list_is_reference__bindgen_ty_1 = false;
    pub type __vcrt_va_list_is_reference__bindgen_ty_1 = bool;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct __vcrt_assert_va_start_is_not_reference {
        pub _address: u8,
    }
    pub type __vcrt_bool = bool;
    unsafe extern "C" {
        pub static mut __security_cookie: usize;
    }
    pub type int_least8_t = ::std::os::raw::c_schar;
    pub type int_least16_t = ::std::os::raw::c_short;
    pub type int_least32_t = ::std::os::raw::c_int;
    pub type int_least64_t = ::std::os::raw::c_longlong;
    pub type uint_least8_t = ::std::os::raw::c_uchar;
    pub type uint_least16_t = ::std::os::raw::c_ushort;
    pub type uint_least32_t = ::std::os::raw::c_uint;
    pub type uint_least64_t = ::std::os::raw::c_ulonglong;
    pub type int_fast8_t = ::std::os::raw::c_schar;
    pub type int_fast16_t = ::std::os::raw::c_int;
    pub type int_fast32_t = ::std::os::raw::c_int;
    pub type int_fast64_t = ::std::os::raw::c_longlong;
    pub type uint_fast8_t = ::std::os::raw::c_uchar;
    pub type uint_fast16_t = ::std::os::raw::c_uint;
    pub type uint_fast32_t = ::std::os::raw::c_uint;
    pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
    pub type intmax_t = ::std::os::raw::c_longlong;
    pub type uintmax_t = ::std::os::raw::c_ulonglong;
    pub type __crt_bool = bool;
    pub type errno_t = ::std::os::raw::c_int;
    pub type wint_t = ::std::os::raw::c_ushort;
    pub type wctype_t = ::std::os::raw::c_ushort;
    pub type __time32_t = ::std::os::raw::c_long;
    pub type __time64_t = ::std::os::raw::c_longlong;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __crt_locale_data_public {
        pub _locale_pctype: *const ::std::os::raw::c_ushort,
        pub _locale_mb_cur_max: ::std::os::raw::c_int,
        pub _locale_lc_codepage: ::std::os::raw::c_uint,
    }
    impl Default for __crt_locale_data_public {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __crt_locale_pointers {
        pub locinfo: *mut root::__crt_locale_data,
        pub mbcinfo: *mut root::__crt_multibyte_data,
    }
    impl Default for __crt_locale_pointers {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    pub type _locale_t = *mut root::__crt_locale_pointers;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct _Mbstatet {
        pub _Wchar: ::std::os::raw::c_ulong,
        pub _Byte: ::std::os::raw::c_ushort,
        pub _State: ::std::os::raw::c_ushort,
    }
    pub type mbstate_t = root::_Mbstatet;
    pub type time_t = root::__time64_t;
    pub type rsize_t = usize;
    pub mod std {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub type nullptr_t = *const ::std::os::raw::c_void;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppProfiler {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppCustomAttrInfo {
        _unused: [u8; 0],
    }
    pub const IL2CPP_PROFILE_NONE: Il2CppProfileFlags = 0;
    pub const IL2CPP_PROFILE_APPDOMAIN_EVENTS: Il2CppProfileFlags = 1;
    pub const IL2CPP_PROFILE_ASSEMBLY_EVENTS: Il2CppProfileFlags = 2;
    pub const IL2CPP_PROFILE_MODULE_EVENTS: Il2CppProfileFlags = 4;
    pub const IL2CPP_PROFILE_CLASS_EVENTS: Il2CppProfileFlags = 8;
    pub const IL2CPP_PROFILE_JIT_COMPILATION: Il2CppProfileFlags = 16;
    pub const IL2CPP_PROFILE_INLINING: Il2CppProfileFlags = 32;
    pub const IL2CPP_PROFILE_EXCEPTIONS: Il2CppProfileFlags = 64;
    pub const IL2CPP_PROFILE_ALLOCATIONS: Il2CppProfileFlags = 128;
    pub const IL2CPP_PROFILE_GC: Il2CppProfileFlags = 256;
    pub const IL2CPP_PROFILE_THREADS: Il2CppProfileFlags = 512;
    pub const IL2CPP_PROFILE_REMOTING: Il2CppProfileFlags = 1024;
    pub const IL2CPP_PROFILE_TRANSITIONS: Il2CppProfileFlags = 2048;
    pub const IL2CPP_PROFILE_ENTER_LEAVE: Il2CppProfileFlags = 4096;
    pub const IL2CPP_PROFILE_COVERAGE: Il2CppProfileFlags = 8192;
    pub const IL2CPP_PROFILE_INS_COVERAGE: Il2CppProfileFlags = 16384;
    pub const IL2CPP_PROFILE_STATISTICAL: Il2CppProfileFlags = 32768;
    pub const IL2CPP_PROFILE_METHOD_EVENTS: Il2CppProfileFlags = 65536;
    pub const IL2CPP_PROFILE_MONITOR_EVENTS: Il2CppProfileFlags = 131072;
    pub const IL2CPP_PROFILE_IOMAP_EVENTS: Il2CppProfileFlags = 262144;
    pub const IL2CPP_PROFILE_GC_MOVES: Il2CppProfileFlags = 524288;
    pub const IL2CPP_PROFILE_FILEIO: Il2CppProfileFlags = 1048576;
    pub type Il2CppProfileFlags = ::std::os::raw::c_int;
    pub const IL2CPP_PROFILE_FILEIO_WRITE: Il2CppProfileFileIOKind = 0;
    pub const IL2CPP_PROFILE_FILEIO_READ: Il2CppProfileFileIOKind = 1;
    pub type Il2CppProfileFileIOKind = ::std::os::raw::c_int;
    pub const IL2CPP_GC_EVENT_START: Il2CppGCEvent = 0;
    pub const IL2CPP_GC_EVENT_MARK_START: Il2CppGCEvent = 1;
    pub const IL2CPP_GC_EVENT_MARK_END: Il2CppGCEvent = 2;
    pub const IL2CPP_GC_EVENT_RECLAIM_START: Il2CppGCEvent = 3;
    pub const IL2CPP_GC_EVENT_RECLAIM_END: Il2CppGCEvent = 4;
    pub const IL2CPP_GC_EVENT_END: Il2CppGCEvent = 5;
    pub const IL2CPP_GC_EVENT_PRE_STOP_WORLD: Il2CppGCEvent = 6;
    pub const IL2CPP_GC_EVENT_POST_STOP_WORLD: Il2CppGCEvent = 7;
    pub const IL2CPP_GC_EVENT_PRE_START_WORLD: Il2CppGCEvent = 8;
    pub const IL2CPP_GC_EVENT_POST_START_WORLD: Il2CppGCEvent = 9;
    pub type Il2CppGCEvent = ::std::os::raw::c_int;
    pub const IL2CPP_GC_MODE_DISABLED: Il2CppGCMode = 0;
    pub const IL2CPP_GC_MODE_ENABLED: Il2CppGCMode = 1;
    pub const IL2CPP_GC_MODE_MANUAL: Il2CppGCMode = 2;
    pub type Il2CppGCMode = ::std::os::raw::c_int;
    pub const IL2CPP_STAT_NEW_OBJECT_COUNT: Il2CppStat = 0;
    pub const IL2CPP_STAT_INITIALIZED_CLASS_COUNT: Il2CppStat = 1;
    pub const IL2CPP_STAT_METHOD_COUNT: Il2CppStat = 2;
    pub const IL2CPP_STAT_CLASS_STATIC_DATA_SIZE: Il2CppStat = 3;
    pub const IL2CPP_STAT_GENERIC_INSTANCE_COUNT: Il2CppStat = 4;
    pub const IL2CPP_STAT_GENERIC_CLASS_COUNT: Il2CppStat = 5;
    pub const IL2CPP_STAT_INFLATED_METHOD_COUNT: Il2CppStat = 6;
    pub const IL2CPP_STAT_INFLATED_TYPE_COUNT: Il2CppStat = 7;
    pub type Il2CppStat = ::std::os::raw::c_int;
    pub const IL2CPP_UNHANDLED_POLICY_LEGACY: Il2CppRuntimeUnhandledExceptionPolicy = 0;
    pub const IL2CPP_UNHANDLED_POLICY_CURRENT: Il2CppRuntimeUnhandledExceptionPolicy = 1;
    pub type Il2CppRuntimeUnhandledExceptionPolicy = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppStackFrameInfo {
        pub method: *const root::MethodInfo,
        pub raw_ip: usize,
        pub sourceCodeLineNumber: ::std::os::raw::c_int,
        pub ilOffset: ::std::os::raw::c_int,
        pub filePath: *const ::std::os::raw::c_char,
    }
    impl Default for Il2CppStackFrameInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    pub type Il2CppMethodPointer = ::std::option::Option<unsafe extern "C" fn()>;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppMethodDebugInfo {
        pub methodPointer: root::Il2CppMethodPointer,
        pub code_size: i32,
        pub file: *const ::std::os::raw::c_char,
    }
    impl Default for Il2CppMethodDebugInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppMemoryCallbacks {
        pub malloc_func:
            ::std::option::Option<unsafe extern "C" fn(size: usize) -> *mut ::std::os::raw::c_void>,
        pub aligned_malloc_func: ::std::option::Option<
            unsafe extern "C" fn(size: usize, alignment: usize) -> *mut ::std::os::raw::c_void,
        >,
        pub free_func:
            ::std::option::Option<unsafe extern "C" fn(ptr: *mut ::std::os::raw::c_void)>,
        pub aligned_free_func:
            ::std::option::Option<unsafe extern "C" fn(ptr: *mut ::std::os::raw::c_void)>,
        pub calloc_func: ::std::option::Option<
            unsafe extern "C" fn(nmemb: usize, size: usize) -> *mut ::std::os::raw::c_void,
        >,
        pub realloc_func: ::std::option::Option<
            unsafe extern "C" fn(
                ptr: *mut ::std::os::raw::c_void,
                size: usize,
            ) -> *mut ::std::os::raw::c_void,
        >,
        pub aligned_realloc_func: ::std::option::Option<
            unsafe extern "C" fn(
                ptr: *mut ::std::os::raw::c_void,
                size: usize,
                alignment: usize,
            ) -> *mut ::std::os::raw::c_void,
        >,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppDebuggerTransport {
        pub name: *const ::std::os::raw::c_char,
        pub connect:
            ::std::option::Option<unsafe extern "C" fn(address: *const ::std::os::raw::c_char)>,
        pub wait_for_attach: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
        pub close1: ::std::option::Option<unsafe extern "C" fn()>,
        pub close2: ::std::option::Option<unsafe extern "C" fn()>,
        pub send: ::std::option::Option<
            unsafe extern "C" fn(
                buf: *mut ::std::os::raw::c_void,
                len: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pub recv: ::std::option::Option<
            unsafe extern "C" fn(
                buf: *mut ::std::os::raw::c_void,
                len: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
    }
    impl Default for Il2CppDebuggerTransport {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    pub type Il2CppChar = u16;
    pub type Il2CppNativeChar = ::std::os::raw::c_char;
    pub type il2cpp_register_object_callback = ::std::option::Option<
        unsafe extern "C" fn(
            arr: *mut *mut root::Il2CppObject,
            size: ::std::os::raw::c_int,
            userdata: *mut ::std::os::raw::c_void,
        ),
    >;
    pub type il2cpp_liveness_reallocate_callback = ::std::option::Option<
        unsafe extern "C" fn(
            ptr: *mut ::std::os::raw::c_void,
            size: usize,
            userdata: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >;
    pub type Il2CppFrameWalkFunc = ::std::option::Option<
        unsafe extern "C" fn(
            info: *const root::Il2CppStackFrameInfo,
            user_data: *mut ::std::os::raw::c_void,
        ),
    >;
    pub type Il2CppProfileFunc =
        ::std::option::Option<unsafe extern "C" fn(prof: *mut root::Il2CppProfiler)>;
    pub type Il2CppProfileMethodFunc = ::std::option::Option<
        unsafe extern "C" fn(prof: *mut root::Il2CppProfiler, method: *const root::MethodInfo),
    >;
    pub type Il2CppProfileAllocFunc = ::std::option::Option<
        unsafe extern "C" fn(
            prof: *mut root::Il2CppProfiler,
            obj: *mut root::Il2CppObject,
            klass: *mut root::Il2CppClass,
        ),
    >;
    pub type Il2CppProfileGCFunc = ::std::option::Option<
        unsafe extern "C" fn(
            prof: *mut root::Il2CppProfiler,
            event: root::Il2CppGCEvent,
            generation: ::std::os::raw::c_int,
        ),
    >;
    pub type Il2CppProfileGCResizeFunc =
        ::std::option::Option<unsafe extern "C" fn(prof: *mut root::Il2CppProfiler, new_size: i64)>;
    pub type Il2CppProfileFileIOFunc = ::std::option::Option<
        unsafe extern "C" fn(
            prof: *mut root::Il2CppProfiler,
            kind: root::Il2CppProfileFileIOKind,
            count: ::std::os::raw::c_int,
        ),
    >;
    pub type Il2CppProfileThreadFunc = ::std::option::Option<
        unsafe extern "C" fn(prof: *mut root::Il2CppProfiler, tid: ::std::os::raw::c_ulong),
    >;
    pub type Il2CppSetFindPlugInCallback = ::std::option::Option<
        unsafe extern "C" fn(arg1: *const root::Il2CppNativeChar) -> *const root::Il2CppNativeChar,
    >;
    pub type Il2CppLogCallback =
        ::std::option::Option<unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char)>;
    pub type Il2CppBacktraceFunc = ::std::option::Option<
        unsafe extern "C" fn(buffer: *mut root::Il2CppMethodPointer, maxSize: usize) -> usize,
    >;
    pub type il2cpp_array_size_t = usize;
    pub type Il2CppAndroidUpStateFunc = ::std::option::Option<
        unsafe extern "C" fn(ifName: *const ::std::os::raw::c_char, is_up: *mut u8) -> u8,
    >;
    pub type SynchronizationContextCallback =
        ::std::option::Option<unsafe extern "C" fn(arg: isize)>;
    pub type CultureInfoChangedCallback =
        ::std::option::Option<unsafe extern "C" fn(arg: *const root::Il2CppChar)>;
    pub type Il2CppMethodSlot = u16;
    pub const kInvalidIl2CppMethodSlot: u16 = 65535;
    pub const ipv6AddressSize: ::std::os::raw::c_int = 16;
    pub type il2cpp_hresult_t = i32;
    pub const IL2CPP_TOKEN_MODULE: Il2CppTokenType = 0;
    pub const IL2CPP_TOKEN_TYPE_REF: Il2CppTokenType = 16777216;
    pub const IL2CPP_TOKEN_TYPE_DEF: Il2CppTokenType = 33554432;
    pub const IL2CPP_TOKEN_FIELD_DEF: Il2CppTokenType = 67108864;
    pub const IL2CPP_TOKEN_METHOD_DEF: Il2CppTokenType = 100663296;
    pub const IL2CPP_TOKEN_PARAM_DEF: Il2CppTokenType = 134217728;
    pub const IL2CPP_TOKEN_INTERFACE_IMPL: Il2CppTokenType = 150994944;
    pub const IL2CPP_TOKEN_MEMBER_REF: Il2CppTokenType = 167772160;
    pub const IL2CPP_TOKEN_CUSTOM_ATTRIBUTE: Il2CppTokenType = 201326592;
    pub const IL2CPP_TOKEN_PERMISSION: Il2CppTokenType = 234881024;
    pub const IL2CPP_TOKEN_SIGNATURE: Il2CppTokenType = 285212672;
    pub const IL2CPP_TOKEN_EVENT: Il2CppTokenType = 335544320;
    pub const IL2CPP_TOKEN_PROPERTY: Il2CppTokenType = 385875968;
    pub const IL2CPP_TOKEN_MODULE_REF: Il2CppTokenType = 436207616;
    pub const IL2CPP_TOKEN_TYPE_SPEC: Il2CppTokenType = 452984832;
    pub const IL2CPP_TOKEN_ASSEMBLY: Il2CppTokenType = 536870912;
    pub const IL2CPP_TOKEN_ASSEMBLY_REF: Il2CppTokenType = 587202560;
    pub const IL2CPP_TOKEN_FILE: Il2CppTokenType = 637534208;
    pub const IL2CPP_TOKEN_EXPORTED_TYPE: Il2CppTokenType = 654311424;
    pub const IL2CPP_TOKEN_MANIFEST_RESOURCE: Il2CppTokenType = 671088640;
    pub const IL2CPP_TOKEN_GENERIC_PARAM: Il2CppTokenType = 704643072;
    pub const IL2CPP_TOKEN_METHOD_SPEC: Il2CppTokenType = 721420288;
    pub type Il2CppTokenType = ::std::os::raw::c_int;
    pub type TypeIndex = i32;
    pub type TypeDefinitionIndex = i32;
    pub type FieldIndex = i32;
    pub type DefaultValueIndex = i32;
    pub type DefaultValueDataIndex = i32;
    pub type CustomAttributeIndex = i32;
    pub type ParameterIndex = i32;
    pub type MethodIndex = i32;
    pub type GenericMethodIndex = i32;
    pub type PropertyIndex = i32;
    pub type EventIndex = i32;
    pub type GenericContainerIndex = i32;
    pub type GenericParameterIndex = i32;
    pub type GenericParameterConstraintIndex = i16;
    pub type NestedTypeIndex = i32;
    pub type InterfacesIndex = i32;
    pub type VTableIndex = i32;
    pub type RGCTXIndex = i32;
    pub type StringIndex = i32;
    pub type StringLiteralIndex = i32;
    pub type GenericInstIndex = i32;
    pub type ImageIndex = i32;
    pub type AssemblyIndex = i32;
    pub type InteropDataIndex = i32;
    pub type TypeFieldIndex = i32;
    pub type TypeMethodIndex = i32;
    pub type MethodParameterIndex = i32;
    pub type TypePropertyIndex = i32;
    pub type TypeEventIndex = i32;
    pub type TypeInterfaceIndex = i32;
    pub type TypeNestedTypeIndex = i32;
    pub type TypeInterfaceOffsetIndex = i32;
    pub type GenericContainerParameterIndex = i32;
    pub type AssemblyTypeIndex = i32;
    pub type AssemblyExportedTypeIndex = i32;
    pub const kTypeIndexInvalid: root::TypeIndex = -1;
    pub const kTypeDefinitionIndexInvalid: root::TypeDefinitionIndex = -1;
    pub const kDefaultValueIndexNull: root::DefaultValueDataIndex = -1;
    pub const kCustomAttributeIndexInvalid: root::CustomAttributeIndex = -1;
    pub const kEventIndexInvalid: root::EventIndex = -1;
    pub const kFieldIndexInvalid: root::FieldIndex = -1;
    pub const kMethodIndexInvalid: root::MethodIndex = -1;
    pub const kPropertyIndexInvalid: root::PropertyIndex = -1;
    pub const kGenericContainerIndexInvalid: root::GenericContainerIndex = -1;
    pub const kGenericParameterIndexInvalid: root::GenericParameterIndex = -1;
    pub const kRGCTXIndexInvalid: root::RGCTXIndex = -1;
    pub const kStringLiteralIndexInvalid: root::StringLiteralIndex = -1;
    pub const kInteropDataIndexInvalid: root::InteropDataIndex = -1;
    pub const kPublicKeyByteLength: ::std::os::raw::c_int = 8;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppMethodSpec {
        pub methodDefinitionIndex: root::MethodIndex,
        pub classIndexIndex: root::GenericInstIndex,
        pub methodIndexIndex: root::GenericInstIndex,
    }
    pub const IL2CPP_RGCTX_DATA_INVALID: Il2CppRGCTXDataType = 0;
    pub const IL2CPP_RGCTX_DATA_TYPE: Il2CppRGCTXDataType = 1;
    pub const IL2CPP_RGCTX_DATA_CLASS: Il2CppRGCTXDataType = 2;
    pub const IL2CPP_RGCTX_DATA_METHOD: Il2CppRGCTXDataType = 3;
    pub const IL2CPP_RGCTX_DATA_ARRAY: Il2CppRGCTXDataType = 4;
    pub const IL2CPP_RGCTX_DATA_CONSTRAINED: Il2CppRGCTXDataType = 5;
    pub type Il2CppRGCTXDataType = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Il2CppRGCTXDefinitionData {
        pub rgctxDataDummy: i32,
        pub __methodIndex: root::MethodIndex,
        pub __typeIndex: root::TypeIndex,
    }
    impl Default for Il2CppRGCTXDefinitionData {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppRGCTXDefinitionData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppRGCTXDefinitionData {{ union }}")
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppRGCTXConstrainedData {
        pub __typeIndex: root::TypeIndex,
        pub __encodedMethodIndex: u32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppRGCTXDefinition {
        pub type_: root::Il2CppRGCTXDataType,
        pub data: *const ::std::os::raw::c_void,
    }
    impl Default for Il2CppRGCTXDefinition {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppGenericMethodIndices {
        pub methodIndex: root::MethodIndex,
        pub invokerIndex: root::MethodIndex,
        pub adjustorThunkIndex: root::MethodIndex,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppGenericMethodFunctionsDefinitions {
        pub genericMethodIndex: root::GenericMethodIndex,
        pub indices: root::Il2CppGenericMethodIndices,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ___Il2CppMetadataImageHandle {
        _unused: [u8; 0],
    }
    pub type Il2CppMetadataImageHandle = *const root::___Il2CppMetadataImageHandle;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ___Il2CppMetadataCustomAttributeHandle {
        _unused: [u8; 0],
    }
    pub type Il2CppMetadataCustomAttributeHandle =
        *const root::___Il2CppMetadataCustomAttributeHandle;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ___Il2CppMetadataTypeHandle {
        _unused: [u8; 0],
    }
    pub type Il2CppMetadataTypeHandle = *const root::___Il2CppMetadataTypeHandle;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ___Il2CppMetadataMethodHandle {
        _unused: [u8; 0],
    }
    pub type Il2CppMetadataMethodDefinitionHandle = *const root::___Il2CppMetadataMethodHandle;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ___Il2CppMetadataGenericContainerHandle {
        _unused: [u8; 0],
    }
    pub type Il2CppMetadataGenericContainerHandle =
        *const root::___Il2CppMetadataGenericContainerHandle;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ___Il2CppMetadataGenericParameterHandle {
        _unused: [u8; 0],
    }
    pub type Il2CppMetadataGenericParameterHandle =
        *const root::___Il2CppMetadataGenericParameterHandle;
    pub type EncodedMethodIndex = u32;
    pub const kIl2CppMetadataUsageInvalid: Il2CppMetadataUsage = 0;
    pub const kIl2CppMetadataUsageTypeInfo: Il2CppMetadataUsage = 1;
    pub const kIl2CppMetadataUsageIl2CppType: Il2CppMetadataUsage = 2;
    pub const kIl2CppMetadataUsageMethodDef: Il2CppMetadataUsage = 3;
    pub const kIl2CppMetadataUsageFieldInfo: Il2CppMetadataUsage = 4;
    pub const kIl2CppMetadataUsageStringLiteral: Il2CppMetadataUsage = 5;
    pub const kIl2CppMetadataUsageMethodRef: Il2CppMetadataUsage = 6;
    pub const kIl2CppMetadataUsageFieldRva: Il2CppMetadataUsage = 7;
    pub type Il2CppMetadataUsage = ::std::os::raw::c_int;
    pub const kIl2CppInvalidMetadataUsageNoData: Il2CppInvalidMetadataUsageToken = 0;
    pub const kIl2CppInvalidMetadataUsageAmbiguousMethod: Il2CppInvalidMetadataUsageToken = 1;
    pub type Il2CppInvalidMetadataUsageToken = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppInterfaceOffsetPair {
        pub interfaceTypeIndex: root::TypeIndex,
        pub offset: i32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppTypeDefinition {
        pub nameIndex: root::StringIndex,
        pub namespaceIndex: root::StringIndex,
        pub byvalTypeIndex: root::TypeIndex,
        pub declaringTypeIndex: root::TypeIndex,
        pub parentIndex: root::TypeIndex,
        pub elementTypeIndex: root::TypeIndex,
        pub genericContainerIndex: root::GenericContainerIndex,
        pub flags: u32,
        pub fieldStart: root::FieldIndex,
        pub methodStart: root::MethodIndex,
        pub eventStart: root::EventIndex,
        pub propertyStart: root::PropertyIndex,
        pub nestedTypesStart: root::NestedTypeIndex,
        pub interfacesStart: root::InterfacesIndex,
        pub vtableStart: root::VTableIndex,
        pub interfaceOffsetsStart: root::InterfacesIndex,
        pub method_count: u16,
        pub property_count: u16,
        pub field_count: u16,
        pub event_count: u16,
        pub nested_type_count: u16,
        pub vtable_count: u16,
        pub interfaces_count: u16,
        pub interface_offsets_count: u16,
        pub bitfield: u32,
        pub token: u32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppFieldDefinition {
        pub nameIndex: root::StringIndex,
        pub typeIndex: root::TypeIndex,
        pub token: u32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppFieldDefaultValue {
        pub fieldIndex: root::FieldIndex,
        pub typeIndex: root::TypeIndex,
        pub dataIndex: root::DefaultValueDataIndex,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppFieldMarshaledSize {
        pub fieldIndex: root::FieldIndex,
        pub typeIndex: root::TypeIndex,
        pub size: i32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppFieldRef {
        pub typeIndex: root::TypeIndex,
        pub fieldIndex: root::FieldIndex,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppParameterDefinition {
        pub nameIndex: root::StringIndex,
        pub token: u32,
        pub typeIndex: root::TypeIndex,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppParameterDefaultValue {
        pub parameterIndex: root::ParameterIndex,
        pub typeIndex: root::TypeIndex,
        pub dataIndex: root::DefaultValueDataIndex,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppMethodDefinition {
        pub nameIndex: root::StringIndex,
        pub declaringType: root::TypeDefinitionIndex,
        pub returnType: root::TypeIndex,
        pub returnParameterToken: u32,
        pub parameterStart: root::ParameterIndex,
        pub genericContainerIndex: root::GenericContainerIndex,
        pub token: u32,
        pub flags: u16,
        pub iflags: u16,
        pub slot: u16,
        pub parameterCount: u16,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppEventDefinition {
        pub nameIndex: root::StringIndex,
        pub typeIndex: root::TypeIndex,
        pub add: root::MethodIndex,
        pub remove: root::MethodIndex,
        pub raise: root::MethodIndex,
        pub token: u32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppPropertyDefinition {
        pub nameIndex: root::StringIndex,
        pub get: root::MethodIndex,
        pub set: root::MethodIndex,
        pub attrs: u32,
        pub token: u32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppStringLiteral {
        pub length: u32,
        pub dataIndex: root::StringLiteralIndex,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppAssemblyNameDefinition {
        pub nameIndex: root::StringIndex,
        pub cultureIndex: root::StringIndex,
        pub publicKeyIndex: root::StringIndex,
        pub hash_alg: u32,
        pub hash_len: i32,
        pub flags: u32,
        pub major: i32,
        pub minor: i32,
        pub build: i32,
        pub revision: i32,
        pub public_key_token: [u8; 8usize],
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppImageDefinition {
        pub nameIndex: root::StringIndex,
        pub assemblyIndex: root::AssemblyIndex,
        pub typeStart: root::TypeDefinitionIndex,
        pub typeCount: u32,
        pub exportedTypeStart: root::TypeDefinitionIndex,
        pub exportedTypeCount: u32,
        pub entryPointIndex: root::MethodIndex,
        pub token: u32,
        pub customAttributeStart: root::CustomAttributeIndex,
        pub customAttributeCount: u32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppAssemblyDefinition {
        pub imageIndex: root::ImageIndex,
        pub token: u32,
        pub referencedAssemblyStart: i32,
        pub referencedAssemblyCount: i32,
        pub aname: root::Il2CppAssemblyNameDefinition,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppCustomAttributeDataRange {
        pub token: u32,
        pub startOffset: u32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppMetadataRange {
        pub start: i32,
        pub length: i32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppGenericContainer {
        pub ownerIndex: i32,
        pub type_argc: i32,
        pub is_method: i32,
        pub genericParameterStart: root::GenericParameterIndex,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppGenericParameter {
        pub ownerIndex: root::GenericContainerIndex,
        pub nameIndex: root::StringIndex,
        pub constraintsStart: root::GenericParameterConstraintIndex,
        pub constraintsCount: i16,
        pub num: u16,
        pub flags: u16,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppWindowsRuntimeTypeNamePair {
        pub nameIndex: root::StringIndex,
        pub typeIndex: root::TypeIndex,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppGlobalMetadataHeader {
        pub sanity: i32,
        pub version: i32,
        pub stringLiteralOffset: i32,
        pub stringLiteralSize: i32,
        pub stringLiteralDataOffset: i32,
        pub stringLiteralDataSize: i32,
        pub stringOffset: i32,
        pub stringSize: i32,
        pub eventsOffset: i32,
        pub eventsSize: i32,
        pub propertiesOffset: i32,
        pub propertiesSize: i32,
        pub methodsOffset: i32,
        pub methodsSize: i32,
        pub parameterDefaultValuesOffset: i32,
        pub parameterDefaultValuesSize: i32,
        pub fieldDefaultValuesOffset: i32,
        pub fieldDefaultValuesSize: i32,
        pub fieldAndParameterDefaultValueDataOffset: i32,
        pub fieldAndParameterDefaultValueDataSize: i32,
        pub fieldMarshaledSizesOffset: i32,
        pub fieldMarshaledSizesSize: i32,
        pub parametersOffset: i32,
        pub parametersSize: i32,
        pub fieldsOffset: i32,
        pub fieldsSize: i32,
        pub genericParametersOffset: i32,
        pub genericParametersSize: i32,
        pub genericParameterConstraintsOffset: i32,
        pub genericParameterConstraintsSize: i32,
        pub genericContainersOffset: i32,
        pub genericContainersSize: i32,
        pub nestedTypesOffset: i32,
        pub nestedTypesSize: i32,
        pub interfacesOffset: i32,
        pub interfacesSize: i32,
        pub vtableMethodsOffset: i32,
        pub vtableMethodsSize: i32,
        pub interfaceOffsetsOffset: i32,
        pub interfaceOffsetsSize: i32,
        pub typeDefinitionsOffset: i32,
        pub typeDefinitionsSize: i32,
        pub imagesOffset: i32,
        pub imagesSize: i32,
        pub assembliesOffset: i32,
        pub assembliesSize: i32,
        pub fieldRefsOffset: i32,
        pub fieldRefsSize: i32,
        pub referencedAssembliesOffset: i32,
        pub referencedAssembliesSize: i32,
        pub attributeDataOffset: i32,
        pub attributeDataSize: i32,
        pub attributeDataRangeOffset: i32,
        pub attributeDataRangeSize: i32,
        pub unresolvedIndirectCallParameterTypesOffset: i32,
        pub unresolvedIndirectCallParameterTypesSize: i32,
        pub unresolvedIndirectCallParameterRangesOffset: i32,
        pub unresolvedIndirectCallParameterRangesSize: i32,
        pub windowsRuntimeTypeNamesOffset: i32,
        pub windowsRuntimeTypeNamesSize: i32,
        pub windowsRuntimeStringsOffset: i32,
        pub windowsRuntimeStringsSize: i32,
        pub exportedTypeDefinitionsOffset: i32,
        pub exportedTypeDefinitionsSize: i32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppMetadataField {
        pub offset: u32,
        pub typeIndex: u32,
        pub name: *const ::std::os::raw::c_char,
        pub isStatic: u8,
    }
    impl Default for Il2CppMetadataField {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    pub const kNone: Il2CppMetadataTypeFlags = 0;
    pub const kValueType: Il2CppMetadataTypeFlags = 1;
    pub const kArray: Il2CppMetadataTypeFlags = 2;
    pub const kArrayRankMask: Il2CppMetadataTypeFlags = -65536;
    pub type Il2CppMetadataTypeFlags = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppMetadataType {
        pub flags: root::Il2CppMetadataTypeFlags,
        pub fields: *mut root::Il2CppMetadataField,
        pub fieldCount: u32,
        pub staticsSize: u32,
        pub statics: *mut u8,
        pub baseOrElementTypeIndex: u32,
        pub name: *mut ::std::os::raw::c_char,
        pub assemblyName: *const ::std::os::raw::c_char,
        pub typeInfoAddress: u64,
        pub size: u32,
    }
    impl Default for Il2CppMetadataType {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppMetadataSnapshot {
        pub typeCount: u32,
        pub types: *mut root::Il2CppMetadataType,
    }
    impl Default for Il2CppMetadataSnapshot {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppManagedMemorySection {
        pub sectionStartAddress: u64,
        pub sectionSize: u32,
        pub sectionBytes: *mut u8,
    }
    impl Default for Il2CppManagedMemorySection {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppManagedHeap {
        pub sectionCount: u32,
        pub sections: *mut root::Il2CppManagedMemorySection,
    }
    impl Default for Il2CppManagedHeap {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppStacks {
        pub stackCount: u32,
        pub stacks: *mut root::Il2CppManagedMemorySection,
    }
    impl Default for Il2CppStacks {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct NativeObject {
        pub gcHandleIndex: u32,
        pub size: u32,
        pub instanceId: u32,
        pub classId: u32,
        pub referencedNativeObjectIndicesCount: u32,
        pub referencedNativeObjectIndices: *mut u32,
    }
    impl Default for NativeObject {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppGCHandles {
        pub trackedObjectCount: u32,
        pub pointersToObjects: *mut u64,
    }
    impl Default for Il2CppGCHandles {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppRuntimeInformation {
        pub pointerSize: u32,
        pub objectHeaderSize: u32,
        pub arrayHeaderSize: u32,
        pub arrayBoundsOffsetInHeader: u32,
        pub arraySizeOffsetInHeader: u32,
        pub allocationGranularity: u32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppManagedMemorySnapshot {
        pub heap: root::Il2CppManagedHeap,
        pub stacks: root::Il2CppStacks,
        pub metadata: root::Il2CppMetadataSnapshot,
        pub gcHandles: root::Il2CppGCHandles,
        pub runtimeInformation: root::Il2CppRuntimeInformation,
        pub additionalUserInformation: *mut ::std::os::raw::c_void,
    }
    impl Default for Il2CppManagedMemorySnapshot {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    pub const IL2CPP_TYPE_END: Il2CppTypeEnum = 0;
    pub const IL2CPP_TYPE_VOID: Il2CppTypeEnum = 1;
    pub const IL2CPP_TYPE_BOOLEAN: Il2CppTypeEnum = 2;
    pub const IL2CPP_TYPE_CHAR: Il2CppTypeEnum = 3;
    pub const IL2CPP_TYPE_I1: Il2CppTypeEnum = 4;
    pub const IL2CPP_TYPE_U1: Il2CppTypeEnum = 5;
    pub const IL2CPP_TYPE_I2: Il2CppTypeEnum = 6;
    pub const IL2CPP_TYPE_U2: Il2CppTypeEnum = 7;
    pub const IL2CPP_TYPE_I4: Il2CppTypeEnum = 8;
    pub const IL2CPP_TYPE_U4: Il2CppTypeEnum = 9;
    pub const IL2CPP_TYPE_I8: Il2CppTypeEnum = 10;
    pub const IL2CPP_TYPE_U8: Il2CppTypeEnum = 11;
    pub const IL2CPP_TYPE_R4: Il2CppTypeEnum = 12;
    pub const IL2CPP_TYPE_R8: Il2CppTypeEnum = 13;
    pub const IL2CPP_TYPE_STRING: Il2CppTypeEnum = 14;
    pub const IL2CPP_TYPE_PTR: Il2CppTypeEnum = 15;
    pub const IL2CPP_TYPE_BYREF: Il2CppTypeEnum = 16;
    pub const IL2CPP_TYPE_VALUETYPE: Il2CppTypeEnum = 17;
    pub const IL2CPP_TYPE_CLASS: Il2CppTypeEnum = 18;
    pub const IL2CPP_TYPE_VAR: Il2CppTypeEnum = 19;
    pub const IL2CPP_TYPE_ARRAY: Il2CppTypeEnum = 20;
    pub const IL2CPP_TYPE_GENERICINST: Il2CppTypeEnum = 21;
    pub const IL2CPP_TYPE_TYPEDBYREF: Il2CppTypeEnum = 22;
    pub const IL2CPP_TYPE_I: Il2CppTypeEnum = 24;
    pub const IL2CPP_TYPE_U: Il2CppTypeEnum = 25;
    pub const IL2CPP_TYPE_FNPTR: Il2CppTypeEnum = 27;
    pub const IL2CPP_TYPE_OBJECT: Il2CppTypeEnum = 28;
    pub const IL2CPP_TYPE_SZARRAY: Il2CppTypeEnum = 29;
    pub const IL2CPP_TYPE_MVAR: Il2CppTypeEnum = 30;
    pub const IL2CPP_TYPE_CMOD_REQD: Il2CppTypeEnum = 31;
    pub const IL2CPP_TYPE_CMOD_OPT: Il2CppTypeEnum = 32;
    pub const IL2CPP_TYPE_INTERNAL: Il2CppTypeEnum = 33;
    pub const IL2CPP_TYPE_MODIFIER: Il2CppTypeEnum = 64;
    pub const IL2CPP_TYPE_SENTINEL: Il2CppTypeEnum = 65;
    pub const IL2CPP_TYPE_PINNED: Il2CppTypeEnum = 69;
    pub const IL2CPP_TYPE_ENUM: Il2CppTypeEnum = 85;
    pub const IL2CPP_TYPE_IL2CPP_TYPE_INDEX: Il2CppTypeEnum = 255;
    pub type Il2CppTypeEnum = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppArrayType {
        pub etype: *const root::Il2CppType,
        pub rank: u8,
        pub numsizes: u8,
        pub numlobounds: u8,
        pub sizes: *mut ::std::os::raw::c_int,
        pub lobounds: *mut ::std::os::raw::c_int,
    }
    impl Default for Il2CppArrayType {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppGenericInst {
        pub type_argc: u32,
        pub type_argv: *mut *const root::Il2CppType,
    }
    impl Default for Il2CppGenericInst {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppGenericContext {
        pub class_inst: *const root::Il2CppGenericInst,
        pub method_inst: *const root::Il2CppGenericInst,
    }
    impl Default for Il2CppGenericContext {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppGenericClass {
        pub type_: *const root::Il2CppType,
        pub context: root::Il2CppGenericContext,
        pub cached_class: *mut root::Il2CppClass,
    }
    impl Default for Il2CppGenericClass {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppGenericMethod {
        pub methodDefinition: *const root::MethodInfo,
        pub context: root::Il2CppGenericContext,
    }
    impl Default for Il2CppGenericMethod {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppType {
        pub data: root::Il2CppType__bindgen_ty_1,
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: root::__BindgenBitfieldUnit<[u8; 4usize]>,
        pub __bindgen_padding_0: u32,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Il2CppType__bindgen_ty_1 {
        pub dummy: *mut ::std::os::raw::c_void,
        pub __klassIndex: root::TypeDefinitionIndex,
        pub typeHandle: root::Il2CppMetadataTypeHandle,
        pub type_: *const root::Il2CppType,
        pub array: *mut root::Il2CppArrayType,
        pub __genericParameterIndex: root::GenericParameterIndex,
        pub genericParameterHandle: root::Il2CppMetadataGenericParameterHandle,
        pub generic_class: *mut root::Il2CppGenericClass,
    }
    impl Default for Il2CppType__bindgen_ty_1 {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppType__bindgen_ty_1 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppType__bindgen_ty_1 {{ union }}")
        }
    }
    impl Default for Il2CppType {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppType {{ data: {:?}, attrs : {:?}, type : {:?}, num_mods : {:?}, byref : {:?}, pinned : {:?}, valuetype : {:?} }}" , self . data , self . attrs () , self . type_ () , self . num_mods () , self . byref () , self . pinned () , self . valuetype ())
        }
    }
    impl Il2CppType {
        #[inline]
        pub fn attrs(&self) -> ::std::os::raw::c_uint {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 16u8) as u32) }
        }
        #[inline]
        pub fn set_attrs(&mut self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn attrs_raw(this: *const Self) -> ::std::os::raw::c_uint {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    0usize,
                    16u8,
                ) as u32)
            }
        }
        #[inline]
        pub unsafe fn set_attrs_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    0usize,
                    16u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn type_(&self) -> root::Il2CppTypeEnum {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 8u8) as u32) }
        }
        #[inline]
        pub fn set_type(&mut self, val: root::Il2CppTypeEnum) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(16usize, 8u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn type__raw(this: *const Self) -> root::Il2CppTypeEnum {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    16usize,
                    8u8,
                ) as u32)
            }
        }
        #[inline]
        pub unsafe fn set_type_raw(this: *mut Self, val: root::Il2CppTypeEnum) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    16usize,
                    8u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn num_mods(&self) -> ::std::os::raw::c_uint {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 5u8) as u32) }
        }
        #[inline]
        pub fn set_num_mods(&mut self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(24usize, 5u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn num_mods_raw(this: *const Self) -> ::std::os::raw::c_uint {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    24usize,
                    5u8,
                ) as u32)
            }
        }
        #[inline]
        pub unsafe fn set_num_mods_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    24usize,
                    5u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn byref(&self) -> ::std::os::raw::c_uint {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(29usize, 1u8) as u32) }
        }
        #[inline]
        pub fn set_byref(&mut self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(29usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn byref_raw(this: *const Self) -> ::std::os::raw::c_uint {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    29usize,
                    1u8,
                ) as u32)
            }
        }
        #[inline]
        pub unsafe fn set_byref_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    29usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn pinned(&self) -> ::std::os::raw::c_uint {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(30usize, 1u8) as u32) }
        }
        #[inline]
        pub fn set_pinned(&mut self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(30usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn pinned_raw(this: *const Self) -> ::std::os::raw::c_uint {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    30usize,
                    1u8,
                ) as u32)
            }
        }
        #[inline]
        pub unsafe fn set_pinned_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    30usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn valuetype(&self) -> ::std::os::raw::c_uint {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
        }
        #[inline]
        pub fn set_valuetype(&mut self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(31usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn valuetype_raw(this: *const Self) -> ::std::os::raw::c_uint {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    31usize,
                    1u8,
                ) as u32)
            }
        }
        #[inline]
        pub unsafe fn set_valuetype_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    31usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn new_bitfield_1(
            attrs: ::std::os::raw::c_uint,
            type_: root::Il2CppTypeEnum,
            num_mods: ::std::os::raw::c_uint,
            byref: ::std::os::raw::c_uint,
            pinned: ::std::os::raw::c_uint,
            valuetype: ::std::os::raw::c_uint,
        ) -> root::__BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit: root::__BindgenBitfieldUnit<[u8; 4usize]> =
                Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8, {
                let attrs: u32 = unsafe { ::std::mem::transmute(attrs) };
                attrs as u64
            });
            __bindgen_bitfield_unit.set(16usize, 8u8, {
                let type_: u32 = unsafe { ::std::mem::transmute(type_) };
                type_ as u64
            });
            __bindgen_bitfield_unit.set(24usize, 5u8, {
                let num_mods: u32 = unsafe { ::std::mem::transmute(num_mods) };
                num_mods as u64
            });
            __bindgen_bitfield_unit.set(29usize, 1u8, {
                let byref: u32 = unsafe { ::std::mem::transmute(byref) };
                byref as u64
            });
            __bindgen_bitfield_unit.set(30usize, 1u8, {
                let pinned: u32 = unsafe { ::std::mem::transmute(pinned) };
                pinned as u64
            });
            __bindgen_bitfield_unit.set(31usize, 1u8, {
                let valuetype: u32 = unsafe { ::std::mem::transmute(valuetype) };
                valuetype as u64
            });
            __bindgen_bitfield_unit
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppMetadataFieldInfo {
        pub type_: *const root::Il2CppType,
        pub name: *const ::std::os::raw::c_char,
        pub token: u32,
    }
    impl Default for Il2CppMetadataFieldInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppMetadataMethodInfo {
        pub handle: root::Il2CppMetadataMethodDefinitionHandle,
        pub name: *const ::std::os::raw::c_char,
        pub return_type: *const root::Il2CppType,
        pub token: u32,
        pub flags: u16,
        pub iflags: u16,
        pub slot: u16,
        pub parameterCount: u16,
    }
    impl Default for Il2CppMetadataMethodInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppMetadataParameterInfo {
        pub name: *const ::std::os::raw::c_char,
        pub token: u32,
        pub type_: *const root::Il2CppType,
    }
    impl Default for Il2CppMetadataParameterInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppMetadataPropertyInfo {
        pub name: *const ::std::os::raw::c_char,
        pub get: *const root::MethodInfo,
        pub set: *const root::MethodInfo,
        pub attrs: u32,
        pub token: u32,
    }
    impl Default for Il2CppMetadataPropertyInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppMetadataEventInfo {
        pub name: *const ::std::os::raw::c_char,
        pub type_: *const root::Il2CppType,
        pub add: *const root::MethodInfo,
        pub remove: *const root::MethodInfo,
        pub raise: *const root::MethodInfo,
        pub token: u32,
    }
    impl Default for Il2CppMetadataEventInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppInterfaceOffsetInfo {
        pub interfaceType: *const root::Il2CppType,
        pub offset: i32,
    }
    impl Default for Il2CppInterfaceOffsetInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppGenericParameterInfo {
        pub containerHandle: root::Il2CppMetadataGenericContainerHandle,
        pub name: *const ::std::os::raw::c_char,
        pub num: u16,
        pub flags: u16,
    }
    impl Default for Il2CppGenericParameterInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    pub const IL2CPP_CALL_DEFAULT: Il2CppCallConvention = 0;
    pub const IL2CPP_CALL_C: Il2CppCallConvention = 1;
    pub const IL2CPP_CALL_STDCALL: Il2CppCallConvention = 2;
    pub const IL2CPP_CALL_THISCALL: Il2CppCallConvention = 3;
    pub const IL2CPP_CALL_FASTCALL: Il2CppCallConvention = 4;
    pub const IL2CPP_CALL_VARARG: Il2CppCallConvention = 5;
    pub type Il2CppCallConvention = ::std::os::raw::c_int;
    pub const CHARSET_ANSI: Il2CppCharSet = 0;
    pub const CHARSET_UNICODE: Il2CppCharSet = 1;
    pub const CHARSET_NOT_SPECIFIED: Il2CppCharSet = 2;
    pub type Il2CppCharSet = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppHString__ {
        pub unused: ::std::os::raw::c_int,
    }
    pub type Il2CppHString = *mut root::Il2CppHString__;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppHStringHeader {
        pub Reserved: root::Il2CppHStringHeader__bindgen_ty_1,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Il2CppHStringHeader__bindgen_ty_1 {
        pub Reserved1: *mut ::std::os::raw::c_void,
        pub Reserved2: [::std::os::raw::c_char; 24usize],
    }
    impl Default for Il2CppHStringHeader__bindgen_ty_1 {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppHStringHeader__bindgen_ty_1 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppHStringHeader__bindgen_ty_1 {{ union }}")
        }
    }
    impl Default for Il2CppHStringHeader {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppHStringHeader {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppHStringHeader {{ Reserved: {:?} }}", self.Reserved)
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppGuid {
        pub data1: u32,
        pub data2: u16,
        pub data3: u16,
        pub data4: [u8; 8usize],
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppSafeArrayBound {
        pub element_count: u32,
        pub lower_bound: i32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppSafeArray {
        pub dimension_count: u16,
        pub features: u16,
        pub element_size: u32,
        pub lock_count: u32,
        pub data: *mut ::std::os::raw::c_void,
        pub bounds: [root::Il2CppSafeArrayBound; 1usize],
    }
    impl Default for Il2CppSafeArray {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppWin32Decimal {
        pub reserved: u16,
        pub u: root::Il2CppWin32Decimal__bindgen_ty_1,
        pub hi32: u32,
        pub u2: root::Il2CppWin32Decimal__bindgen_ty_2,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Il2CppWin32Decimal__bindgen_ty_1 {
        pub s: root::Il2CppWin32Decimal__bindgen_ty_1__bindgen_ty_1,
        pub signscale: u16,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppWin32Decimal__bindgen_ty_1__bindgen_ty_1 {
        pub scale: u8,
        pub sign: u8,
    }
    impl Default for Il2CppWin32Decimal__bindgen_ty_1 {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppWin32Decimal__bindgen_ty_1 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppWin32Decimal__bindgen_ty_1 {{ union }}")
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Il2CppWin32Decimal__bindgen_ty_2 {
        pub s2: root::Il2CppWin32Decimal__bindgen_ty_2__bindgen_ty_1,
        pub lo64: u64,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppWin32Decimal__bindgen_ty_2__bindgen_ty_1 {
        pub lo32: u32,
        pub mid32: u32,
    }
    impl Default for Il2CppWin32Decimal__bindgen_ty_2 {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppWin32Decimal__bindgen_ty_2 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppWin32Decimal__bindgen_ty_2 {{ union }}")
        }
    }
    impl Default for Il2CppWin32Decimal {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppWin32Decimal {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppWin32Decimal {{ reserved: {:?}, u: {:?}, hi32: {:?}, u2: {:?} }}",
                self.reserved, self.u, self.hi32, self.u2
            )
        }
    }
    pub type IL2CPP_VARIANT_BOOL = i16;
    pub const IL2CPP_VT_EMPTY: Il2CppVarType = 0;
    pub const IL2CPP_VT_NULL: Il2CppVarType = 1;
    pub const IL2CPP_VT_I2: Il2CppVarType = 2;
    pub const IL2CPP_VT_I4: Il2CppVarType = 3;
    pub const IL2CPP_VT_R4: Il2CppVarType = 4;
    pub const IL2CPP_VT_R8: Il2CppVarType = 5;
    pub const IL2CPP_VT_CY: Il2CppVarType = 6;
    pub const IL2CPP_VT_DATE: Il2CppVarType = 7;
    pub const IL2CPP_VT_BSTR: Il2CppVarType = 8;
    pub const IL2CPP_VT_DISPATCH: Il2CppVarType = 9;
    pub const IL2CPP_VT_ERROR: Il2CppVarType = 10;
    pub const IL2CPP_VT_BOOL: Il2CppVarType = 11;
    pub const IL2CPP_VT_VARIANT: Il2CppVarType = 12;
    pub const IL2CPP_VT_UNKNOWN: Il2CppVarType = 13;
    pub const IL2CPP_VT_DECIMAL: Il2CppVarType = 14;
    pub const IL2CPP_VT_I1: Il2CppVarType = 16;
    pub const IL2CPP_VT_UI1: Il2CppVarType = 17;
    pub const IL2CPP_VT_UI2: Il2CppVarType = 18;
    pub const IL2CPP_VT_UI4: Il2CppVarType = 19;
    pub const IL2CPP_VT_I8: Il2CppVarType = 20;
    pub const IL2CPP_VT_UI8: Il2CppVarType = 21;
    pub const IL2CPP_VT_INT: Il2CppVarType = 22;
    pub const IL2CPP_VT_UINT: Il2CppVarType = 23;
    pub const IL2CPP_VT_VOID: Il2CppVarType = 24;
    pub const IL2CPP_VT_HRESULT: Il2CppVarType = 25;
    pub const IL2CPP_VT_PTR: Il2CppVarType = 26;
    pub const IL2CPP_VT_SAFEARRAY: Il2CppVarType = 27;
    pub const IL2CPP_VT_CARRAY: Il2CppVarType = 28;
    pub const IL2CPP_VT_USERDEFINED: Il2CppVarType = 29;
    pub const IL2CPP_VT_LPSTR: Il2CppVarType = 30;
    pub const IL2CPP_VT_LPWSTR: Il2CppVarType = 31;
    pub const IL2CPP_VT_RECORD: Il2CppVarType = 36;
    pub const IL2CPP_VT_INT_PTR: Il2CppVarType = 37;
    pub const IL2CPP_VT_UINT_PTR: Il2CppVarType = 38;
    pub const IL2CPP_VT_FILETIME: Il2CppVarType = 64;
    pub const IL2CPP_VT_BLOB: Il2CppVarType = 65;
    pub const IL2CPP_VT_STREAM: Il2CppVarType = 66;
    pub const IL2CPP_VT_STORAGE: Il2CppVarType = 67;
    pub const IL2CPP_VT_STREAMED_OBJECT: Il2CppVarType = 68;
    pub const IL2CPP_VT_STORED_OBJECT: Il2CppVarType = 69;
    pub const IL2CPP_VT_BLOB_OBJECT: Il2CppVarType = 70;
    pub const IL2CPP_VT_CF: Il2CppVarType = 71;
    pub const IL2CPP_VT_CLSID: Il2CppVarType = 72;
    pub const IL2CPP_VT_VERSIONED_STREAM: Il2CppVarType = 73;
    pub const IL2CPP_VT_BSTR_BLOB: Il2CppVarType = 4095;
    pub const IL2CPP_VT_VECTOR: Il2CppVarType = 4096;
    pub const IL2CPP_VT_ARRAY: Il2CppVarType = 8192;
    pub const IL2CPP_VT_BYREF: Il2CppVarType = 16384;
    pub const IL2CPP_VT_RESERVED: Il2CppVarType = 32768;
    pub const IL2CPP_VT_ILLEGAL: Il2CppVarType = 65535;
    pub const IL2CPP_VT_ILLEGALMASKED: Il2CppVarType = 4095;
    pub const IL2CPP_VT_TYPEMASK: Il2CppVarType = 4095;
    pub type Il2CppVarType = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppIUnknown {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppVariant {
        pub n1: root::Il2CppVariant__bindgen_ty_1,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Il2CppVariant__bindgen_ty_1 {
        pub n2: root::Il2CppVariant__bindgen_ty_1___tagVARIANT,
        pub decVal: root::Il2CppWin32Decimal,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppVariant__bindgen_ty_1___tagVARIANT {
        pub type_: u16,
        pub reserved1: u16,
        pub reserved2: u16,
        pub reserved3: u16,
        pub n3: root::Il2CppVariant__bindgen_ty_1___tagVARIANT__bindgen_ty_1,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Il2CppVariant__bindgen_ty_1___tagVARIANT__bindgen_ty_1 {
        pub llVal: i64,
        pub lVal: i32,
        pub bVal: u8,
        pub iVal: i16,
        pub fltVal: f32,
        pub dblVal: f64,
        pub boolVal: root::IL2CPP_VARIANT_BOOL,
        pub scode: i32,
        pub cyVal: i64,
        pub date: f64,
        pub bstrVal: *mut root::Il2CppChar,
        pub punkVal: *mut root::Il2CppIUnknown,
        pub pdispVal: *mut ::std::os::raw::c_void,
        pub parray: *mut root::Il2CppSafeArray,
        pub pbVal: *mut u8,
        pub piVal: *mut i16,
        pub plVal: *mut i32,
        pub pllVal: *mut i64,
        pub pfltVal: *mut f32,
        pub pdblVal: *mut f64,
        pub pboolVal: *mut root::IL2CPP_VARIANT_BOOL,
        pub pscode: *mut i32,
        pub pcyVal: *mut i64,
        pub pdate: *mut f64,
        pub pbstrVal: *mut root::Il2CppChar,
        pub ppunkVal: *mut *mut root::Il2CppIUnknown,
        pub ppdispVal: *mut *mut ::std::os::raw::c_void,
        pub pparray: *mut *mut root::Il2CppSafeArray,
        pub pvarVal: *mut root::Il2CppVariant,
        pub byref: *mut ::std::os::raw::c_void,
        pub cVal: ::std::os::raw::c_char,
        pub uiVal: u16,
        pub ulVal: u32,
        pub ullVal: u64,
        pub intVal: ::std::os::raw::c_int,
        pub uintVal: ::std::os::raw::c_uint,
        pub pdecVal: *mut root::Il2CppWin32Decimal,
        pub pcVal: *mut ::std::os::raw::c_char,
        pub puiVal: *mut u16,
        pub pulVal: *mut u32,
        pub pullVal: *mut u64,
        pub pintVal: *mut ::std::os::raw::c_int,
        pub puintVal: *mut ::std::os::raw::c_uint,
        pub n4: root::Il2CppVariant__bindgen_ty_1___tagVARIANT__bindgen_ty_1___tagBRECORD,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppVariant__bindgen_ty_1___tagVARIANT__bindgen_ty_1___tagBRECORD {
        pub pvRecord: *mut ::std::os::raw::c_void,
        pub pRecInfo: *mut ::std::os::raw::c_void,
    }
    impl Default for Il2CppVariant__bindgen_ty_1___tagVARIANT__bindgen_ty_1___tagBRECORD {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl Default for Il2CppVariant__bindgen_ty_1___tagVARIANT__bindgen_ty_1 {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppVariant__bindgen_ty_1___tagVARIANT__bindgen_ty_1 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppVariant__bindgen_ty_1___tagVARIANT__bindgen_ty_1 {{ union }}"
            )
        }
    }
    impl Default for Il2CppVariant__bindgen_ty_1___tagVARIANT {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppVariant__bindgen_ty_1___tagVARIANT {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppVariant__bindgen_ty_1___tagVARIANT {{ type: {:?}, reserved1: {:?}, reserved2: {:?}, reserved3: {:?}, n3: {:?} }}" , self . type_ , self . reserved1 , self . reserved2 , self . reserved3 , self . n3)
        }
    }
    impl Default for Il2CppVariant__bindgen_ty_1 {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppVariant__bindgen_ty_1 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppVariant__bindgen_ty_1 {{ union }}")
        }
    }
    impl Default for Il2CppVariant {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppVariant {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppVariant {{ n1: {:?} }}", self.n1)
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppFileTime {
        pub low: u32,
        pub high: u32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppStatStg {
        pub name: *mut root::Il2CppChar,
        pub type_: u32,
        pub size: u64,
        pub mtime: root::Il2CppFileTime,
        pub ctime: root::Il2CppFileTime,
        pub atime: root::Il2CppFileTime,
        pub mode: u32,
        pub locks: u32,
        pub clsid: root::Il2CppGuid,
        pub state: u32,
        pub reserved: u32,
    }
    impl Default for Il2CppStatStg {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    pub const kTypeKindPrimitive: Il2CppWindowsRuntimeTypeKind = 0;
    pub const kTypeKindMetadata: Il2CppWindowsRuntimeTypeKind = 1;
    pub const kTypeKindCustom: Il2CppWindowsRuntimeTypeKind = 2;
    pub type Il2CppWindowsRuntimeTypeKind = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppWindowsRuntimeTypeName {
        pub typeName: root::Il2CppHString,
        pub typeKind: root::Il2CppWindowsRuntimeTypeKind,
    }
    impl Default for Il2CppWindowsRuntimeTypeName {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    pub type PInvokeMarshalToNativeFunc = ::std::option::Option<
        unsafe extern "C" fn(
            managedStructure: *mut ::std::os::raw::c_void,
            marshaledStructure: *mut ::std::os::raw::c_void,
        ),
    >;
    pub type PInvokeMarshalFromNativeFunc = ::std::option::Option<
        unsafe extern "C" fn(
            marshaledStructure: *mut ::std::os::raw::c_void,
            managedStructure: *mut ::std::os::raw::c_void,
        ),
    >;
    pub type PInvokeMarshalCleanupFunc = ::std::option::Option<
        unsafe extern "C" fn(marshaledStructure: *mut ::std::os::raw::c_void),
    >;
    pub type CreateCCWFunc = ::std::option::Option<
        unsafe extern "C" fn(obj: *mut root::Il2CppObject) -> *mut root::Il2CppIUnknown,
    >;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppInteropData {
        pub delegatePInvokeWrapperFunction: root::Il2CppMethodPointer,
        pub pinvokeMarshalToNativeFunction: root::PInvokeMarshalToNativeFunc,
        pub pinvokeMarshalFromNativeFunction: root::PInvokeMarshalFromNativeFunc,
        pub pinvokeMarshalCleanupFunction: root::PInvokeMarshalCleanupFunc,
        pub createCCWFunction: root::CreateCCWFunc,
        pub guid: *const root::Il2CppGuid,
        pub type_: *const root::Il2CppType,
    }
    impl Default for Il2CppInteropData {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppNameToTypeHandleHashTable {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VirtualInvokeData {
        pub methodPtr: root::Il2CppMethodPointer,
        pub method: *const root::MethodInfo,
    }
    impl Default for VirtualInvokeData {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    pub const IL2CPP_TYPE_NAME_FORMAT_IL: Il2CppTypeNameFormat = 0;
    pub const IL2CPP_TYPE_NAME_FORMAT_REFLECTION: Il2CppTypeNameFormat = 1;
    pub const IL2CPP_TYPE_NAME_FORMAT_FULL_NAME: Il2CppTypeNameFormat = 2;
    pub const IL2CPP_TYPE_NAME_FORMAT_ASSEMBLY_QUALIFIED: Il2CppTypeNameFormat = 3;
    pub type Il2CppTypeNameFormat = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppDefaults {
        pub corlib: *mut root::Il2CppImage,
        pub corlib_gen: *mut root::Il2CppImage,
        pub object_class: *mut root::Il2CppClass,
        pub byte_class: *mut root::Il2CppClass,
        pub void_class: *mut root::Il2CppClass,
        pub boolean_class: *mut root::Il2CppClass,
        pub sbyte_class: *mut root::Il2CppClass,
        pub int16_class: *mut root::Il2CppClass,
        pub uint16_class: *mut root::Il2CppClass,
        pub int32_class: *mut root::Il2CppClass,
        pub uint32_class: *mut root::Il2CppClass,
        pub int_class: *mut root::Il2CppClass,
        pub uint_class: *mut root::Il2CppClass,
        pub int64_class: *mut root::Il2CppClass,
        pub uint64_class: *mut root::Il2CppClass,
        pub single_class: *mut root::Il2CppClass,
        pub double_class: *mut root::Il2CppClass,
        pub char_class: *mut root::Il2CppClass,
        pub string_class: *mut root::Il2CppClass,
        pub enum_class: *mut root::Il2CppClass,
        pub array_class: *mut root::Il2CppClass,
        pub delegate_class: *mut root::Il2CppClass,
        pub multicastdelegate_class: *mut root::Il2CppClass,
        pub asyncresult_class: *mut root::Il2CppClass,
        pub manualresetevent_class: *mut root::Il2CppClass,
        pub typehandle_class: *mut root::Il2CppClass,
        pub fieldhandle_class: *mut root::Il2CppClass,
        pub methodhandle_class: *mut root::Il2CppClass,
        pub systemtype_class: *mut root::Il2CppClass,
        pub monotype_class: *mut root::Il2CppClass,
        pub exception_class: *mut root::Il2CppClass,
        pub threadabortexception_class: *mut root::Il2CppClass,
        pub thread_class: *mut root::Il2CppClass,
        pub internal_thread_class: *mut root::Il2CppClass,
        pub appdomain_class: *mut root::Il2CppClass,
        pub appdomain_setup_class: *mut root::Il2CppClass,
        pub member_info_class: *mut root::Il2CppClass,
        pub field_info_class: *mut root::Il2CppClass,
        pub method_info_class: *mut root::Il2CppClass,
        pub property_info_class: *mut root::Il2CppClass,
        pub event_info_class: *mut root::Il2CppClass,
        pub stringbuilder_class: *mut root::Il2CppClass,
        pub stack_frame_class: *mut root::Il2CppClass,
        pub stack_trace_class: *mut root::Il2CppClass,
        pub marshal_class: *mut root::Il2CppClass,
        pub typed_reference_class: *mut root::Il2CppClass,
        pub marshalbyrefobject_class: *mut root::Il2CppClass,
        pub generic_ilist_class: *mut root::Il2CppClass,
        pub generic_icollection_class: *mut root::Il2CppClass,
        pub generic_ienumerable_class: *mut root::Il2CppClass,
        pub generic_ireadonlylist_class: *mut root::Il2CppClass,
        pub generic_ireadonlycollection_class: *mut root::Il2CppClass,
        pub runtimetype_class: *mut root::Il2CppClass,
        pub generic_nullable_class: *mut root::Il2CppClass,
        pub il2cpp_com_object_class: *mut root::Il2CppClass,
        pub attribute_class: *mut root::Il2CppClass,
        pub customattribute_data_class: *mut root::Il2CppClass,
        pub customattribute_typed_argument_class: *mut root::Il2CppClass,
        pub customattribute_named_argument_class: *mut root::Il2CppClass,
        pub version: *mut root::Il2CppClass,
        pub culture_info: *mut root::Il2CppClass,
        pub async_call_class: *mut root::Il2CppClass,
        pub assembly_class: *mut root::Il2CppClass,
        pub assembly_name_class: *mut root::Il2CppClass,
        pub parameter_info_class: *mut root::Il2CppClass,
        pub module_class: *mut root::Il2CppClass,
        pub system_exception_class: *mut root::Il2CppClass,
        pub argument_exception_class: *mut root::Il2CppClass,
        pub wait_handle_class: *mut root::Il2CppClass,
        pub safe_handle_class: *mut root::Il2CppClass,
        pub sort_key_class: *mut root::Il2CppClass,
        pub dbnull_class: *mut root::Il2CppClass,
        pub error_wrapper_class: *mut root::Il2CppClass,
        pub missing_class: *mut root::Il2CppClass,
        pub value_type_class: *mut root::Il2CppClass,
        pub threadpool_wait_callback_class: *mut root::Il2CppClass,
        pub threadpool_perform_wait_callback_method: *mut root::MethodInfo,
        pub mono_method_message_class: *mut root::Il2CppClass,
        pub ireference_class: *mut root::Il2CppClass,
        pub ireferencearray_class: *mut root::Il2CppClass,
        pub ikey_value_pair_class: *mut root::Il2CppClass,
        pub key_value_pair_class: *mut root::Il2CppClass,
        pub windows_foundation_uri_class: *mut root::Il2CppClass,
        pub windows_foundation_iuri_runtime_class_class: *mut root::Il2CppClass,
        pub system_uri_class: *mut root::Il2CppClass,
        pub system_guid_class: *mut root::Il2CppClass,
        pub sbyte_shared_enum: *mut root::Il2CppClass,
        pub int16_shared_enum: *mut root::Il2CppClass,
        pub int32_shared_enum: *mut root::Il2CppClass,
        pub int64_shared_enum: *mut root::Il2CppClass,
        pub byte_shared_enum: *mut root::Il2CppClass,
        pub uint16_shared_enum: *mut root::Il2CppClass,
        pub uint32_shared_enum: *mut root::Il2CppClass,
        pub uint64_shared_enum: *mut root::Il2CppClass,
        pub il2cpp_fully_shared_type: *mut root::Il2CppClass,
        pub il2cpp_fully_shared_struct_type: *mut root::Il2CppClass,
    }
    impl Default for Il2CppDefaults {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    unsafe extern "C" {
        #[link_name = "\u{1}?il2cpp_defaults@@3UIl2CppDefaults@@A"]
        pub static mut il2cpp_defaults: root::Il2CppDefaults;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct MemberInfo {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct FieldInfo {
        pub name: *const ::std::os::raw::c_char,
        pub type_: *const root::Il2CppType,
        pub parent: *mut root::Il2CppClass,
        pub offset: i32,
        pub token: u32,
    }
    impl Default for FieldInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct PropertyInfo {
        pub parent: *mut root::Il2CppClass,
        pub name: *const ::std::os::raw::c_char,
        pub get: *const root::MethodInfo,
        pub set: *const root::MethodInfo,
        pub attrs: u32,
        pub token: u32,
    }
    impl Default for PropertyInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct EventInfo {
        pub name: *const ::std::os::raw::c_char,
        pub eventType: *const root::Il2CppType,
        pub parent: *mut root::Il2CppClass,
        pub add: *const root::MethodInfo,
        pub remove: *const root::MethodInfo,
        pub raise: *const root::MethodInfo,
        pub token: u32,
    }
    impl Default for EventInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    pub type InvokerMethod = ::std::option::Option<
        unsafe extern "C" fn(
            arg1: root::Il2CppMethodPointer,
            arg2: *const root::MethodInfo,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut *mut ::std::os::raw::c_void,
            arg5: *mut ::std::os::raw::c_void,
        ),
    >;
    pub const kMethodVariableKind_This: MethodVariableKind = 0;
    pub const kMethodVariableKind_Parameter: MethodVariableKind = 1;
    pub const kMethodVariableKind_LocalVariable: MethodVariableKind = 2;
    pub type MethodVariableKind = ::std::os::raw::c_int;
    pub const kSequencePointKind_Normal: SequencePointKind = 0;
    pub const kSequencePointKind_StepOut: SequencePointKind = 1;
    pub type SequencePointKind = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppMethodExecutionContextInfo {
        pub typeIndex: root::TypeIndex,
        pub nameIndex: i32,
        pub scopeIndex: i32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppMethodExecutionContextInfoIndex {
        pub startIndex: i32,
        pub count: i32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppMethodScope {
        pub startOffset: i32,
        pub endOffset: i32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppMethodHeaderInfo {
        pub code_size: i32,
        pub startScope: i32,
        pub numScopes: i32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppSequencePointSourceFile {
        pub file: *const ::std::os::raw::c_char,
        pub hash: [u8; 16usize],
    }
    impl Default for Il2CppSequencePointSourceFile {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppTypeSourceFilePair {
        pub __klassIndex: root::TypeDefinitionIndex,
        pub sourceFileIndex: i32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppSequencePoint {
        pub __methodDefinitionIndex: root::MethodIndex,
        pub sourceFileIndex: i32,
        pub lineStart: i32,
        pub lineEnd: i32,
        pub columnStart: i32,
        pub columnEnd: i32,
        pub ilOffset: i32,
        pub kind: root::SequencePointKind,
        pub isActive: i32,
        pub id: i32,
    }
    impl Default for Il2CppSequencePoint {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppCatchPoint {
        pub __methodDefinitionIndex: root::MethodIndex,
        pub catchTypeIndex: root::TypeIndex,
        pub ilOffset: i32,
        pub tryId: i32,
        pub parentTryId: i32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppDebuggerMetadataRegistration {
        pub methodExecutionContextInfos: *mut root::Il2CppMethodExecutionContextInfo,
        pub methodExecutionContextInfoIndexes: *mut root::Il2CppMethodExecutionContextInfoIndex,
        pub methodScopes: *mut root::Il2CppMethodScope,
        pub methodHeaderInfos: *mut root::Il2CppMethodHeaderInfo,
        pub sequencePointSourceFiles: *mut root::Il2CppSequencePointSourceFile,
        pub numSequencePoints: i32,
        pub sequencePoints: *mut root::Il2CppSequencePoint,
        pub numCatchPoints: i32,
        pub catchPoints: *mut root::Il2CppCatchPoint,
        pub numTypeSourceFileEntries: i32,
        pub typeSourceFiles: *mut root::Il2CppTypeSourceFilePair,
        pub methodExecutionContextInfoStrings: *mut *const ::std::os::raw::c_char,
    }
    impl Default for Il2CppDebuggerMetadataRegistration {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Il2CppRGCTXData {
        pub rgctxDataDummy: *mut ::std::os::raw::c_void,
        pub method: *const root::MethodInfo,
        pub type_: *const root::Il2CppType,
        pub klass: *mut root::Il2CppClass,
    }
    impl Default for Il2CppRGCTXData {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppRGCTXData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppRGCTXData {{ union }}")
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MethodInfo {
        pub methodPointer: root::Il2CppMethodPointer,
        pub virtualMethodPointer: root::Il2CppMethodPointer,
        pub invoker_method: root::InvokerMethod,
        pub name: *const ::std::os::raw::c_char,
        pub klass: *mut root::Il2CppClass,
        pub return_type: *const root::Il2CppType,
        pub parameters: *mut *const root::Il2CppType,
        pub __bindgen_anon_1: root::MethodInfo__bindgen_ty_1,
        pub __bindgen_anon_2: root::MethodInfo__bindgen_ty_2,
        pub token: u32,
        pub flags: u16,
        pub iflags: u16,
        pub slot: u16,
        pub parameters_count: u8,
        pub _bitfield_align_1: [u8; 0],
        pub _bitfield_1: root::__BindgenBitfieldUnit<[u8; 1usize]>,
        pub __bindgen_padding_0: u32,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union MethodInfo__bindgen_ty_1 {
        pub rgctx_data: *const root::Il2CppRGCTXData,
        pub methodMetadataHandle: root::Il2CppMetadataMethodDefinitionHandle,
    }
    impl Default for MethodInfo__bindgen_ty_1 {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for MethodInfo__bindgen_ty_1 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "MethodInfo__bindgen_ty_1 {{ union }}")
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union MethodInfo__bindgen_ty_2 {
        pub genericMethod: *const root::Il2CppGenericMethod,
        pub genericContainerHandle: root::Il2CppMetadataGenericContainerHandle,
    }
    impl Default for MethodInfo__bindgen_ty_2 {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for MethodInfo__bindgen_ty_2 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "MethodInfo__bindgen_ty_2 {{ union }}")
        }
    }
    impl Default for MethodInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for MethodInfo {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "MethodInfo {{ methodPointer: {:?}, virtualMethodPointer: {:?}, invoker_method: {:?}, name: {:?}, klass: {:?}, return_type: {:?}, parameters: {:?}, __bindgen_anon_1: {:?}, __bindgen_anon_2: {:?}, token: {:?}, flags: {:?}, iflags: {:?}, slot: {:?}, parameters_count: {:?}, is_generic : {:?}, is_inflated : {:?}, wrapper_type : {:?}, has_full_generic_sharing_signature : {:?} }}" , self . methodPointer , self . virtualMethodPointer , self . invoker_method , self . name , self . klass , self . return_type , self . parameters , self . __bindgen_anon_1 , self . __bindgen_anon_2 , self . token , self . flags , self . iflags , self . slot , self . parameters_count , self . is_generic () , self . is_inflated () , self . wrapper_type () , self . has_full_generic_sharing_signature ())
        }
    }
    impl MethodInfo {
        #[inline]
        pub fn is_generic(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_is_generic(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(0usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn is_generic_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    0usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_is_generic_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    0usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn is_inflated(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_is_inflated(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(1usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn is_inflated_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    1usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_is_inflated_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    1usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn wrapper_type(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_wrapper_type(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn wrapper_type_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    2usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_wrapper_type_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    2usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn has_full_generic_sharing_signature(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_has_full_generic_sharing_signature(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn has_full_generic_sharing_signature_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    3usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_has_full_generic_sharing_signature_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    3usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn new_bitfield_1(
            is_generic: u8,
            is_inflated: u8,
            wrapper_type: u8,
            has_full_generic_sharing_signature: u8,
        ) -> root::__BindgenBitfieldUnit<[u8; 1usize]> {
            let mut __bindgen_bitfield_unit: root::__BindgenBitfieldUnit<[u8; 1usize]> =
                Default::default();
            __bindgen_bitfield_unit.set(0usize, 1u8, {
                let is_generic: u8 = unsafe { ::std::mem::transmute(is_generic) };
                is_generic as u64
            });
            __bindgen_bitfield_unit.set(1usize, 1u8, {
                let is_inflated: u8 = unsafe { ::std::mem::transmute(is_inflated) };
                is_inflated as u64
            });
            __bindgen_bitfield_unit.set(2usize, 1u8, {
                let wrapper_type: u8 = unsafe { ::std::mem::transmute(wrapper_type) };
                wrapper_type as u64
            });
            __bindgen_bitfield_unit.set(3usize, 1u8, {
                let has_full_generic_sharing_signature: u8 =
                    unsafe { ::std::mem::transmute(has_full_generic_sharing_signature) };
                has_full_generic_sharing_signature as u64
            });
            __bindgen_bitfield_unit
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppRuntimeInterfaceOffsetPair {
        pub interfaceType: *mut root::Il2CppClass,
        pub offset: i32,
    }
    impl Default for Il2CppRuntimeInterfaceOffsetPair {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppClass {
        pub image: *const root::Il2CppImage,
        pub gc_desc: *mut ::std::os::raw::c_void,
        pub name: *const ::std::os::raw::c_char,
        pub namespaze: *const ::std::os::raw::c_char,
        pub byval_arg: root::Il2CppType,
        pub this_arg: root::Il2CppType,
        pub element_class: *mut root::Il2CppClass,
        pub castClass: *mut root::Il2CppClass,
        pub declaringType: *mut root::Il2CppClass,
        pub parent: *mut root::Il2CppClass,
        pub generic_class: *mut root::Il2CppGenericClass,
        pub typeMetadataHandle: root::Il2CppMetadataTypeHandle,
        pub interopData: *const root::Il2CppInteropData,
        pub klass: *mut root::Il2CppClass,
        pub fields: *mut root::FieldInfo,
        pub events: *const root::EventInfo,
        pub properties: *const root::PropertyInfo,
        pub methods: *mut *const root::MethodInfo,
        pub nestedTypes: *mut *mut root::Il2CppClass,
        pub implementedInterfaces: *mut *mut root::Il2CppClass,
        pub interfaceOffsets: *mut root::Il2CppRuntimeInterfaceOffsetPair,
        pub static_fields: *mut ::std::os::raw::c_void,
        pub rgctx_data: *const root::Il2CppRGCTXData,
        pub typeHierarchy: *mut *mut root::Il2CppClass,
        pub unity_user_data: *mut ::std::os::raw::c_void,
        pub initializationExceptionGCHandle: u32,
        pub cctor_started: u32,
        pub cctor_finished_or_no_cctor: u32,
        pub cctor_thread: usize,
        pub genericContainerHandle: root::Il2CppMetadataGenericContainerHandle,
        pub instance_size: u32,
        pub stack_slot_size: u32,
        pub actualSize: u32,
        pub element_size: u32,
        pub native_size: i32,
        pub static_fields_size: u32,
        pub thread_static_fields_size: u32,
        pub thread_static_fields_offset: i32,
        pub flags: u32,
        pub token: u32,
        pub method_count: u16,
        pub property_count: u16,
        pub field_count: u16,
        pub event_count: u16,
        pub nested_type_count: u16,
        pub vtable_count: u16,
        pub interfaces_count: u16,
        pub interface_offsets_count: u16,
        pub typeHierarchyDepth: u8,
        pub genericRecursionDepth: u8,
        pub rank: u8,
        pub minimumAlignment: u8,
        pub packingSize: u8,
        pub _bitfield_align_1: [u8; 0],
        pub _bitfield_1: root::__BindgenBitfieldUnit<[u8; 2usize]>,
        pub vtable: [root::VirtualInvokeData; 32usize],
    }
    impl Default for Il2CppClass {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppClass {{ image: {:?}, gc_desc: {:?}, name: {:?}, namespaze: {:?}, byval_arg: {:?}, this_arg: {:?}, element_class: {:?}, castClass: {:?}, declaringType: {:?}, parent: {:?}, generic_class: {:?}, typeMetadataHandle: {:?}, interopData: {:?}, klass: {:?}, fields: {:?}, events: {:?}, properties: {:?}, methods: {:?}, nestedTypes: {:?}, implementedInterfaces: {:?}, interfaceOffsets: {:?}, static_fields: {:?}, rgctx_data: {:?}, typeHierarchy: {:?}, unity_user_data: {:?}, initializationExceptionGCHandle: {:?}, cctor_started: {:?}, cctor_finished_or_no_cctor: {:?}, cctor_thread: {:?}, genericContainerHandle: {:?}, instance_size: {:?}, stack_slot_size: {:?}, actualSize: {:?}, element_size: {:?}, native_size: {:?}, static_fields_size: {:?}, thread_static_fields_size: {:?}, thread_static_fields_offset: {:?}, flags: {:?}, token: {:?}, method_count: {:?}, property_count: {:?}, field_count: {:?}, event_count: {:?}, nested_type_count: {:?}, vtable_count: {:?}, interfaces_count: {:?}, interface_offsets_count: {:?}, typeHierarchyDepth: {:?}, genericRecursionDepth: {:?}, rank: {:?}, minimumAlignment: {:?}, packingSize: {:?}, initialized_and_no_error : {:?}, initialized : {:?}, enumtype : {:?}, nullabletype : {:?}, is_generic : {:?}, has_references : {:?}, init_pending : {:?}, size_init_pending : {:?}, size_inited : {:?}, has_finalize : {:?}, has_cctor : {:?}, is_blittable : {:?}, is_import_or_windows_runtime : {:?}, is_vtable_initialized : {:?}, is_byref_like : {:?}, vtable: {:?} }}" , self . image , self . gc_desc , self . name , self . namespaze , self . byval_arg , self . this_arg , self . element_class , self . castClass , self . declaringType , self . parent , self . generic_class , self . typeMetadataHandle , self . interopData , self . klass , self . fields , self . events , self . properties , self . methods , self . nestedTypes , self . implementedInterfaces , self . interfaceOffsets , self . static_fields , self . rgctx_data , self . typeHierarchy , self . unity_user_data , self . initializationExceptionGCHandle , self . cctor_started , self . cctor_finished_or_no_cctor , self . cctor_thread , self . genericContainerHandle , self . instance_size , self . stack_slot_size , self . actualSize , self . element_size , self . native_size , self . static_fields_size , self . thread_static_fields_size , self . thread_static_fields_offset , self . flags , self . token , self . method_count , self . property_count , self . field_count , self . event_count , self . nested_type_count , self . vtable_count , self . interfaces_count , self . interface_offsets_count , self . typeHierarchyDepth , self . genericRecursionDepth , self . rank , self . minimumAlignment , self . packingSize , self . initialized_and_no_error () , self . initialized () , self . enumtype () , self . nullabletype () , self . is_generic () , self . has_references () , self . init_pending () , self . size_init_pending () , self . size_inited () , self . has_finalize () , self . has_cctor () , self . is_blittable () , self . is_import_or_windows_runtime () , self . is_vtable_initialized () , self . is_byref_like () , self . vtable)
        }
    }
    impl Il2CppClass {
        #[inline]
        pub fn initialized_and_no_error(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_initialized_and_no_error(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(0usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn initialized_and_no_error_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    0usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_initialized_and_no_error_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    0usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn initialized(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_initialized(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(1usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn initialized_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    1usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_initialized_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    1usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn enumtype(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_enumtype(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn enumtype_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    2usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_enumtype_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    2usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn nullabletype(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_nullabletype(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn nullabletype_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    3usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_nullabletype_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    3usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn is_generic(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_is_generic(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn is_generic_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    4usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_is_generic_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    4usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn has_references(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_has_references(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn has_references_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    5usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_has_references_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    5usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn init_pending(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_init_pending(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn init_pending_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    6usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_init_pending_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    6usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn size_init_pending(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_size_init_pending(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn size_init_pending_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    7usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_size_init_pending_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    7usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn size_inited(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_size_inited(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(8usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn size_inited_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    8usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_size_inited_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    8usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn has_finalize(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_has_finalize(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(9usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn has_finalize_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    9usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_has_finalize_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    9usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn has_cctor(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_has_cctor(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(10usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn has_cctor_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    10usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_has_cctor_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    10usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn is_blittable(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_is_blittable(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(11usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn is_blittable_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    11usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_is_blittable_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    11usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn is_import_or_windows_runtime(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_is_import_or_windows_runtime(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(12usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn is_import_or_windows_runtime_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    12usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_is_import_or_windows_runtime_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    12usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn is_vtable_initialized(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_is_vtable_initialized(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(13usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn is_vtable_initialized_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    13usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_is_vtable_initialized_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    13usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn is_byref_like(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_is_byref_like(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(14usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn is_byref_like_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    14usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_is_byref_like_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    14usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn new_bitfield_1(
            initialized_and_no_error: u8,
            initialized: u8,
            enumtype: u8,
            nullabletype: u8,
            is_generic: u8,
            has_references: u8,
            init_pending: u8,
            size_init_pending: u8,
            size_inited: u8,
            has_finalize: u8,
            has_cctor: u8,
            is_blittable: u8,
            is_import_or_windows_runtime: u8,
            is_vtable_initialized: u8,
            is_byref_like: u8,
        ) -> root::__BindgenBitfieldUnit<[u8; 2usize]> {
            let mut __bindgen_bitfield_unit: root::__BindgenBitfieldUnit<[u8; 2usize]> =
                Default::default();
            __bindgen_bitfield_unit.set(0usize, 1u8, {
                let initialized_and_no_error: u8 =
                    unsafe { ::std::mem::transmute(initialized_and_no_error) };
                initialized_and_no_error as u64
            });
            __bindgen_bitfield_unit.set(1usize, 1u8, {
                let initialized: u8 = unsafe { ::std::mem::transmute(initialized) };
                initialized as u64
            });
            __bindgen_bitfield_unit.set(2usize, 1u8, {
                let enumtype: u8 = unsafe { ::std::mem::transmute(enumtype) };
                enumtype as u64
            });
            __bindgen_bitfield_unit.set(3usize, 1u8, {
                let nullabletype: u8 = unsafe { ::std::mem::transmute(nullabletype) };
                nullabletype as u64
            });
            __bindgen_bitfield_unit.set(4usize, 1u8, {
                let is_generic: u8 = unsafe { ::std::mem::transmute(is_generic) };
                is_generic as u64
            });
            __bindgen_bitfield_unit.set(5usize, 1u8, {
                let has_references: u8 = unsafe { ::std::mem::transmute(has_references) };
                has_references as u64
            });
            __bindgen_bitfield_unit.set(6usize, 1u8, {
                let init_pending: u8 = unsafe { ::std::mem::transmute(init_pending) };
                init_pending as u64
            });
            __bindgen_bitfield_unit.set(7usize, 1u8, {
                let size_init_pending: u8 = unsafe { ::std::mem::transmute(size_init_pending) };
                size_init_pending as u64
            });
            __bindgen_bitfield_unit.set(8usize, 1u8, {
                let size_inited: u8 = unsafe { ::std::mem::transmute(size_inited) };
                size_inited as u64
            });
            __bindgen_bitfield_unit.set(9usize, 1u8, {
                let has_finalize: u8 = unsafe { ::std::mem::transmute(has_finalize) };
                has_finalize as u64
            });
            __bindgen_bitfield_unit.set(10usize, 1u8, {
                let has_cctor: u8 = unsafe { ::std::mem::transmute(has_cctor) };
                has_cctor as u64
            });
            __bindgen_bitfield_unit.set(11usize, 1u8, {
                let is_blittable: u8 = unsafe { ::std::mem::transmute(is_blittable) };
                is_blittable as u64
            });
            __bindgen_bitfield_unit.set(12usize, 1u8, {
                let is_import_or_windows_runtime: u8 =
                    unsafe { ::std::mem::transmute(is_import_or_windows_runtime) };
                is_import_or_windows_runtime as u64
            });
            __bindgen_bitfield_unit.set(13usize, 1u8, {
                let is_vtable_initialized: u8 =
                    unsafe { ::std::mem::transmute(is_vtable_initialized) };
                is_vtable_initialized as u64
            });
            __bindgen_bitfield_unit.set(14usize, 1u8, {
                let is_byref_like: u8 = unsafe { ::std::mem::transmute(is_byref_like) };
                is_byref_like as u64
            });
            __bindgen_bitfield_unit
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppClass_0 {
        pub image: *const root::Il2CppImage,
        pub gc_desc: *mut ::std::os::raw::c_void,
        pub name: *const ::std::os::raw::c_char,
        pub namespaze: *const ::std::os::raw::c_char,
        pub byval_arg: root::Il2CppType,
        pub this_arg: root::Il2CppType,
        pub element_class: *mut root::Il2CppClass,
        pub castClass: *mut root::Il2CppClass,
        pub declaringType: *mut root::Il2CppClass,
        pub parent: *mut root::Il2CppClass,
        pub generic_class: *mut root::Il2CppGenericClass,
        pub typeMetadataHandle: root::Il2CppMetadataTypeHandle,
        pub interopData: *const root::Il2CppInteropData,
        pub klass: *mut root::Il2CppClass,
        pub fields: *mut root::FieldInfo,
        pub events: *const root::EventInfo,
        pub properties: *const root::PropertyInfo,
        pub methods: *mut *const root::MethodInfo,
        pub nestedTypes: *mut *mut root::Il2CppClass,
        pub implementedInterfaces: *mut *mut root::Il2CppClass,
    }
    impl Default for Il2CppClass_0 {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppClass_0 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppClass_0 {{ image: {:?}, gc_desc: {:?}, name: {:?}, namespaze: {:?}, byval_arg: {:?}, this_arg: {:?}, element_class: {:?}, castClass: {:?}, declaringType: {:?}, parent: {:?}, generic_class: {:?}, typeMetadataHandle: {:?}, interopData: {:?}, klass: {:?}, fields: {:?}, events: {:?}, properties: {:?}, methods: {:?}, nestedTypes: {:?}, implementedInterfaces: {:?} }}" , self . image , self . gc_desc , self . name , self . namespaze , self . byval_arg , self . this_arg , self . element_class , self . castClass , self . declaringType , self . parent , self . generic_class , self . typeMetadataHandle , self . interopData , self . klass , self . fields , self . events , self . properties , self . methods , self . nestedTypes , self . implementedInterfaces)
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppClass_1 {
        pub typeHierarchy: *mut *mut root::Il2CppClass,
        pub unity_user_data: *mut ::std::os::raw::c_void,
        pub initializationExceptionGCHandle: u32,
        pub cctor_started: u32,
        pub cctor_finished_or_no_cctor: u32,
        pub cctor_thread: usize,
        pub genericContainerHandle: root::Il2CppMetadataGenericContainerHandle,
        pub instance_size: u32,
        pub stack_slot_size: u32,
        pub actualSize: u32,
        pub element_size: u32,
        pub native_size: i32,
        pub static_fields_size: u32,
        pub thread_static_fields_size: u32,
        pub thread_static_fields_offset: i32,
        pub flags: u32,
        pub token: u32,
        pub method_count: u16,
        pub property_count: u16,
        pub field_count: u16,
        pub event_count: u16,
        pub nested_type_count: u16,
        pub vtable_count: u16,
        pub interfaces_count: u16,
        pub interface_offsets_count: u16,
        pub typeHierarchyDepth: u8,
        pub genericRecursionDepth: u8,
        pub rank: u8,
        pub minimumAlignment: u8,
        pub packingSize: u8,
        pub _bitfield_align_1: [u8; 0],
        pub _bitfield_1: root::__BindgenBitfieldUnit<[u8; 2usize]>,
        pub __bindgen_padding_0: u8,
    }
    impl Default for Il2CppClass_1 {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl Il2CppClass_1 {
        #[inline]
        pub fn initialized_and_no_error(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_initialized_and_no_error(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(0usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn initialized_and_no_error_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    0usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_initialized_and_no_error_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    0usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn initialized(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_initialized(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(1usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn initialized_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    1usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_initialized_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    1usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn enumtype(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_enumtype(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn enumtype_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    2usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_enumtype_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    2usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn nullabletype(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_nullabletype(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn nullabletype_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    3usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_nullabletype_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    3usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn is_generic(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_is_generic(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn is_generic_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    4usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_is_generic_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    4usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn has_references(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_has_references(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn has_references_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    5usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_has_references_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    5usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn init_pending(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_init_pending(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn init_pending_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    6usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_init_pending_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    6usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn size_init_pending(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_size_init_pending(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn size_init_pending_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    7usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_size_init_pending_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    7usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn size_inited(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_size_inited(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(8usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn size_inited_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    8usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_size_inited_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    8usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn has_finalize(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_has_finalize(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(9usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn has_finalize_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    9usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_has_finalize_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    9usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn has_cctor(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_has_cctor(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(10usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn has_cctor_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    10usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_has_cctor_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    10usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn is_blittable(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_is_blittable(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(11usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn is_blittable_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    11usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_is_blittable_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    11usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn is_import_or_windows_runtime(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_is_import_or_windows_runtime(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(12usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn is_import_or_windows_runtime_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    12usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_is_import_or_windows_runtime_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    12usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn is_vtable_initialized(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_is_vtable_initialized(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(13usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn is_vtable_initialized_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    13usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_is_vtable_initialized_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    13usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn is_byref_like(&self) -> u8 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u8) }
        }
        #[inline]
        pub fn set_is_byref_like(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                self._bitfield_1.set(14usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn is_byref_like_raw(this: *const Self) -> u8 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    14usize,
                    1u8,
                ) as u8)
            }
        }
        #[inline]
        pub unsafe fn set_is_byref_like_raw(this: *mut Self, val: u8) {
            unsafe {
                let val: u8 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    14usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn new_bitfield_1(
            initialized_and_no_error: u8,
            initialized: u8,
            enumtype: u8,
            nullabletype: u8,
            is_generic: u8,
            has_references: u8,
            init_pending: u8,
            size_init_pending: u8,
            size_inited: u8,
            has_finalize: u8,
            has_cctor: u8,
            is_blittable: u8,
            is_import_or_windows_runtime: u8,
            is_vtable_initialized: u8,
            is_byref_like: u8,
        ) -> root::__BindgenBitfieldUnit<[u8; 2usize]> {
            let mut __bindgen_bitfield_unit: root::__BindgenBitfieldUnit<[u8; 2usize]> =
                Default::default();
            __bindgen_bitfield_unit.set(0usize, 1u8, {
                let initialized_and_no_error: u8 =
                    unsafe { ::std::mem::transmute(initialized_and_no_error) };
                initialized_and_no_error as u64
            });
            __bindgen_bitfield_unit.set(1usize, 1u8, {
                let initialized: u8 = unsafe { ::std::mem::transmute(initialized) };
                initialized as u64
            });
            __bindgen_bitfield_unit.set(2usize, 1u8, {
                let enumtype: u8 = unsafe { ::std::mem::transmute(enumtype) };
                enumtype as u64
            });
            __bindgen_bitfield_unit.set(3usize, 1u8, {
                let nullabletype: u8 = unsafe { ::std::mem::transmute(nullabletype) };
                nullabletype as u64
            });
            __bindgen_bitfield_unit.set(4usize, 1u8, {
                let is_generic: u8 = unsafe { ::std::mem::transmute(is_generic) };
                is_generic as u64
            });
            __bindgen_bitfield_unit.set(5usize, 1u8, {
                let has_references: u8 = unsafe { ::std::mem::transmute(has_references) };
                has_references as u64
            });
            __bindgen_bitfield_unit.set(6usize, 1u8, {
                let init_pending: u8 = unsafe { ::std::mem::transmute(init_pending) };
                init_pending as u64
            });
            __bindgen_bitfield_unit.set(7usize, 1u8, {
                let size_init_pending: u8 = unsafe { ::std::mem::transmute(size_init_pending) };
                size_init_pending as u64
            });
            __bindgen_bitfield_unit.set(8usize, 1u8, {
                let size_inited: u8 = unsafe { ::std::mem::transmute(size_inited) };
                size_inited as u64
            });
            __bindgen_bitfield_unit.set(9usize, 1u8, {
                let has_finalize: u8 = unsafe { ::std::mem::transmute(has_finalize) };
                has_finalize as u64
            });
            __bindgen_bitfield_unit.set(10usize, 1u8, {
                let has_cctor: u8 = unsafe { ::std::mem::transmute(has_cctor) };
                has_cctor as u64
            });
            __bindgen_bitfield_unit.set(11usize, 1u8, {
                let is_blittable: u8 = unsafe { ::std::mem::transmute(is_blittable) };
                is_blittable as u64
            });
            __bindgen_bitfield_unit.set(12usize, 1u8, {
                let is_import_or_windows_runtime: u8 =
                    unsafe { ::std::mem::transmute(is_import_or_windows_runtime) };
                is_import_or_windows_runtime as u64
            });
            __bindgen_bitfield_unit.set(13usize, 1u8, {
                let is_vtable_initialized: u8 =
                    unsafe { ::std::mem::transmute(is_vtable_initialized) };
                is_vtable_initialized as u64
            });
            __bindgen_bitfield_unit.set(14usize, 1u8, {
                let is_byref_like: u8 = unsafe { ::std::mem::transmute(is_byref_like) };
                is_byref_like as u64
            });
            __bindgen_bitfield_unit
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppClass_Merged {
        pub _0: root::Il2CppClass_0,
        pub interfaceOffsets: *mut root::Il2CppRuntimeInterfaceOffsetPair,
        pub static_fields: *mut ::std::os::raw::c_void,
        pub rgctx_data: *const root::Il2CppRGCTXData,
        pub _1: root::Il2CppClass_1,
        pub vtable: [root::VirtualInvokeData; 32usize],
    }
    impl Default for Il2CppClass_Merged {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppClass_Merged {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppClass_Merged {{ _0: {:?}, interfaceOffsets: {:?}, static_fields: {:?}, rgctx_data: {:?}, _1: {:?}, vtable: {:?} }}" , self . _0 , self . interfaceOffsets , self . static_fields , self . rgctx_data , self . _1 , self . vtable)
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppTypeDefinitionSizes {
        pub instance_size: u32,
        pub native_size: i32,
        pub static_fields_size: u32,
        pub thread_static_fields_size: u32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppDomain {
        pub domain: *mut root::Il2CppAppDomain,
        pub setup: *mut root::Il2CppAppDomainSetup,
        pub default_context: *mut root::Il2CppAppContext,
        pub ephemeron_tombstone: *mut root::Il2CppObject,
        pub friendly_name: *const ::std::os::raw::c_char,
        pub domain_id: u32,
        pub threadpool_jobs: ::std::os::raw::c_int,
        pub agent_info: *mut ::std::os::raw::c_void,
    }
    impl Default for Il2CppDomain {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppAssemblyName {
        pub name: *const ::std::os::raw::c_char,
        pub culture: *const ::std::os::raw::c_char,
        pub public_key: *const u8,
        pub hash_alg: u32,
        pub hash_len: i32,
        pub flags: u32,
        pub major: i32,
        pub minor: i32,
        pub build: i32,
        pub revision: i32,
        pub public_key_token: [u8; 8usize],
    }
    impl Default for Il2CppAssemblyName {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppImage {
        pub name: *const ::std::os::raw::c_char,
        pub nameNoExt: *const ::std::os::raw::c_char,
        pub assembly: *mut root::Il2CppAssembly,
        pub typeCount: u32,
        pub exportedTypeCount: u32,
        pub customAttributeCount: u32,
        pub metadataHandle: root::Il2CppMetadataImageHandle,
        pub nameToClassHashTable: *mut root::Il2CppNameToTypeHandleHashTable,
        pub codeGenModule: *const root::Il2CppCodeGenModule,
        pub token: u32,
        pub dynamic: u8,
    }
    impl Default for Il2CppImage {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppAssembly {
        pub image: *mut root::Il2CppImage,
        pub token: u32,
        pub referencedAssemblyStart: i32,
        pub referencedAssemblyCount: i32,
        pub aname: root::Il2CppAssemblyName,
    }
    impl Default for Il2CppAssembly {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppCodeGenOptions {
        pub enablePrimitiveValueTypeGenericSharing: u8,
        pub maximumRuntimeGenericDepth: ::std::os::raw::c_int,
        pub recursiveGenericIterations: ::std::os::raw::c_int,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppRange {
        pub start: i32,
        pub length: i32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppTokenRangePair {
        pub token: u32,
        pub range: root::Il2CppRange,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppTokenIndexMethodTuple {
        pub token: u32,
        pub index: i32,
        pub method: *mut *mut ::std::os::raw::c_void,
        pub __genericMethodIndex: u32,
    }
    impl Default for Il2CppTokenIndexMethodTuple {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppTokenAdjustorThunkPair {
        pub token: u32,
        pub adjustorThunk: root::Il2CppMethodPointer,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppWindowsRuntimeFactoryTableEntry {
        pub type_: *const root::Il2CppType,
        pub createFactoryFunction: root::Il2CppMethodPointer,
    }
    impl Default for Il2CppWindowsRuntimeFactoryTableEntry {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppCodeGenModule {
        pub moduleName: *const ::std::os::raw::c_char,
        pub methodPointerCount: u32,
        pub methodPointers: *const root::Il2CppMethodPointer,
        pub adjustorThunkCount: u32,
        pub adjustorThunks: *const root::Il2CppTokenAdjustorThunkPair,
        pub invokerIndices: *const i32,
        pub reversePInvokeWrapperCount: u32,
        pub reversePInvokeWrapperIndices: *const root::Il2CppTokenIndexMethodTuple,
        pub rgctxRangesCount: u32,
        pub rgctxRanges: *const root::Il2CppTokenRangePair,
        pub rgctxsCount: u32,
        pub rgctxs: *const root::Il2CppRGCTXDefinition,
        pub debuggerMetadata: *const root::Il2CppDebuggerMetadataRegistration,
        pub moduleInitializer: root::Il2CppMethodPointer,
        pub staticConstructorTypeIndices: *mut root::TypeDefinitionIndex,
        pub metadataRegistration: *const root::Il2CppMetadataRegistration,
        pub codeRegistaration: *const root::Il2CppCodeRegistration,
    }
    impl Default for Il2CppCodeGenModule {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppCodeRegistration {
        pub reversePInvokeWrapperCount: u32,
        pub reversePInvokeWrappers: *const root::Il2CppMethodPointer,
        pub genericMethodPointersCount: u32,
        pub genericMethodPointers: *const root::Il2CppMethodPointer,
        pub genericAdjustorThunks: *const root::Il2CppMethodPointer,
        pub invokerPointersCount: u32,
        pub invokerPointers: *const root::InvokerMethod,
        pub unresolvedIndirectCallCount: u32,
        pub unresolvedVirtualCallPointers: *const root::Il2CppMethodPointer,
        pub unresolvedInstanceCallPointers: *const root::Il2CppMethodPointer,
        pub unresolvedStaticCallPointers: *const root::Il2CppMethodPointer,
        pub interopDataCount: u32,
        pub interopData: *mut root::Il2CppInteropData,
        pub windowsRuntimeFactoryCount: u32,
        pub windowsRuntimeFactoryTable: *mut root::Il2CppWindowsRuntimeFactoryTableEntry,
        pub codeGenModulesCount: u32,
        pub codeGenModules: *mut *const root::Il2CppCodeGenModule,
    }
    impl Default for Il2CppCodeRegistration {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppMetadataRegistration {
        pub genericClassesCount: i32,
        pub genericClasses: *const *mut root::Il2CppGenericClass,
        pub genericInstsCount: i32,
        pub genericInsts: *const *const root::Il2CppGenericInst,
        pub genericMethodTableCount: i32,
        pub genericMethodTable: *const root::Il2CppGenericMethodFunctionsDefinitions,
        pub typesCount: i32,
        pub types: *const *const root::Il2CppType,
        pub methodSpecsCount: i32,
        pub methodSpecs: *const root::Il2CppMethodSpec,
        pub fieldOffsetsCount: root::FieldIndex,
        pub fieldOffsets: *mut *const i32,
        pub typeDefinitionsSizesCount: root::TypeDefinitionIndex,
        pub typeDefinitionsSizes: *mut *const root::Il2CppTypeDefinitionSizes,
        pub metadataUsagesCount: usize,
        pub metadataUsages: *const *mut *mut ::std::os::raw::c_void,
    }
    impl Default for Il2CppMetadataRegistration {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppPerfCounters {
        pub jit_methods: u32,
        pub jit_bytes: u32,
        pub jit_time: u32,
        pub jit_failures: u32,
        pub exceptions_thrown: u32,
        pub exceptions_filters: u32,
        pub exceptions_finallys: u32,
        pub exceptions_depth: u32,
        pub aspnet_requests_queued: u32,
        pub aspnet_requests: u32,
        pub gc_collections0: u32,
        pub gc_collections1: u32,
        pub gc_collections2: u32,
        pub gc_promotions0: u32,
        pub gc_promotions1: u32,
        pub gc_promotion_finalizers: u32,
        pub gc_gen0size: u32,
        pub gc_gen1size: u32,
        pub gc_gen2size: u32,
        pub gc_lossize: u32,
        pub gc_fin_survivors: u32,
        pub gc_num_handles: u32,
        pub gc_allocated: u32,
        pub gc_induced: u32,
        pub gc_time: u32,
        pub gc_total_bytes: u32,
        pub gc_committed_bytes: u32,
        pub gc_reserved_bytes: u32,
        pub gc_num_pinned: u32,
        pub gc_sync_blocks: u32,
        pub remoting_calls: u32,
        pub remoting_channels: u32,
        pub remoting_proxies: u32,
        pub remoting_classes: u32,
        pub remoting_objects: u32,
        pub remoting_contexts: u32,
        pub loader_classes: u32,
        pub loader_total_classes: u32,
        pub loader_appdomains: u32,
        pub loader_total_appdomains: u32,
        pub loader_assemblies: u32,
        pub loader_total_assemblies: u32,
        pub loader_failures: u32,
        pub loader_bytes: u32,
        pub loader_appdomains_uloaded: u32,
        pub thread_contentions: u32,
        pub thread_queue_len: u32,
        pub thread_queue_max: u32,
        pub thread_num_logical: u32,
        pub thread_num_physical: u32,
        pub thread_cur_recognized: u32,
        pub thread_num_recognized: u32,
        pub interop_num_ccw: u32,
        pub interop_num_stubs: u32,
        pub interop_num_marshals: u32,
        pub security_num_checks: u32,
        pub security_num_link_checks: u32,
        pub security_time: u32,
        pub security_depth: u32,
        pub unused: u32,
        pub threadpool_workitems: u64,
        pub threadpool_ioworkitems: u64,
        pub threadpool_threads: ::std::os::raw::c_uint,
        pub threadpool_iothreads: ::std::os::raw::c_uint,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppWaitHandle {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct MonitorData {
        _unused: [u8; 0],
    }
    pub type Il2CppVTable = root::Il2CppClass;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppObject {
        pub __bindgen_anon_1: root::Il2CppObject__bindgen_ty_1,
        pub monitor: *mut root::MonitorData,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Il2CppObject__bindgen_ty_1 {
        pub klass: *mut root::Il2CppClass,
        pub vtable: *mut root::Il2CppVTable,
    }
    impl Default for Il2CppObject__bindgen_ty_1 {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppObject__bindgen_ty_1 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppObject__bindgen_ty_1 {{ union }}")
        }
    }
    impl Default for Il2CppObject {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppObject {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppObject {{ __bindgen_anon_1: {:?}, monitor: {:?} }}",
                self.__bindgen_anon_1, self.monitor
            )
        }
    }
    pub type il2cpp_array_lower_bound_t = i32;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppArrayBounds {
        pub length: root::il2cpp_array_size_t,
        pub lower_bound: root::il2cpp_array_lower_bound_t,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppArray {
        pub obj: root::Il2CppObject,
        pub bounds: *mut root::Il2CppArrayBounds,
        pub max_length: root::il2cpp_array_size_t,
    }
    impl Default for Il2CppArray {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppArray {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppArray {{ obj: {:?}, bounds: {:?}, max_length: {:?} }}",
                self.obj, self.bounds, self.max_length
            )
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppArraySize {
        pub obj: root::Il2CppObject,
        pub bounds: *mut root::Il2CppArrayBounds,
        pub max_length: root::il2cpp_array_size_t,
        pub vector: [*mut ::std::os::raw::c_void; 32usize],
    }
    impl Default for Il2CppArraySize {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppArraySize {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppArraySize {{ obj: {:?}, bounds: {:?}, max_length: {:?}, vector: {:?} }}",
                self.obj, self.bounds, self.max_length, self.vector
            )
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppString {
        pub object: root::Il2CppObject,
        pub length: i32,
        pub chars: [root::Il2CppChar; 32usize],
    }
    impl Default for Il2CppString {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppString {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppString {{ object: {:?}, length: {:?}, chars: {:?} }}",
                self.object, self.length, self.chars
            )
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionType {
        pub object: root::Il2CppObject,
        pub type_: *const root::Il2CppType,
    }
    impl Default for Il2CppReflectionType {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppReflectionType {{ object: {:?}, type: {:?} }}",
                self.object, self.type_
            )
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionRuntimeType {
        pub type_: root::Il2CppReflectionType,
        pub type_info: *mut root::Il2CppObject,
        pub genericCache: *mut root::Il2CppObject,
        pub serializationCtor: *mut root::Il2CppObject,
    }
    impl Default for Il2CppReflectionRuntimeType {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionRuntimeType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppReflectionRuntimeType {{ type: {:?}, type_info: {:?}, genericCache: {:?}, serializationCtor: {:?} }}" , self . type_ , self . type_info , self . genericCache , self . serializationCtor)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionMonoType {
        pub type_: root::Il2CppReflectionRuntimeType,
    }
    impl Default for Il2CppReflectionMonoType {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionMonoType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppReflectionMonoType {{ type: {:?} }}", self.type_)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionEvent {
        pub object: root::Il2CppObject,
        pub cached_add_event: *mut root::Il2CppObject,
    }
    impl Default for Il2CppReflectionEvent {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionEvent {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppReflectionEvent {{ object: {:?}, cached_add_event: {:?} }}",
                self.object, self.cached_add_event
            )
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionMonoEvent {
        pub event: root::Il2CppReflectionEvent,
        pub reflectedType: *mut root::Il2CppReflectionType,
        pub eventInfo: *const root::EventInfo,
    }
    impl Default for Il2CppReflectionMonoEvent {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionMonoEvent {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppReflectionMonoEvent {{ event: {:?}, reflectedType: {:?}, eventInfo: {:?} }}",
                self.event, self.reflectedType, self.eventInfo
            )
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppReflectionMonoEventInfo {
        pub declaringType: *mut root::Il2CppReflectionType,
        pub reflectedType: *mut root::Il2CppReflectionType,
        pub name: *mut root::Il2CppString,
        pub addMethod: *mut root::Il2CppReflectionMethod,
        pub removeMethod: *mut root::Il2CppReflectionMethod,
        pub raiseMethod: *mut root::Il2CppReflectionMethod,
        pub eventAttributes: u32,
        pub otherMethods: *mut root::Il2CppArray,
    }
    impl Default for Il2CppReflectionMonoEventInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionField {
        pub object: root::Il2CppObject,
        pub klass: *mut root::Il2CppClass,
        pub field: *mut root::FieldInfo,
        pub name: *mut root::Il2CppString,
        pub type_: *mut root::Il2CppReflectionType,
        pub attrs: u32,
    }
    impl Default for Il2CppReflectionField {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionField {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppReflectionField {{ object: {:?}, klass: {:?}, field: {:?}, name: {:?}, type: {:?}, attrs: {:?} }}" , self . object , self . klass , self . field , self . name , self . type_ , self . attrs)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionProperty {
        pub object: root::Il2CppObject,
        pub klass: *mut root::Il2CppClass,
        pub property: *const root::PropertyInfo,
    }
    impl Default for Il2CppReflectionProperty {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionProperty {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppReflectionProperty {{ object: {:?}, klass: {:?}, property: {:?} }}",
                self.object, self.klass, self.property
            )
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionMethod {
        pub object: root::Il2CppObject,
        pub method: *const root::MethodInfo,
        pub name: *mut root::Il2CppString,
        pub reftype: *mut root::Il2CppReflectionType,
    }
    impl Default for Il2CppReflectionMethod {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionMethod {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppReflectionMethod {{ object: {:?}, method: {:?}, name: {:?}, reftype: {:?} }}" , self . object , self . method , self . name , self . reftype)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionGenericMethod {
        pub base: root::Il2CppReflectionMethod,
    }
    impl Default for Il2CppReflectionGenericMethod {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionGenericMethod {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppReflectionGenericMethod {{ base: {:?} }}",
                self.base
            )
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppMethodInfo {
        pub parent: *mut root::Il2CppReflectionType,
        pub ret: *mut root::Il2CppReflectionType,
        pub attrs: u32,
        pub implattrs: u32,
        pub callconv: u32,
    }
    impl Default for Il2CppMethodInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppPropertyInfo {
        pub parent: *mut root::Il2CppReflectionType,
        pub declaringType: *mut root::Il2CppReflectionType,
        pub name: *mut root::Il2CppString,
        pub get: *mut root::Il2CppReflectionMethod,
        pub set: *mut root::Il2CppReflectionMethod,
        pub attrs: u32,
    }
    impl Default for Il2CppPropertyInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionParameter {
        pub object: root::Il2CppObject,
        pub AttrsImpl: u32,
        pub ClassImpl: *mut root::Il2CppReflectionType,
        pub DefaultValueImpl: *mut root::Il2CppObject,
        pub MemberImpl: *mut root::Il2CppObject,
        pub NameImpl: *mut root::Il2CppString,
        pub PositionImpl: i32,
        pub MarshalAs: *mut root::Il2CppObject,
    }
    impl Default for Il2CppReflectionParameter {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionParameter {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppReflectionParameter {{ object: {:?}, AttrsImpl: {:?}, ClassImpl: {:?}, DefaultValueImpl: {:?}, MemberImpl: {:?}, NameImpl: {:?}, PositionImpl: {:?}, MarshalAs: {:?} }}" , self . object , self . AttrsImpl , self . ClassImpl , self . DefaultValueImpl , self . MemberImpl , self . NameImpl , self . PositionImpl , self . MarshalAs)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionModule {
        pub obj: root::Il2CppObject,
        pub image: *const root::Il2CppImage,
        pub assembly: *mut root::Il2CppReflectionAssembly,
        pub fqname: *mut root::Il2CppString,
        pub name: *mut root::Il2CppString,
        pub scopename: *mut root::Il2CppString,
        pub is_resource: u8,
        pub token: u32,
    }
    impl Default for Il2CppReflectionModule {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionModule {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppReflectionModule {{ obj: {:?}, image: {:?}, assembly: {:?}, fqname: {:?}, name: {:?}, scopename: {:?}, is_resource: {:?}, token: {:?} }}" , self . obj , self . image , self . assembly , self . fqname , self . name , self . scopename , self . is_resource , self . token)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionAssemblyName {
        pub obj: root::Il2CppObject,
        pub name: *mut root::Il2CppString,
        pub codebase: *mut root::Il2CppString,
        pub major: i32,
        pub minor: i32,
        pub build: i32,
        pub revision: i32,
        pub cultureInfo: *mut root::Il2CppObject,
        pub flags: u32,
        pub hashalg: u32,
        pub keypair: *mut root::Il2CppObject,
        pub publicKey: *mut root::Il2CppArray,
        pub keyToken: *mut root::Il2CppArray,
        pub versioncompat: u32,
        pub version: *mut root::Il2CppObject,
        pub processor_architecture: u32,
        pub contentType: u32,
    }
    impl Default for Il2CppReflectionAssemblyName {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionAssemblyName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppReflectionAssemblyName {{ obj: {:?}, name: {:?}, codebase: {:?}, major: {:?}, minor: {:?}, build: {:?}, revision: {:?}, cultureInfo: {:?}, flags: {:?}, hashalg: {:?}, keypair: {:?}, publicKey: {:?}, keyToken: {:?}, versioncompat: {:?}, version: {:?}, processor_architecture: {:?}, contentType: {:?} }}" , self . obj , self . name , self . codebase , self . major , self . minor , self . build , self . revision , self . cultureInfo , self . flags , self . hashalg , self . keypair , self . publicKey , self . keyToken , self . versioncompat , self . version , self . processor_architecture , self . contentType)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionAssembly {
        pub object: root::Il2CppObject,
        pub assembly: *const root::Il2CppAssembly,
        pub evidence: *mut root::Il2CppObject,
        pub resolve_event_holder: *mut root::Il2CppObject,
        pub minimum: *mut root::Il2CppObject,
        pub optional: *mut root::Il2CppObject,
        pub refuse: *mut root::Il2CppObject,
        pub granted: *mut root::Il2CppObject,
        pub denied: *mut root::Il2CppObject,
        pub from_byte_array: u8,
        pub name: *mut root::Il2CppString,
    }
    impl Default for Il2CppReflectionAssembly {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionAssembly {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppReflectionAssembly {{ object: {:?}, assembly: {:?}, evidence: {:?}, resolve_event_holder: {:?}, minimum: {:?}, optional: {:?}, refuse: {:?}, granted: {:?}, denied: {:?}, from_byte_array: {:?}, name: {:?} }}" , self . object , self . assembly , self . evidence , self . resolve_event_holder , self . minimum , self . optional , self . refuse , self . granted , self . denied , self . from_byte_array , self . name)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionMarshal {
        pub object: root::Il2CppObject,
        pub count: i32,
        pub type_: i32,
        pub eltype: i32,
        pub guid: *mut root::Il2CppString,
        pub mcookie: *mut root::Il2CppString,
        pub marshaltype: *mut root::Il2CppString,
        pub marshaltyperef: *mut root::Il2CppObject,
        pub param_num: i32,
        pub has_size: u8,
    }
    impl Default for Il2CppReflectionMarshal {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionMarshal {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppReflectionMarshal {{ object: {:?}, count: {:?}, type: {:?}, eltype: {:?}, guid: {:?}, mcookie: {:?}, marshaltype: {:?}, marshaltyperef: {:?}, param_num: {:?}, has_size: {:?} }}" , self . object , self . count , self . type_ , self . eltype , self . guid , self . mcookie , self . marshaltype , self . marshaltyperef , self . param_num , self . has_size)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppReflectionPointer {
        pub object: root::Il2CppObject,
        pub data: *mut ::std::os::raw::c_void,
        pub type_: *mut root::Il2CppReflectionType,
    }
    impl Default for Il2CppReflectionPointer {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppReflectionPointer {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppReflectionPointer {{ object: {:?}, data: {:?}, type: {:?} }}",
                self.object, self.data, self.type_
            )
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppThreadName {
        pub chars: *mut root::Il2CppChar,
        pub unused: i32,
        pub length: i32,
    }
    impl Default for Il2CppThreadName {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppRefCount {
        pub ref_: u32,
        pub destructor:
            ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void)>,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppLongLivedThreadData {
        pub ref_: root::Il2CppRefCount,
        pub synch_cs: *mut ::std::os::raw::c_void,
    }
    impl Default for Il2CppLongLivedThreadData {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppInternalThread {
        pub obj: root::Il2CppObject,
        pub lock_thread_id: ::std::os::raw::c_int,
        pub handle: *mut ::std::os::raw::c_void,
        pub native_handle: *mut ::std::os::raw::c_void,
        pub name: root::Il2CppThreadName,
        pub state: u32,
        pub abort_exc: *mut root::Il2CppObject,
        pub abort_state_handle: ::std::os::raw::c_int,
        pub tid: u64,
        pub debugger_thread: isize,
        pub static_data: *mut ::std::os::raw::c_void,
        pub runtime_thread_info: *mut ::std::os::raw::c_void,
        pub current_appcontext: *mut root::Il2CppObject,
        pub root_domain_thread: *mut root::Il2CppObject,
        pub _serialized_principal: *mut root::Il2CppArray,
        pub _serialized_principal_version: ::std::os::raw::c_int,
        pub appdomain_refs: *mut ::std::os::raw::c_void,
        pub interruption_requested: i32,
        pub longlived: *mut ::std::os::raw::c_void,
        pub threadpool_thread: u8,
        pub thread_interrupt_requested: u8,
        pub stack_size: ::std::os::raw::c_int,
        pub apartment_state: u8,
        pub critical_region_level: ::std::os::raw::c_int,
        pub managed_id: ::std::os::raw::c_int,
        pub small_id: u32,
        pub manage_callback: *mut ::std::os::raw::c_void,
        pub flags: isize,
        pub thread_pinning_ref: *mut ::std::os::raw::c_void,
        pub abort_protected_block_count: *mut ::std::os::raw::c_void,
        pub priority: i32,
        pub owned_mutexes: *mut ::std::os::raw::c_void,
        pub suspended: *mut ::std::os::raw::c_void,
        pub self_suspended: i32,
        pub thread_state: usize,
        pub unused: [*mut ::std::os::raw::c_void; 3usize],
        pub last: *mut ::std::os::raw::c_void,
    }
    impl Default for Il2CppInternalThread {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppInternalThread {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppInternalThread {{ obj: {:?}, lock_thread_id: {:?}, handle: {:?}, native_handle: {:?}, name: {:?}, state: {:?}, abort_exc: {:?}, abort_state_handle: {:?}, tid: {:?}, debugger_thread: {:?}, static_data: {:?}, runtime_thread_info: {:?}, current_appcontext: {:?}, root_domain_thread: {:?}, _serialized_principal: {:?}, _serialized_principal_version: {:?}, appdomain_refs: {:?}, interruption_requested: {:?}, longlived: {:?}, threadpool_thread: {:?}, thread_interrupt_requested: {:?}, stack_size: {:?}, apartment_state: {:?}, critical_region_level: {:?}, managed_id: {:?}, small_id: {:?}, manage_callback: {:?}, flags: {:?}, thread_pinning_ref: {:?}, abort_protected_block_count: {:?}, priority: {:?}, owned_mutexes: {:?}, suspended: {:?}, self_suspended: {:?}, thread_state: {:?}, unused: {:?}, last: {:?} }}" , self . obj , self . lock_thread_id , self . handle , self . native_handle , self . name , self . state , self . abort_exc , self . abort_state_handle , self . tid , self . debugger_thread , self . static_data , self . runtime_thread_info , self . current_appcontext , self . root_domain_thread , self . _serialized_principal , self . _serialized_principal_version , self . appdomain_refs , self . interruption_requested , self . longlived , self . threadpool_thread , self . thread_interrupt_requested , self . stack_size , self . apartment_state , self . critical_region_level , self . managed_id , self . small_id , self . manage_callback , self . flags , self . thread_pinning_ref , self . abort_protected_block_count , self . priority , self . owned_mutexes , self . suspended , self . self_suspended , self . thread_state , self . unused , self . last)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppIOSelectorJob {
        pub object: root::Il2CppObject,
        pub operation: i32,
        pub callback: *mut root::Il2CppObject,
        pub state: *mut root::Il2CppObject,
    }
    impl Default for Il2CppIOSelectorJob {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppIOSelectorJob {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppIOSelectorJob {{ object: {:?}, operation: {:?}, callback: {:?}, state: {:?} }}" , self . object , self . operation , self . callback , self . state)
        }
    }
    pub const Il2Cpp_CallType_Sync: Il2CppCallType = 0;
    pub const Il2Cpp_CallType_BeginInvoke: Il2CppCallType = 1;
    pub const Il2Cpp_CallType_EndInvoke: Il2CppCallType = 2;
    pub const Il2Cpp_CallType_OneWay: Il2CppCallType = 3;
    pub type Il2CppCallType = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppMethodMessage {
        pub obj: root::Il2CppObject,
        pub method: *mut root::Il2CppReflectionMethod,
        pub args: *mut root::Il2CppArray,
        pub names: *mut root::Il2CppArray,
        pub arg_types: *mut root::Il2CppArray,
        pub ctx: *mut root::Il2CppObject,
        pub rval: *mut root::Il2CppObject,
        pub exc: *mut root::Il2CppObject,
        pub async_result: *mut root::Il2CppAsyncResult,
        pub call_type: u32,
    }
    impl Default for Il2CppMethodMessage {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppMethodMessage {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppMethodMessage {{ obj: {:?}, method: {:?}, args: {:?}, names: {:?}, arg_types: {:?}, ctx: {:?}, rval: {:?}, exc: {:?}, async_result: {:?}, call_type: {:?} }}" , self . obj , self . method , self . args , self . names , self . arg_types , self . ctx , self . rval , self . exc , self . async_result , self . call_type)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppAppDomainSetup {
        pub object: root::Il2CppObject,
        pub application_base: *mut root::Il2CppString,
        pub application_name: *mut root::Il2CppString,
        pub cache_path: *mut root::Il2CppString,
        pub configuration_file: *mut root::Il2CppString,
        pub dynamic_base: *mut root::Il2CppString,
        pub license_file: *mut root::Il2CppString,
        pub private_bin_path: *mut root::Il2CppString,
        pub private_bin_path_probe: *mut root::Il2CppString,
        pub shadow_copy_directories: *mut root::Il2CppString,
        pub shadow_copy_files: *mut root::Il2CppString,
        pub publisher_policy: u8,
        pub path_changed: u8,
        pub loader_optimization: ::std::os::raw::c_int,
        pub disallow_binding_redirects: u8,
        pub disallow_code_downloads: u8,
        pub activation_arguments: *mut root::Il2CppObject,
        pub domain_initializer: *mut root::Il2CppObject,
        pub application_trust: *mut root::Il2CppObject,
        pub domain_initializer_args: *mut root::Il2CppArray,
        pub disallow_appbase_probe: u8,
        pub configuration_bytes: *mut root::Il2CppArray,
        pub serialized_non_primitives: *mut root::Il2CppArray,
    }
    impl Default for Il2CppAppDomainSetup {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppAppDomainSetup {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppAppDomainSetup {{ object: {:?}, application_base: {:?}, application_name: {:?}, cache_path: {:?}, configuration_file: {:?}, dynamic_base: {:?}, license_file: {:?}, private_bin_path: {:?}, private_bin_path_probe: {:?}, shadow_copy_directories: {:?}, shadow_copy_files: {:?}, publisher_policy: {:?}, path_changed: {:?}, loader_optimization: {:?}, disallow_binding_redirects: {:?}, disallow_code_downloads: {:?}, activation_arguments: {:?}, domain_initializer: {:?}, application_trust: {:?}, domain_initializer_args: {:?}, disallow_appbase_probe: {:?}, configuration_bytes: {:?}, serialized_non_primitives: {:?} }}" , self . object , self . application_base , self . application_name , self . cache_path , self . configuration_file , self . dynamic_base , self . license_file , self . private_bin_path , self . private_bin_path_probe , self . shadow_copy_directories , self . shadow_copy_files , self . publisher_policy , self . path_changed , self . loader_optimization , self . disallow_binding_redirects , self . disallow_code_downloads , self . activation_arguments , self . domain_initializer , self . application_trust , self . domain_initializer_args , self . disallow_appbase_probe , self . configuration_bytes , self . serialized_non_primitives)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppThread {
        pub obj: root::Il2CppObject,
        pub internal_thread: *mut root::Il2CppInternalThread,
        pub start_obj: *mut root::Il2CppObject,
        pub pending_exception: *mut root::Il2CppException,
        pub principal: *mut root::Il2CppObject,
        pub principal_version: i32,
        pub delegate: *mut root::Il2CppDelegate,
        pub executionContext: *mut root::Il2CppObject,
        pub executionContextBelongsToOuterScope: u8,
    }
    impl Default for Il2CppThread {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppThread {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppThread {{ obj: {:?}, internal_thread: {:?}, start_obj: {:?}, pending_exception: {:?}, principal: {:?}, principal_version: {:?}, delegate: {:?}, executionContext: {:?}, executionContextBelongsToOuterScope: {:?} }}" , self . obj , self . internal_thread , self . start_obj , self . pending_exception , self . principal , self . principal_version , self . delegate , self . executionContext , self . executionContextBelongsToOuterScope)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppException {
        pub object: root::Il2CppObject,
        pub className: *mut root::Il2CppString,
        pub message: *mut root::Il2CppString,
        pub _data: *mut root::Il2CppObject,
        pub inner_ex: *mut root::Il2CppException,
        pub _helpURL: *mut root::Il2CppString,
        pub trace_ips: *mut root::Il2CppArray,
        pub stack_trace: *mut root::Il2CppString,
        pub remote_stack_trace: *mut root::Il2CppString,
        pub remote_stack_index: ::std::os::raw::c_int,
        pub _dynamicMethods: *mut root::Il2CppObject,
        pub hresult: root::il2cpp_hresult_t,
        pub source: *mut root::Il2CppString,
        pub safeSerializationManager: *mut root::Il2CppObject,
        pub captured_traces: *mut root::Il2CppArray,
        pub native_trace_ips: *mut root::Il2CppArray,
        pub caught_in_unmanaged: i32,
    }
    impl Default for Il2CppException {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppException {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppException {{ object: {:?}, className: {:?}, message: {:?}, _data: {:?}, inner_ex: {:?}, _helpURL: {:?}, trace_ips: {:?}, stack_trace: {:?}, remote_stack_trace: {:?}, remote_stack_index: {:?}, _dynamicMethods: {:?}, hresult: {:?}, source: {:?}, safeSerializationManager: {:?}, captured_traces: {:?}, native_trace_ips: {:?}, caught_in_unmanaged: {:?} }}" , self . object , self . className , self . message , self . _data , self . inner_ex , self . _helpURL , self . trace_ips , self . stack_trace , self . remote_stack_trace , self . remote_stack_index , self . _dynamicMethods , self . hresult , self . source , self . safeSerializationManager , self . captured_traces , self . native_trace_ips , self . caught_in_unmanaged)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppSystemException {
        pub base: root::Il2CppException,
    }
    impl Default for Il2CppSystemException {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppSystemException {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppSystemException {{ base: {:?} }}", self.base)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppArgumentException {
        pub base: root::Il2CppException,
        pub argName: *mut root::Il2CppString,
    }
    impl Default for Il2CppArgumentException {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppArgumentException {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppArgumentException {{ base: {:?}, argName: {:?} }}",
                self.base, self.argName
            )
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppTypedRef {
        pub type_: *const root::Il2CppType,
        pub value: *mut ::std::os::raw::c_void,
        pub klass: *mut root::Il2CppClass,
    }
    impl Default for Il2CppTypedRef {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppDelegate {
        pub object: root::Il2CppObject,
        pub method_ptr: root::Il2CppMethodPointer,
        pub invoke_impl: root::Il2CppMethodPointer,
        pub target: *mut root::Il2CppObject,
        pub method: *const root::MethodInfo,
        pub delegate_trampoline: *mut ::std::os::raw::c_void,
        pub extraArg: isize,
        pub invoke_impl_this: *mut root::Il2CppObject,
        pub interp_method: *mut ::std::os::raw::c_void,
        pub interp_invoke_impl: *mut ::std::os::raw::c_void,
        pub method_info: *mut root::Il2CppReflectionMethod,
        pub original_method_info: *mut root::Il2CppReflectionMethod,
        pub data: *mut root::Il2CppObject,
        pub method_is_virtual: u8,
    }
    impl Default for Il2CppDelegate {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppDelegate {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppDelegate {{ object: {:?}, method_ptr: {:?}, invoke_impl: {:?}, target: {:?}, method: {:?}, delegate_trampoline: {:?}, extraArg: {:?}, invoke_impl_this: {:?}, interp_method: {:?}, interp_invoke_impl: {:?}, method_info: {:?}, original_method_info: {:?}, data: {:?}, method_is_virtual: {:?} }}" , self . object , self . method_ptr , self . invoke_impl , self . target , self . method , self . delegate_trampoline , self . extraArg , self . invoke_impl_this , self . interp_method , self . interp_invoke_impl , self . method_info , self . original_method_info , self . data , self . method_is_virtual)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppMulticastDelegate {
        pub delegate: root::Il2CppDelegate,
        pub delegates: *mut root::Il2CppArray,
    }
    impl Default for Il2CppMulticastDelegate {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppMulticastDelegate {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppMulticastDelegate {{ delegate: {:?}, delegates: {:?} }}",
                self.delegate, self.delegates
            )
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppMarshalByRefObject {
        pub obj: root::Il2CppObject,
        pub identity: *mut root::Il2CppObject,
    }
    impl Default for Il2CppMarshalByRefObject {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppMarshalByRefObject {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppMarshalByRefObject {{ obj: {:?}, identity: {:?} }}",
                self.obj, self.identity
            )
        }
    }
    pub type Il2CppFullySharedGenericAny = *mut ::std::os::raw::c_void;
    pub type Il2CppFullySharedGenericStruct = *mut ::std::os::raw::c_void;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppAppDomain {
        pub mbr: root::Il2CppMarshalByRefObject,
        pub data: *mut root::Il2CppDomain,
    }
    impl Default for Il2CppAppDomain {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppAppDomain {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppAppDomain {{ mbr: {:?}, data: {:?} }}",
                self.mbr, self.data
            )
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppStackFrame {
        pub obj: root::Il2CppObject,
        pub il_offset: i32,
        pub native_offset: i32,
        pub methodAddress: u64,
        pub methodIndex: u32,
        pub method: *mut root::Il2CppReflectionMethod,
        pub filename: *mut root::Il2CppString,
        pub line: i32,
        pub column: i32,
        pub internal_method_name: *mut root::Il2CppString,
    }
    impl Default for Il2CppStackFrame {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppStackFrame {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppStackFrame {{ obj: {:?}, il_offset: {:?}, native_offset: {:?}, methodAddress: {:?}, methodIndex: {:?}, method: {:?}, filename: {:?}, line: {:?}, column: {:?}, internal_method_name: {:?} }}" , self . obj , self . il_offset , self . native_offset , self . methodAddress , self . methodIndex , self . method , self . filename , self . line , self . column , self . internal_method_name)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppDateTimeFormatInfo {
        pub obj: root::Il2CppObject,
        pub CultureData: *mut root::Il2CppObject,
        pub Name: *mut root::Il2CppString,
        pub LangName: *mut root::Il2CppString,
        pub CompareInfo: *mut root::Il2CppObject,
        pub CultureInfo: *mut root::Il2CppObject,
        pub AMDesignator: *mut root::Il2CppString,
        pub PMDesignator: *mut root::Il2CppString,
        pub DateSeparator: *mut root::Il2CppString,
        pub GeneralShortTimePattern: *mut root::Il2CppString,
        pub GeneralLongTimePattern: *mut root::Il2CppString,
        pub TimeSeparator: *mut root::Il2CppString,
        pub MonthDayPattern: *mut root::Il2CppString,
        pub DateTimeOffsetPattern: *mut root::Il2CppString,
        pub Calendar: *mut root::Il2CppObject,
        pub FirstDayOfWeek: u32,
        pub CalendarWeekRule: u32,
        pub FullDateTimePattern: *mut root::Il2CppString,
        pub AbbreviatedDayNames: *mut root::Il2CppArray,
        pub ShortDayNames: *mut root::Il2CppArray,
        pub DayNames: *mut root::Il2CppArray,
        pub AbbreviatedMonthNames: *mut root::Il2CppArray,
        pub MonthNames: *mut root::Il2CppArray,
        pub GenitiveMonthNames: *mut root::Il2CppArray,
        pub GenitiveAbbreviatedMonthNames: *mut root::Il2CppArray,
        pub LeapYearMonthNames: *mut root::Il2CppArray,
        pub LongDatePattern: *mut root::Il2CppString,
        pub ShortDatePattern: *mut root::Il2CppString,
        pub YearMonthPattern: *mut root::Il2CppString,
        pub LongTimePattern: *mut root::Il2CppString,
        pub ShortTimePattern: *mut root::Il2CppString,
        pub YearMonthPatterns: *mut root::Il2CppArray,
        pub ShortDatePatterns: *mut root::Il2CppArray,
        pub LongDatePatterns: *mut root::Il2CppArray,
        pub ShortTimePatterns: *mut root::Il2CppArray,
        pub LongTimePatterns: *mut root::Il2CppArray,
        pub EraNames: *mut root::Il2CppArray,
        pub AbbrevEraNames: *mut root::Il2CppArray,
        pub AbbrevEnglishEraNames: *mut root::Il2CppArray,
        pub OptionalCalendars: *mut root::Il2CppArray,
        pub readOnly: u8,
        pub FormatFlags: i32,
        pub CultureID: i32,
        pub UseUserOverride: u8,
        pub UseCalendarInfo: u8,
        pub DataItem: i32,
        pub IsDefaultCalendar: u8,
        pub DateWords: *mut root::Il2CppArray,
        pub FullTimeSpanPositivePattern: *mut root::Il2CppString,
        pub FullTimeSpanNegativePattern: *mut root::Il2CppString,
        pub dtfiTokenHash: *mut root::Il2CppArray,
    }
    impl Default for Il2CppDateTimeFormatInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppDateTimeFormatInfo {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppDateTimeFormatInfo {{ obj: {:?}, CultureData: {:?}, Name: {:?}, LangName: {:?}, CompareInfo: {:?}, CultureInfo: {:?}, AMDesignator: {:?}, PMDesignator: {:?}, DateSeparator: {:?}, GeneralShortTimePattern: {:?}, GeneralLongTimePattern: {:?}, TimeSeparator: {:?}, MonthDayPattern: {:?}, DateTimeOffsetPattern: {:?}, Calendar: {:?}, FirstDayOfWeek: {:?}, CalendarWeekRule: {:?}, FullDateTimePattern: {:?}, AbbreviatedDayNames: {:?}, ShortDayNames: {:?}, DayNames: {:?}, AbbreviatedMonthNames: {:?}, MonthNames: {:?}, GenitiveMonthNames: {:?}, GenitiveAbbreviatedMonthNames: {:?}, LeapYearMonthNames: {:?}, LongDatePattern: {:?}, ShortDatePattern: {:?}, YearMonthPattern: {:?}, LongTimePattern: {:?}, ShortTimePattern: {:?}, YearMonthPatterns: {:?}, ShortDatePatterns: {:?}, LongDatePatterns: {:?}, ShortTimePatterns: {:?}, LongTimePatterns: {:?}, EraNames: {:?}, AbbrevEraNames: {:?}, AbbrevEnglishEraNames: {:?}, OptionalCalendars: {:?}, readOnly: {:?}, FormatFlags: {:?}, CultureID: {:?}, UseUserOverride: {:?}, UseCalendarInfo: {:?}, DataItem: {:?}, IsDefaultCalendar: {:?}, DateWords: {:?}, FullTimeSpanPositivePattern: {:?}, FullTimeSpanNegativePattern: {:?}, dtfiTokenHash: {:?} }}" , self . obj , self . CultureData , self . Name , self . LangName , self . CompareInfo , self . CultureInfo , self . AMDesignator , self . PMDesignator , self . DateSeparator , self . GeneralShortTimePattern , self . GeneralLongTimePattern , self . TimeSeparator , self . MonthDayPattern , self . DateTimeOffsetPattern , self . Calendar , self . FirstDayOfWeek , self . CalendarWeekRule , self . FullDateTimePattern , self . AbbreviatedDayNames , self . ShortDayNames , self . DayNames , self . AbbreviatedMonthNames , self . MonthNames , self . GenitiveMonthNames , self . GenitiveAbbreviatedMonthNames , self . LeapYearMonthNames , self . LongDatePattern , self . ShortDatePattern , self . YearMonthPattern , self . LongTimePattern , self . ShortTimePattern , self . YearMonthPatterns , self . ShortDatePatterns , self . LongDatePatterns , self . ShortTimePatterns , self . LongTimePatterns , self . EraNames , self . AbbrevEraNames , self . AbbrevEnglishEraNames , self . OptionalCalendars , self . readOnly , self . FormatFlags , self . CultureID , self . UseUserOverride , self . UseCalendarInfo , self . DataItem , self . IsDefaultCalendar , self . DateWords , self . FullTimeSpanPositivePattern , self . FullTimeSpanNegativePattern , self . dtfiTokenHash)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppNumberFormatInfo {
        pub obj: root::Il2CppObject,
        pub numberGroupSizes: *mut root::Il2CppArray,
        pub currencyGroupSizes: *mut root::Il2CppArray,
        pub percentGroupSizes: *mut root::Il2CppArray,
        pub positiveSign: *mut root::Il2CppString,
        pub negativeSign: *mut root::Il2CppString,
        pub numberDecimalSeparator: *mut root::Il2CppString,
        pub numberGroupSeparator: *mut root::Il2CppString,
        pub currencyGroupSeparator: *mut root::Il2CppString,
        pub currencyDecimalSeparator: *mut root::Il2CppString,
        pub currencySymbol: *mut root::Il2CppString,
        pub ansiCurrencySymbol: *mut root::Il2CppString,
        pub naNSymbol: *mut root::Il2CppString,
        pub positiveInfinitySymbol: *mut root::Il2CppString,
        pub negativeInfinitySymbol: *mut root::Il2CppString,
        pub percentDecimalSeparator: *mut root::Il2CppString,
        pub percentGroupSeparator: *mut root::Il2CppString,
        pub percentSymbol: *mut root::Il2CppString,
        pub perMilleSymbol: *mut root::Il2CppString,
        pub nativeDigits: *mut root::Il2CppArray,
        pub dataItem: ::std::os::raw::c_int,
        pub numberDecimalDigits: ::std::os::raw::c_int,
        pub currencyDecimalDigits: ::std::os::raw::c_int,
        pub currencyPositivePattern: ::std::os::raw::c_int,
        pub currencyNegativePattern: ::std::os::raw::c_int,
        pub numberNegativePattern: ::std::os::raw::c_int,
        pub percentPositivePattern: ::std::os::raw::c_int,
        pub percentNegativePattern: ::std::os::raw::c_int,
        pub percentDecimalDigits: ::std::os::raw::c_int,
        pub digitSubstitution: ::std::os::raw::c_int,
        pub readOnly: u8,
        pub useUserOverride: u8,
        pub isInvariant: u8,
        pub validForParseAsNumber: u8,
        pub validForParseAsCurrency: u8,
    }
    impl Default for Il2CppNumberFormatInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppNumberFormatInfo {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppNumberFormatInfo {{ obj: {:?}, numberGroupSizes: {:?}, currencyGroupSizes: {:?}, percentGroupSizes: {:?}, positiveSign: {:?}, negativeSign: {:?}, numberDecimalSeparator: {:?}, numberGroupSeparator: {:?}, currencyGroupSeparator: {:?}, currencyDecimalSeparator: {:?}, currencySymbol: {:?}, ansiCurrencySymbol: {:?}, naNSymbol: {:?}, positiveInfinitySymbol: {:?}, negativeInfinitySymbol: {:?}, percentDecimalSeparator: {:?}, percentGroupSeparator: {:?}, percentSymbol: {:?}, perMilleSymbol: {:?}, nativeDigits: {:?}, dataItem: {:?}, numberDecimalDigits: {:?}, currencyDecimalDigits: {:?}, currencyPositivePattern: {:?}, currencyNegativePattern: {:?}, numberNegativePattern: {:?}, percentPositivePattern: {:?}, percentNegativePattern: {:?}, percentDecimalDigits: {:?}, digitSubstitution: {:?}, readOnly: {:?}, useUserOverride: {:?}, isInvariant: {:?}, validForParseAsNumber: {:?}, validForParseAsCurrency: {:?} }}" , self . obj , self . numberGroupSizes , self . currencyGroupSizes , self . percentGroupSizes , self . positiveSign , self . negativeSign , self . numberDecimalSeparator , self . numberGroupSeparator , self . currencyGroupSeparator , self . currencyDecimalSeparator , self . currencySymbol , self . ansiCurrencySymbol , self . naNSymbol , self . positiveInfinitySymbol , self . negativeInfinitySymbol , self . percentDecimalSeparator , self . percentGroupSeparator , self . percentSymbol , self . perMilleSymbol , self . nativeDigits , self . dataItem , self . numberDecimalDigits , self . currencyDecimalDigits , self . currencyPositivePattern , self . currencyNegativePattern , self . numberNegativePattern , self . percentPositivePattern , self . percentNegativePattern , self . percentDecimalDigits , self . digitSubstitution , self . readOnly , self . useUserOverride , self . isInvariant , self . validForParseAsNumber , self . validForParseAsCurrency)
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct NumberFormatEntryManaged {
        pub currency_decimal_digits: i32,
        pub currency_decimal_separator: i32,
        pub currency_group_separator: i32,
        pub currency_group_sizes0: i32,
        pub currency_group_sizes1: i32,
        pub currency_negative_pattern: i32,
        pub currency_positive_pattern: i32,
        pub currency_symbol: i32,
        pub nan_symbol: i32,
        pub negative_infinity_symbol: i32,
        pub negative_sign: i32,
        pub number_decimal_digits: i32,
        pub number_decimal_separator: i32,
        pub number_group_separator: i32,
        pub number_group_sizes0: i32,
        pub number_group_sizes1: i32,
        pub number_negative_pattern: i32,
        pub per_mille_symbol: i32,
        pub percent_negative_pattern: i32,
        pub percent_positive_pattern: i32,
        pub percent_symbol: i32,
        pub positive_infinity_symbol: i32,
        pub positive_sign: i32,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppCultureData {
        pub obj: root::Il2CppObject,
        pub AMDesignator: *mut root::Il2CppString,
        pub PMDesignator: *mut root::Il2CppString,
        pub TimeSeparator: *mut root::Il2CppString,
        pub LongTimePatterns: *mut root::Il2CppArray,
        pub ShortTimePatterns: *mut root::Il2CppArray,
        pub FirstDayOfWeek: u32,
        pub CalendarWeekRule: u32,
    }
    impl Default for Il2CppCultureData {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppCultureData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppCultureData {{ obj: {:?}, AMDesignator: {:?}, PMDesignator: {:?}, TimeSeparator: {:?}, LongTimePatterns: {:?}, ShortTimePatterns: {:?}, FirstDayOfWeek: {:?}, CalendarWeekRule: {:?} }}" , self . obj , self . AMDesignator , self . PMDesignator , self . TimeSeparator , self . LongTimePatterns , self . ShortTimePatterns , self . FirstDayOfWeek , self . CalendarWeekRule)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppCalendarData {
        pub obj: root::Il2CppObject,
        pub NativeName: *mut root::Il2CppString,
        pub ShortDatePatterns: *mut root::Il2CppArray,
        pub YearMonthPatterns: *mut root::Il2CppArray,
        pub LongDatePatterns: *mut root::Il2CppArray,
        pub MonthDayPattern: *mut root::Il2CppString,
        pub EraNames: *mut root::Il2CppArray,
        pub AbbreviatedEraNames: *mut root::Il2CppArray,
        pub AbbreviatedEnglishEraNames: *mut root::Il2CppArray,
        pub DayNames: *mut root::Il2CppArray,
        pub AbbreviatedDayNames: *mut root::Il2CppArray,
        pub SuperShortDayNames: *mut root::Il2CppArray,
        pub MonthNames: *mut root::Il2CppArray,
        pub AbbreviatedMonthNames: *mut root::Il2CppArray,
        pub GenitiveMonthNames: *mut root::Il2CppArray,
        pub GenitiveAbbreviatedMonthNames: *mut root::Il2CppArray,
    }
    impl Default for Il2CppCalendarData {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppCalendarData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppCalendarData {{ obj: {:?}, NativeName: {:?}, ShortDatePatterns: {:?}, YearMonthPatterns: {:?}, LongDatePatterns: {:?}, MonthDayPattern: {:?}, EraNames: {:?}, AbbreviatedEraNames: {:?}, AbbreviatedEnglishEraNames: {:?}, DayNames: {:?}, AbbreviatedDayNames: {:?}, SuperShortDayNames: {:?}, MonthNames: {:?}, AbbreviatedMonthNames: {:?}, GenitiveMonthNames: {:?}, GenitiveAbbreviatedMonthNames: {:?} }}" , self . obj , self . NativeName , self . ShortDatePatterns , self . YearMonthPatterns , self . LongDatePatterns , self . MonthDayPattern , self . EraNames , self . AbbreviatedEraNames , self . AbbreviatedEnglishEraNames , self . DayNames , self . AbbreviatedDayNames , self . SuperShortDayNames , self . MonthNames , self . AbbreviatedMonthNames , self . GenitiveMonthNames , self . GenitiveAbbreviatedMonthNames)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppCultureInfo {
        pub obj: root::Il2CppObject,
        pub is_read_only: u8,
        pub lcid: i32,
        pub parent_lcid: i32,
        pub datetime_index: i32,
        pub number_index: i32,
        pub default_calendar_type: i32,
        pub use_user_override: u8,
        pub number_format: *mut root::Il2CppNumberFormatInfo,
        pub datetime_format: *mut root::Il2CppDateTimeFormatInfo,
        pub textinfo: *mut root::Il2CppObject,
        pub name: *mut root::Il2CppString,
        pub englishname: *mut root::Il2CppString,
        pub nativename: *mut root::Il2CppString,
        pub iso3lang: *mut root::Il2CppString,
        pub iso2lang: *mut root::Il2CppString,
        pub win3lang: *mut root::Il2CppString,
        pub territory: *mut root::Il2CppString,
        pub native_calendar_names: *mut root::Il2CppArray,
        pub compareinfo: *mut root::Il2CppString,
        pub text_info_data: *const ::std::os::raw::c_void,
        pub dataItem: ::std::os::raw::c_int,
        pub calendar: *mut root::Il2CppObject,
        pub parent_culture: *mut root::Il2CppObject,
        pub constructed: u8,
        pub cached_serialized_form: *mut root::Il2CppArray,
        pub cultureData: *mut root::Il2CppObject,
        pub isInherited: u8,
    }
    impl Default for Il2CppCultureInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppCultureInfo {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppCultureInfo {{ obj: {:?}, is_read_only: {:?}, lcid: {:?}, parent_lcid: {:?}, datetime_index: {:?}, number_index: {:?}, default_calendar_type: {:?}, use_user_override: {:?}, number_format: {:?}, datetime_format: {:?}, textinfo: {:?}, name: {:?}, englishname: {:?}, nativename: {:?}, iso3lang: {:?}, iso2lang: {:?}, win3lang: {:?}, territory: {:?}, native_calendar_names: {:?}, compareinfo: {:?}, text_info_data: {:?}, dataItem: {:?}, calendar: {:?}, parent_culture: {:?}, constructed: {:?}, cached_serialized_form: {:?}, cultureData: {:?}, isInherited: {:?} }}" , self . obj , self . is_read_only , self . lcid , self . parent_lcid , self . datetime_index , self . number_index , self . default_calendar_type , self . use_user_override , self . number_format , self . datetime_format , self . textinfo , self . name , self . englishname , self . nativename , self . iso3lang , self . iso2lang , self . win3lang , self . territory , self . native_calendar_names , self . compareinfo , self . text_info_data , self . dataItem , self . calendar , self . parent_culture , self . constructed , self . cached_serialized_form , self . cultureData , self . isInherited)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppRegionInfo {
        pub obj: root::Il2CppObject,
        pub geo_id: i32,
        pub iso2name: *mut root::Il2CppString,
        pub iso3name: *mut root::Il2CppString,
        pub win3name: *mut root::Il2CppString,
        pub english_name: *mut root::Il2CppString,
        pub native_name: *mut root::Il2CppString,
        pub currency_symbol: *mut root::Il2CppString,
        pub iso_currency_symbol: *mut root::Il2CppString,
        pub currency_english_name: *mut root::Il2CppString,
        pub currency_native_name: *mut root::Il2CppString,
    }
    impl Default for Il2CppRegionInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppRegionInfo {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppRegionInfo {{ obj: {:?}, geo_id: {:?}, iso2name: {:?}, iso3name: {:?}, win3name: {:?}, english_name: {:?}, native_name: {:?}, currency_symbol: {:?}, iso_currency_symbol: {:?}, currency_english_name: {:?}, currency_native_name: {:?} }}" , self . obj , self . geo_id , self . iso2name , self . iso3name , self . win3name , self . english_name , self . native_name , self . currency_symbol , self . iso_currency_symbol , self . currency_english_name , self . currency_native_name)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppSafeHandle {
        pub base: root::Il2CppObject,
        pub handle: *mut ::std::os::raw::c_void,
        pub state: ::std::os::raw::c_int,
        pub owns_handle: u8,
        pub fullyInitialized: u8,
    }
    impl Default for Il2CppSafeHandle {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppSafeHandle {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppSafeHandle {{ base: {:?}, handle: {:?}, state: {:?}, owns_handle: {:?}, fullyInitialized: {:?} }}" , self . base , self . handle , self . state , self . owns_handle , self . fullyInitialized)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppStringBuilder {
        pub object: root::Il2CppObject,
        pub chunkChars: *mut root::Il2CppArray,
        pub chunkPrevious: *mut root::Il2CppStringBuilder,
        pub chunkLength: ::std::os::raw::c_int,
        pub chunkOffset: ::std::os::raw::c_int,
        pub maxCapacity: ::std::os::raw::c_int,
    }
    impl Default for Il2CppStringBuilder {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppStringBuilder {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppStringBuilder {{ object: {:?}, chunkChars: {:?}, chunkPrevious: {:?}, chunkLength: {:?}, chunkOffset: {:?}, maxCapacity: {:?} }}" , self . object , self . chunkChars , self . chunkPrevious , self . chunkLength , self . chunkOffset , self . maxCapacity)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppSocketAddress {
        pub base: root::Il2CppObject,
        pub m_Size: ::std::os::raw::c_int,
        pub data: *mut root::Il2CppArray,
        pub m_changed: u8,
        pub m_hash: ::std::os::raw::c_int,
    }
    impl Default for Il2CppSocketAddress {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppSocketAddress {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppSocketAddress {{ base: {:?}, m_Size: {:?}, data: {:?}, m_changed: {:?}, m_hash: {:?} }}" , self . base , self . m_Size , self . data , self . m_changed , self . m_hash)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppSortKey {
        pub base: root::Il2CppObject,
        pub str_: *mut root::Il2CppString,
        pub key: *mut root::Il2CppArray,
        pub options: i32,
        pub lcid: i32,
    }
    impl Default for Il2CppSortKey {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppSortKey {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppSortKey {{ base: {:?}, str: {:?}, key: {:?}, options: {:?}, lcid: {:?} }}",
                self.base, self.str_, self.key, self.options, self.lcid
            )
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppErrorWrapper {
        pub base: root::Il2CppObject,
        pub errorCode: i32,
    }
    impl Default for Il2CppErrorWrapper {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppErrorWrapper {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppErrorWrapper {{ base: {:?}, errorCode: {:?} }}",
                self.base, self.errorCode
            )
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppAsyncResult {
        pub base: root::Il2CppObject,
        pub async_state: *mut root::Il2CppObject,
        pub handle: *mut root::Il2CppWaitHandle,
        pub async_delegate: *mut root::Il2CppDelegate,
        pub data: *mut ::std::os::raw::c_void,
        pub object_data: *mut root::Il2CppAsyncCall,
        pub sync_completed: u8,
        pub completed: u8,
        pub endinvoke_called: u8,
        pub async_callback: *mut root::Il2CppObject,
        pub execution_context: *mut root::Il2CppObject,
        pub original_context: *mut root::Il2CppObject,
    }
    impl Default for Il2CppAsyncResult {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppAsyncResult {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppAsyncResult {{ base: {:?}, async_state: {:?}, handle: {:?}, async_delegate: {:?}, data: {:?}, object_data: {:?}, sync_completed: {:?}, completed: {:?}, endinvoke_called: {:?}, async_callback: {:?}, execution_context: {:?}, original_context: {:?} }}" , self . base , self . async_state , self . handle , self . async_delegate , self . data , self . object_data , self . sync_completed , self . completed , self . endinvoke_called , self . async_callback , self . execution_context , self . original_context)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppAsyncCall {
        pub base: root::Il2CppObject,
        pub msg: *mut root::Il2CppMethodMessage,
        pub cb_method: *mut root::MethodInfo,
        pub cb_target: *mut root::Il2CppDelegate,
        pub state: *mut root::Il2CppObject,
        pub res: *mut root::Il2CppObject,
        pub out_args: *mut root::Il2CppArray,
    }
    impl Default for Il2CppAsyncCall {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppAsyncCall {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppAsyncCall {{ base: {:?}, msg: {:?}, cb_method: {:?}, cb_target: {:?}, state: {:?}, res: {:?}, out_args: {:?} }}" , self . base , self . msg , self . cb_method , self . cb_target , self . state , self . res , self . out_args)
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Il2CppExceptionWrapper {
        pub ex: *mut root::Il2CppException,
    }
    impl Default for Il2CppExceptionWrapper {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppIOAsyncResult {
        pub base: root::Il2CppObject,
        pub callback: *mut root::Il2CppDelegate,
        pub state: *mut root::Il2CppObject,
        pub wait_handle: *mut root::Il2CppWaitHandle,
        pub completed_synchronously: u8,
        pub completed: u8,
    }
    impl Default for Il2CppIOAsyncResult {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppIOAsyncResult {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppIOAsyncResult {{ base: {:?}, callback: {:?}, state: {:?}, wait_handle: {:?}, completed_synchronously: {:?}, completed: {:?} }}" , self . base , self . callback , self . state , self . wait_handle , self . completed_synchronously , self . completed)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppSocketAsyncResult {
        pub base: root::Il2CppIOAsyncResult,
        pub socket: *mut root::Il2CppObject,
        pub operation: i32,
        pub delayedException: *mut root::Il2CppException,
        pub endPoint: *mut root::Il2CppObject,
        pub buffer: *mut root::Il2CppArray,
        pub offset: i32,
        pub size: i32,
        pub socket_flags: i32,
        pub acceptSocket: *mut root::Il2CppObject,
        pub addresses: *mut root::Il2CppArray,
        pub port: i32,
        pub buffers: *mut root::Il2CppObject,
        pub reuseSocket: u8,
        pub currentAddress: i32,
        pub acceptedSocket: *mut root::Il2CppObject,
        pub total: i32,
        pub error: i32,
        pub endCalled: i32,
    }
    impl Default for Il2CppSocketAsyncResult {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppSocketAsyncResult {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppSocketAsyncResult {{ base: {:?}, socket: {:?}, operation: {:?}, delayedException: {:?}, endPoint: {:?}, buffer: {:?}, offset: {:?}, size: {:?}, socket_flags: {:?}, acceptSocket: {:?}, addresses: {:?}, port: {:?}, buffers: {:?}, reuseSocket: {:?}, currentAddress: {:?}, acceptedSocket: {:?}, total: {:?}, error: {:?}, endCalled: {:?} }}" , self . base , self . socket , self . operation , self . delayedException , self . endPoint , self . buffer , self . offset , self . size , self . socket_flags , self . acceptSocket , self . addresses , self . port , self . buffers , self . reuseSocket , self . currentAddress , self . acceptedSocket , self . total , self . error , self . endCalled)
        }
    }
    pub const IL2CPP_RESOURCE_LOCATION_EMBEDDED: Il2CppResourceLocation = 1;
    pub const IL2CPP_RESOURCE_LOCATION_ANOTHER_ASSEMBLY: Il2CppResourceLocation = 2;
    pub const IL2CPP_RESOURCE_LOCATION_IN_MANIFEST: Il2CppResourceLocation = 4;
    pub type Il2CppResourceLocation = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppManifestResourceInfo {
        pub object: root::Il2CppObject,
        pub assembly: *mut root::Il2CppReflectionAssembly,
        pub filename: *mut root::Il2CppString,
        pub location: u32,
    }
    impl Default for Il2CppManifestResourceInfo {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppManifestResourceInfo {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppManifestResourceInfo {{ object: {:?}, assembly: {:?}, filename: {:?}, location: {:?} }}" , self . object , self . assembly , self . filename , self . location)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppAppContext {
        pub obj: root::Il2CppObject,
        pub domain_id: i32,
        pub context_id: i32,
        pub static_data: *mut ::std::os::raw::c_void,
    }
    impl Default for Il2CppAppContext {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppAppContext {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write ! (f , "Il2CppAppContext {{ obj: {:?}, domain_id: {:?}, context_id: {:?}, static_data: {:?} }}" , self . obj , self . domain_id , self . context_id , self . static_data)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Il2CppDecimal {
        pub reserved: u16,
        pub u: root::Il2CppDecimal__bindgen_ty_1,
        pub Hi32: u32,
        pub v: root::Il2CppDecimal__bindgen_ty_2,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Il2CppDecimal__bindgen_ty_1 {
        pub u: root::Il2CppDecimal__bindgen_ty_1__bindgen_ty_1,
        pub signscale: u16,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppDecimal__bindgen_ty_1__bindgen_ty_1 {
        pub scale: u8,
        pub sign: u8,
    }
    impl Default for Il2CppDecimal__bindgen_ty_1 {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppDecimal__bindgen_ty_1 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppDecimal__bindgen_ty_1 {{ union }}")
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Il2CppDecimal__bindgen_ty_2 {
        pub v: root::Il2CppDecimal__bindgen_ty_2__bindgen_ty_1,
        pub Lo64: u64,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppDecimal__bindgen_ty_2__bindgen_ty_1 {
        pub Lo32: u32,
        pub Mid32: u32,
    }
    impl Default for Il2CppDecimal__bindgen_ty_2 {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppDecimal__bindgen_ty_2 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppDecimal__bindgen_ty_2 {{ union }}")
        }
    }
    impl Default for Il2CppDecimal {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppDecimal {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(
                f,
                "Il2CppDecimal {{ reserved: {:?}, u: {:?}, Hi32: {:?}, v: {:?} }}",
                self.reserved, self.u, self.Hi32, self.v
            )
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppDouble {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: root::__BindgenBitfieldUnit<[u8; 8usize]>,
    }
    impl Il2CppDouble {
        #[inline]
        pub fn mantLo(&self) -> u32 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 32u8) as u32) }
        }
        #[inline]
        pub fn set_mantLo(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(0usize, 32u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn mantLo_raw(this: *const Self) -> u32 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    0usize,
                    32u8,
                ) as u32)
            }
        }
        #[inline]
        pub unsafe fn set_mantLo_raw(this: *mut Self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    0usize,
                    32u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn mantHi(&self) -> u32 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(32usize, 20u8) as u32) }
        }
        #[inline]
        pub fn set_mantHi(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(32usize, 20u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn mantHi_raw(this: *const Self) -> u32 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    32usize,
                    20u8,
                ) as u32)
            }
        }
        #[inline]
        pub unsafe fn set_mantHi_raw(this: *mut Self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    32usize,
                    20u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn exp(&self) -> u32 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(52usize, 11u8) as u32) }
        }
        #[inline]
        pub fn set_exp(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(52usize, 11u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn exp_raw(this: *const Self) -> u32 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    52usize,
                    11u8,
                ) as u32)
            }
        }
        #[inline]
        pub unsafe fn set_exp_raw(this: *mut Self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    52usize,
                    11u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn sign(&self) -> u32 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(63usize, 1u8) as u32) }
        }
        #[inline]
        pub fn set_sign(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(63usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn sign_raw(this: *const Self) -> u32 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    63usize,
                    1u8,
                ) as u32)
            }
        }
        #[inline]
        pub unsafe fn set_sign_raw(this: *mut Self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    63usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn new_bitfield_1(
            mantLo: u32,
            mantHi: u32,
            exp: u32,
            sign: u32,
        ) -> root::__BindgenBitfieldUnit<[u8; 8usize]> {
            let mut __bindgen_bitfield_unit: root::__BindgenBitfieldUnit<[u8; 8usize]> =
                Default::default();
            __bindgen_bitfield_unit.set(0usize, 32u8, {
                let mantLo: u32 = unsafe { ::std::mem::transmute(mantLo) };
                mantLo as u64
            });
            __bindgen_bitfield_unit.set(32usize, 20u8, {
                let mantHi: u32 = unsafe { ::std::mem::transmute(mantHi) };
                mantHi as u64
            });
            __bindgen_bitfield_unit.set(52usize, 11u8, {
                let exp: u32 = unsafe { ::std::mem::transmute(exp) };
                exp as u64
            });
            __bindgen_bitfield_unit.set(63usize, 1u8, {
                let sign: u32 = unsafe { ::std::mem::transmute(sign) };
                sign as u64
            });
            __bindgen_bitfield_unit
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Il2CppDouble_double {
        pub s: root::Il2CppDouble,
        pub d: f64,
    }
    impl Default for Il2CppDouble_double {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppDouble_double {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppDouble_double {{ union }}")
        }
    }
    pub const IL2CPP_DECIMAL_CMP_LT: Il2CppDecimalCompareResult = -1;
    pub const IL2CPP_DECIMAL_CMP_EQ: Il2CppDecimalCompareResult = 0;
    pub const IL2CPP_DECIMAL_CMP_GT: Il2CppDecimalCompareResult = 1;
    pub type Il2CppDecimalCompareResult = ::std::os::raw::c_int;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppSingle {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: root::__BindgenBitfieldUnit<[u8; 4usize]>,
    }
    impl Il2CppSingle {
        #[inline]
        pub fn mant(&self) -> u32 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 23u8) as u32) }
        }
        #[inline]
        pub fn set_mant(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(0usize, 23u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn mant_raw(this: *const Self) -> u32 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    0usize,
                    23u8,
                ) as u32)
            }
        }
        #[inline]
        pub unsafe fn set_mant_raw(this: *mut Self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    0usize,
                    23u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn exp(&self) -> u32 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(23usize, 8u8) as u32) }
        }
        #[inline]
        pub fn set_exp(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(23usize, 8u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn exp_raw(this: *const Self) -> u32 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    23usize,
                    8u8,
                ) as u32)
            }
        }
        #[inline]
        pub unsafe fn set_exp_raw(this: *mut Self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    23usize,
                    8u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn sign(&self) -> u32 {
            unsafe { ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
        }
        #[inline]
        pub fn set_sign(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                self._bitfield_1.set(31usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub unsafe fn sign_raw(this: *const Self) -> u32 {
            unsafe {
                ::std::mem::transmute(<root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                    ::std::ptr::addr_of!((*this)._bitfield_1),
                    31usize,
                    1u8,
                ) as u32)
            }
        }
        #[inline]
        pub unsafe fn set_sign_raw(this: *mut Self, val: u32) {
            unsafe {
                let val: u32 = ::std::mem::transmute(val);
                <root::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                    ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                    31usize,
                    1u8,
                    val as u64,
                )
            }
        }
        #[inline]
        pub fn new_bitfield_1(
            mant: u32,
            exp: u32,
            sign: u32,
        ) -> root::__BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit: root::__BindgenBitfieldUnit<[u8; 4usize]> =
                Default::default();
            __bindgen_bitfield_unit.set(0usize, 23u8, {
                let mant: u32 = unsafe { ::std::mem::transmute(mant) };
                mant as u64
            });
            __bindgen_bitfield_unit.set(23usize, 8u8, {
                let exp: u32 = unsafe { ::std::mem::transmute(exp) };
                exp as u64
            });
            __bindgen_bitfield_unit.set(31usize, 1u8, {
                let sign: u32 = unsafe { ::std::mem::transmute(sign) };
                sign as u64
            });
            __bindgen_bitfield_unit
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Il2CppSingle_float {
        pub s: root::Il2CppSingle,
        pub f: f32,
    }
    impl Default for Il2CppSingle_float {
        fn default() -> Self {
            let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
            unsafe {
                ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                s.assume_init()
            }
        }
    }
    impl ::std::fmt::Debug for Il2CppSingle_float {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "Il2CppSingle_float {{ union }}")
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Il2CppByReference {
        pub value: isize,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct __crt_locale_data {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct __crt_multibyte_data {
        pub _address: u8,
    }
}
