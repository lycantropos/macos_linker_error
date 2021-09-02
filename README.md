On macOS 11.4 command
```shell
cargo run
```
gives output
```shell
   Compiling autocfg v1.0.1
   Compiling proc-macro2 v1.0.29
   Compiling unicode-xid v0.2.2
   Compiling proc-macro-hack v0.5.19
   Compiling pyo3-build-config v0.14.4
   Compiling syn v1.0.75
   Compiling once_cell v1.8.0
   Compiling libc v0.2.101
   Compiling parking_lot_core v0.8.5
   Compiling cfg-if v1.0.0
   Compiling smallvec v1.6.1
   Compiling unindent v0.1.7
   Compiling scopeguard v1.1.0
   Compiling instant v0.1.10
   Compiling lock_api v0.4.5
   Compiling num-traits v0.2.14
   Compiling num-integer v0.1.44
   Compiling num-bigint v0.4.1
   Compiling num-iter v0.1.42
   Compiling num-rational v0.4.0
   Compiling quote v1.0.9
   Compiling paste-impl v0.1.18
   Compiling parking_lot v0.11.2
   Compiling paste v0.1.18
   Compiling pyo3 v0.14.4
   Compiling num-complex v0.4.0
   Compiling num v0.4.0
   Compiling pyo3-macros-backend v0.14.4
   Compiling indoc-impl v0.3.6
   Compiling indoc v0.3.6
   Compiling pyo3-macros v0.14.4
   Compiling rithm v0.1.0-alpha (https://github.com/lycantropos/rithm#a955e421)
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-Wl,-exported_symbols_list,/var/folders/1m/d564g9s90wx59lvyhc7qr04h0000gp/T/rustcExXkFQ/list" "-m64" "-arch" "x86_64" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.0.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.1.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.10.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.11.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.12.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.13.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.14.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.15.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.2.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.3.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.4.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.5.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.6.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.7.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.8.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.9.rcgu.o" "/path/to/project/macos_link_error/target/debug/deps/rithm-874fa952c0dfbfcd.4ennbeh2p24arvji.rcgu.o" "-L" "/path/to/project/macos_link_error/target/debug/deps" "-L" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "/path/to/project/macos_link_error/target/debug/deps/libpyo3-3e4af82221a23595.rlib" "/path/to/project/macos_link_error/target/debug/deps/libparking_lot-f6facfbd8ae81600.rlib" "/path/to/project/macos_link_error/target/debug/deps/libparking_lot_core-7286ee42ede0aba5.rlib" "/path/to/project/macos_link_error/target/debug/deps/libsmallvec-71a3c70e4815f439.rlib" "/path/to/project/macos_link_error/target/debug/deps/liblock_api-3eb450f70214f740.rlib" "/path/to/project/macos_link_error/target/debug/deps/libscopeguard-21323d145333e807.rlib" "/path/to/project/macos_link_error/target/debug/deps/libinstant-b84201e8bc3c59f3.rlib" "/path/to/project/macos_link_error/target/debug/deps/libcfg_if-89cb16fc176d2309.rlib" "/path/to/project/macos_link_error/target/debug/deps/liblibc-7c9338df47123672.rlib" "/path/to/project/macos_link_error/target/debug/deps/libunindent-a8335a8d0262c9b2.rlib" "/path/to/project/macos_link_error/target/debug/deps/libpaste-09a60650307b2a1f.rlib" "/path/to/project/macos_link_error/target/debug/deps/libindoc-ea44f38dea687b6a.rlib" "/path/to/project/macos_link_error/target/debug/deps/libnum-58da3eb914fb7571.rlib" "/path/to/project/macos_link_error/target/debug/deps/libnum_iter-fc2359c618fd6ce6.rlib" "/path/to/project/macos_link_error/target/debug/deps/libnum_rational-89fd82a023da9c86.rlib" "/path/to/project/macos_link_error/target/debug/deps/libnum_complex-18bd085ad380808c.rlib" "/path/to/project/macos_link_error/target/debug/deps/libnum_bigint-518e3d7ff2f5b9d5.rlib" "/path/to/project/macos_link_error/target/debug/deps/libnum_integer-f4aa57a9652c5732.rlib" "/path/to/project/macos_link_error/target/debug/deps/libnum_traits-27951fbbe5422f10.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libstd-0b5b2ab270b30aa6.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-9b1c58d152d1cf8a.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libobject-af4549a9ce174752.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libmemchr-07ae024de83bd825.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libaddr2line-e29567287f6f2631.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libgimli-b890d23aa1a4f088.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libstd_detect-7f2c5c43576f3f1e.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-9ef7eac13f2aef02.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-254c70bb4aa29f7f.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-8b2f07d42fae2b94.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libunwind-36662cd7bf4acdfe.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-c2481d7a76910e24.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/liblibc-6927d30bb41b6c68.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/liballoc-4659604f23c5c081.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-fd73057e4f65bab6.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libcore-42ed710bc402770f.rlib" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-2b886adf769c3d99.rlib" "-liconv" "-lSystem" "-lresolv" "-lc" "-lm" "-liconv" "-L" "/Users/aibrakov/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "-o" "/path/to/project/macos_link_error/target/debug/deps/librithm-874fa952c0dfbfcd.dylib" "-Wl,-dead_strip" "-dynamiclib" "-Wl,-dylib" "-nodefaultlibs"
  = note: Undefined symbols for architecture x86_64:
            "_PyBytes_Size", referenced from:
                pyo3::types::bytes::PyBytes::as_bytes::h0a92f94eeb6b9ce3 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.10.rcgu.o)
            "_PyObject_GenericGetDict", referenced from:
                pyo3::pyclass::push_dict_getset::hfb7e87260bb05444 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.13.rcgu.o)
            "_PyObject_GenericSetDict", referenced from:
                pyo3::pyclass::push_dict_getset::hfb7e87260bb05444 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.13.rcgu.o)
            "_PyErr_Print", referenced from:
                pyo3::err::panic_after_error::h323aca14c5c07bb6 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.13.rcgu.o)
            "_PyErr_PrintEx", referenced from:
                pyo3::err::PyErr::print::hf2ac913468255f46 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.13.rcgu.o)
            "_PyExc_ValueError", referenced from:
                _$LT$pyo3..exceptions..PyValueError$u20$as$u20$pyo3..type_object..PyTypeInfo$GT$::type_object_raw::haeb1489a68c9292f in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.1.rcgu.o)
            "_PyEval_ThreadsInitialized", referenced from:
                pyo3::gil::GILGuard::acquire::_$u7b$$u7b$closure$u7d$$u7d$::h1f1574ad8006a9e6 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.7.rcgu.o)
            "_PyErr_Occurred", referenced from:
                pyo3::err::PyErr::occurred::h0497f2928d2ce77d in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.13.rcgu.o)
            "_PyExc_RuntimeError", referenced from:
                _$LT$pyo3..exceptions..PyRuntimeError$u20$as$u20$pyo3..type_object..PyTypeInfo$GT$::type_object_raw::h1c49a8bfca7d1d4d in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.1.rcgu.o)
            "_PyExc_OverflowError", referenced from:
                _$LT$pyo3..exceptions..PyOverflowError$u20$as$u20$pyo3..type_object..PyTypeInfo$GT$::type_object_raw::h8b37095595e3aea9 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.1.rcgu.o)
            "_PyExc_BaseException", referenced from:
                _$LT$pyo3..exceptions..PyBaseException$u20$as$u20$pyo3..type_object..PyTypeInfo$GT$::type_object_raw::hf3fd9ce8d45bb1d5 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.1.rcgu.o)
            "_PyObject_Repr", referenced from:
                pyo3::types::any::PyAny::repr::h6e764a53a1c3bb33 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.8.rcgu.o)
            "_PyObject_SetAttrString", referenced from:
                pyo3::type_object::initialize_tp_dict::h38c0ade6ee2dd689 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.8.rcgu.o)
            "_PyObject_GetAttr", referenced from:
                pyo3::types::any::PyAny::getattr::_$u7b$$u7b$closure$u7d$$u7d$::h25e36f53e1803bbb in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.8.rcgu.o)
            "_PyLong_AsLong", referenced from:
                pyo3::types::num::_$LT$impl$u20$pyo3..conversion..FromPyObject$u20$for$u20$u8$GT$::extract::hfccf073d58243a7e in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.15.rcgu.o)
            "_PyUnicode_FromStringAndSize", referenced from:
                pyo3::types::string::PyString::new::hbd829df65bafcce7 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.14.rcgu.o)
            "_PyList_Append", referenced from:
                pyo3::types::list::PyList::append::_$u7b$$u7b$closure$u7d$$u7d$::had9367b763e78a6a in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.7.rcgu.o)
            "_PyObject_Free", referenced from:
                pyo3::pyclass::tp_init_additional::hc41681a131483556 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.4.rcgu.o
            "_PyExc_SystemError", referenced from:
                _$LT$pyo3..exceptions..PySystemError$u20$as$u20$pyo3..type_object..PyTypeInfo$GT$::type_object_raw::hf102b784e7c02b7a in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.1.rcgu.o)
            "_PyList_New", referenced from:
                pyo3::types::list::PyList::empty::h3c387d3ec88c48e6 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.7.rcgu.o)
            "_PyDict_New", referenced from:
                pyo3::types::dict::PyDict::new::h078416249d7977bb in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.7.rcgu.o)
            "_Py_IsInitialized", referenced from:
                pyo3::gil::GILGuard::acquire::_$u7b$$u7b$closure$u7d$$u7d$::h1f1574ad8006a9e6 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.7.rcgu.o)
            "_PyErr_GivenExceptionMatches", referenced from:
                pyo3::err::PyErr::is_instance::hd0af6010215c2eb5 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.13.rcgu.o)
            "__Py_NoneStruct", referenced from:
                pyo3::ffi::object::Py_None::h11515a69f276fa6b in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.9.rcgu.o)
            "_PyErr_NewException", referenced from:
                pyo3::err::PyErr::new_type::hb2222e7e809df23d in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.13.rcgu.o)
            "_PyExc_TypeError", referenced from:
                _$LT$pyo3..exceptions..PyTypeError$u20$as$u20$pyo3..type_object..PyTypeInfo$GT$::type_object_raw::hbd69f6586ab999f3 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.1.rcgu.o)
            "_PyTuple_SetItem", referenced from:
                pyo3::types::tuple::_$LT$impl$u20$pyo3..conversion..IntoPy$LT$pyo3..instance..Py$LT$pyo3..types..any..PyAny$GT$$GT$$u20$for$u20$$LP$T0$C$$RP$$GT$::into_py::h0736c870fe173006 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.11.rcgu.o)
                pyo3::types::tuple::_$LT$impl$u20$pyo3..conversion..IntoPy$LT$pyo3..instance..Py$LT$pyo3..types..any..PyAny$GT$$GT$$u20$for$u20$$LP$T0$C$$RP$$GT$::into_py::h7d6cfbcc1c527d02 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.14.rcgu.o)
            "_PyUnicode_AsEncodedString", referenced from:
                pyo3::types::string::PyString::to_string_lossy::heae192009201c3ad in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.14.rcgu.o)
            "_PyDict_SetItem", referenced from:
                pyo3::types::dict::PyDict::set_item::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hd12fe35f73fccedd in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.6.rcgu.o
            "_PyErr_Fetch", referenced from:
                pyo3::err::PyErr::fetch::he2e90f1c53969f8a in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.13.rcgu.o)
            "__Py_NotImplementedStruct", referenced from:
                pyo3::ffi::object::Py_NotImplemented::h446af0e179f01138 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.14.rcgu.o
            "_PyUnicode_AsUTF8AndSize", referenced from:
                pyo3::types::string::PyString::to_str::h21fdc806b6bb9479 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.3.rcgu.o
                pyo3::types::string::PyString::to_str::hd149b9babf09e49c in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.14.rcgu.o)
            "_PyObject_Malloc", referenced from:
                pyo3::pyclass::tp_init_additional::hc41681a131483556 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.4.rcgu.o
            "_PyObject_SetAttr", referenced from:
                pyo3::types::any::PyAny::setattr::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h1a4c583099b40966 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.6.rcgu.o
                pyo3::types::any::PyAny::setattr::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h27c82efc0c09851e in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.8.rcgu.o)
                pyo3::types::any::PyAny::setattr::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hb524b6692b92213c in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.8.rcgu.o)
            "_PyType_IsSubtype", referenced from:
                pyo3::ffi::object::PyObject_TypeCheck::h38c721ccf9c419c4 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.14.rcgu.o
            "_PyDict_Next", referenced from:
                _$LT$pyo3..types..dict..PyDictIterator$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next::h860e184d79564b0e in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.6.rcgu.o
            "_PyObject_Str", referenced from:
                pyo3::types::any::PyAny::str::h9f0c8ae693dc8fe4 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.8.rcgu.o)
            "_PyGILState_Ensure", referenced from:
                pyo3::gil::GILGuard::acquire_unchecked::h34e362b9598aeadd in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.7.rcgu.o)
            "_PyNumber_Index", referenced from:
                pyo3::types::num::_$LT$impl$u20$pyo3..conversion..FromPyObject$u20$for$u20$u8$GT$::extract::hfccf073d58243a7e in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.15.rcgu.o)
            "__Py_TrueStruct", referenced from:
                pyo3::ffi::boolobject::Py_True::h4c3622081745565d in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.11.rcgu.o
            "_PyBaseObject_Type", referenced from:
                _$LT$pyo3..pyclass_init..PyNativeTypeInitializer$LT$T$GT$$u20$as$u20$pyo3..pyclass_init..PyObjectInit$LT$T$GT$$GT$::into_new_object::h1394575e81701c68 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.14.rcgu.o
                _$LT$pyo3..pycell..PyCellBase$LT$U$GT$$u20$as$u20$pyo3..pycell..PyCellLayout$LT$T$GT$$GT$::tp_dealloc::h8ac029b2462fdeb8 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.3.rcgu.o
                _$LT$pyo3..types..any..PyAny$u20$as$u20$pyo3..type_object..PyTypeInfo$GT$::type_object_raw::hc0409148af23ac41 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.6.rcgu.o
            "_PyErr_NormalizeException", referenced from:
                pyo3::err::PyErr::normalized::h1d57f3fc9d50c136 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.13.rcgu.o)
            "_PyTuple_New", referenced from:
                pyo3::types::tuple::PyTuple::new::hf47320c28eed559f in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.3.rcgu.o
                pyo3::types::tuple::_$LT$impl$u20$pyo3..conversion..IntoPy$LT$pyo3..instance..Py$LT$pyo3..types..any..PyAny$GT$$GT$$u20$for$u20$$LP$T0$C$$RP$$GT$::into_py::h0736c870fe173006 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.11.rcgu.o)
                pyo3::types::tuple::_$LT$impl$u20$pyo3..conversion..IntoPy$LT$pyo3..instance..Py$LT$pyo3..types..any..PyAny$GT$$GT$$u20$for$u20$$LP$T0$C$$RP$$GT$::into_py::h7d6cfbcc1c527d02 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.14.rcgu.o)
            "__Py_FalseStruct", referenced from:
                pyo3::ffi::boolobject::Py_False::h1fb702c7e4702596 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.11.rcgu.o
            "_PyType_FromSpec", referenced from:
                pyo3::pyclass::create_type_object::h286b11f067397739 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.4.rcgu.o
            "_PyExc_AttributeError", referenced from:
                _$LT$pyo3..exceptions..PyAttributeError$u20$as$u20$pyo3..type_object..PyTypeInfo$GT$::type_object_raw::h6cb302f7aba2cbae in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.1.rcgu.o)
            "_PyType_GenericAlloc", referenced from:
                _$LT$pyo3..pyclass_init..PyNativeTypeInitializer$LT$T$GT$$u20$as$u20$pyo3..pyclass_init..PyObjectInit$LT$T$GT$$GT$::into_new_object::h1394575e81701c68 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.14.rcgu.o
            "__Py_Dealloc", referenced from:
                pyo3::ffi::object::Py_DECREF::hfc9ac5cb4c677ad9 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.14.rcgu.o
                pyo3::ffi::object::Py_DECREF::hb56235cf32aeef23 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.9.rcgu.o)
            "_PyModule_Create2", referenced from:
                pyo3::ffi::modsupport::PyModule_Create::h2da0bd53b5428218 in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.15.rcgu.o
            "_PyGILState_Release", referenced from:
                _$LT$pyo3..gil..GILGuard$u20$as$u20$core..ops..drop..Drop$GT$::drop::hdf51a16ccba4497f in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.7.rcgu.o)
            "_PyBytes_AsString", referenced from:
                pyo3::types::bytes::PyBytes::as_bytes::h0a92f94eeb6b9ce3 in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.10.rcgu.o)
            "_PyErr_Restore", referenced from:
                pyo3::err::PyErr::restore::hd62c8e4708e13dee in rithm-874fa952c0dfbfcd.rithm.0d752b03-cgu.6.rcgu.o
                pyo3::err::PyErr::restore::h336a50ff0290ce2a in libpyo3-3e4af82221a23595.rlib(pyo3-3e4af82221a23595.pyo3.874c2c37-cgu.13.rcgu.o)
          ld: symbol(s) not found for architecture x86_64
          clang: error: linker command failed with exit code 1 (use -v to see invocation)
          

error: could not compile `rithm` due to previous error
```
