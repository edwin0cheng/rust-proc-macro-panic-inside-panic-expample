# Rust proc_macro panic inside panic example

This example project demonstrates a strange panic that happens inside 
procedural macros when they are compiled as dynamic libraries, 
dynamically linked and then called explicitly using 
`proc_macro::bridge::client`.

This example is expected to work only on Linux machines for now.

## How to see the panic

To get the error you should run the single test from `src/main.rs` file.

```
> RUST_BACKTRACE=1 cargo +nightly test -- --nocapture
```

When you do that, the test will be aborted, and you should see the next stack trace:

```
thread 'test_getset_expansion' panicked at 'procedural macro API is used outside of a procedural macro', src/libproc_macro/bridge/client.rs:315:17
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:197
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   4: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   5: std::panicking::begin_panic
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panicking.rs:408
   6: <proc_macro::TokenStream as core::str::FromStr>::from_str
             at src/libproc_macro/bridge/client.rs:315
             at src/libproc_macro/bridge/client.rs:285
             at src/libproc_macro/bridge/scoped_cell.rs:73
             at src/libproc_macro/bridge/client.rs:283
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/local.rs:299
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/local.rs:245
             at src/libproc_macro/bridge/client.rs:282
             at src/libproc_macro/bridge/client.rs:313
             at src/libproc_macro/bridge/client.rs:229
             at src/libproc_macro/lib.rs:101
   7: core::str::<impl str>::parse
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libcore/str/mod.rs:3920
   8: test_proc_macro::make_answer_macro
             at src/lib.rs:8
   9: std::panicking::try::do_call
             at src/libproc_macro/bridge/client.rs:358
             at src/libproc_macro/bridge/scoped_cell.rs:78
             at src/libproc_macro/bridge/scoped_cell.rs:73
             at src/libproc_macro/bridge/scoped_cell.rs:78
             at src/libproc_macro/bridge/client.rs:309
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/local.rs:299
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/local.rs:245
             at src/libproc_macro/bridge/client.rs:309
             at src/libproc_macro/bridge/client.rs:351
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panic.rs:309
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panicking.rs:293
  10: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:85
  11: proc_macro::bridge::client::__run_expand1
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panicking.rs:272
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panic.rs:388
             at src/libproc_macro/bridge/client.rs:350
  12: <proc_macro::bridge::server::SameThread as proc_macro::bridge::server::ExecutionStrategy>::run_bridge_and_client
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libproc_macro/bridge/server.rs:151
  13: proc_macro::bridge::server::run_server
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libproc_macro/bridge/server.rs:287
  14: proc_macro::bridge::server::<impl proc_macro::bridge::client::Client<fn(proc_macro::TokenStream) .> proc_macro::TokenStream>>::run
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libproc_macro/bridge/server.rs:304
  15: rust_proc_macro_panic_inside_panic_expample::test_getset_expansion
             at src/main.rs:146
  16: rust_proc_macro_panic_inside_panic_expample::test_getset_expansion::{{closure}}
             at src/main.rs:127
  17: core::ops::function::FnOnce::call_once
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libcore/ops/function.rs:231
  18: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/liballoc/boxed.rs:704
  19: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:85
  20: test::run_test::run_test_inner::{{closure}}
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panicking.rs:272
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panic.rs:388
             at src/libtest/lib.rs:1468
thread 'test_getset_expansion' panicked at 'procedural macro API is used outside of a procedural macro', src/libproc_macro/bridge/client.rs:315:17
stack backtrace:
   0:     0x5592e3161ae3 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h2d6422e3b3d3b496
                               at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1:     0x5592e315cbdb - std::sys_common::backtrace::_print::hdcf024b79128e394
                               at src/libstd/sys_common/backtrace.rs:71
   2:     0x5592e3160716 - std::panicking::default_hook::{{closure}}::h65908dc32223ffbc
                               at src/libstd/sys_common/backtrace.rs:59
                               at src/libstd/panicking.rs:197
   3:     0x5592e31604a9 - std::panicking::default_hook::h8163b58f57533e2a
                               at src/libstd/panicking.rs:211
   4:     0x5592e3160f08 - std::panicking::rust_panic_with_hook::h5afaf61eb853c476
                               at src/libstd/panicking.rs:478
   5:     0x5592e3143f84 - std::panicking::begin_panic::h201a5532d0d32131
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panicking.rs:408
   6:     0x5592e3145a87 - proc_macro::bridge::client::BridgeState::with::{{closure}}::{{closure}}::h43cd42933bf05d3f
                               at src/libproc_macro/bridge/client.rs:315
                               at src/libproc_macro/bridge/client.rs:285
   7:     0x5592e314c3de - <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop::h3acf18fccae34825
                               at src/libproc_macro/bridge/scoped_cell.rs:73
                               at src/libproc_macro/bridge/client.rs:283
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/local.rs:299
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/local.rs:245
                               at src/libproc_macro/bridge/client.rs:282
                               at src/libproc_macro/bridge/client.rs:313
                               at src/libproc_macro/bridge/client.rs:229
                               at src/libproc_macro/bridge/client.rs:54
   8:     0x7fa36b46956e - core::ptr::real_drop_in_place::hb58c8d7f591e2c59
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libcore/ptr.rs:195
   9:     0x7fa36b46955d - core::ptr::real_drop_in_place::h39c30b4f13d1f345
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libcore/ptr.rs:195
  10:     0x7fa36b46963a - test_proc_macro::make_answer_macro::hdf69bc05a4e04cef
                               at src/lib.rs:9
  11:     0x7fa36b46a1e3 - std::panicking::try::do_call::h71ee8d644732cdd4
                               at src/libproc_macro/bridge/client.rs:358
                               at src/libproc_macro/bridge/scoped_cell.rs:78
                               at src/libproc_macro/bridge/scoped_cell.rs:73
                               at src/libproc_macro/bridge/scoped_cell.rs:78
                               at src/libproc_macro/bridge/client.rs:309
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/local.rs:299
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/local.rs:245
                               at src/libproc_macro/bridge/client.rs:309
                               at src/libproc_macro/bridge/client.rs:351
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panic.rs:309
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panicking.rs:293
  12:     0x5592e3168a59 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:85
  13:     0x7fa36b46fd84 - proc_macro::bridge::client::__run_expand1::h7aa14ea7adb2775b
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panicking.rs:272
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panic.rs:388
                               at src/libproc_macro/bridge/client.rs:350
  14:     0x5592e2eebb1a - <proc_macro::bridge::server::SameThread as proc_macro::bridge::server::ExecutionStrategy>::run_bridge_and_client::hf482ceabe96b0313
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libproc_macro/bridge/server.rs:151
  15:     0x5592e2eec7e2 - proc_macro::bridge::server::run_server::hd01705c424e0f87a
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libproc_macro/bridge/server.rs:287
  16:     0x5592e2ed9905 - proc_macro::bridge::server::<impl proc_macro::bridge::client::Client<fn(proc_macro::TokenStream) .> proc_macro::TokenStream>>::run::h85b859d348785565
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libproc_macro/bridge/server.rs:304
  17:     0x5592e2f277b8 - rust_proc_macro_panic_inside_panic_expample::test_getset_expansion::hc04da47dab7f924d
                               at src/main.rs:146
  18:     0x5592e2eea621 - rust_proc_macro_panic_inside_panic_expample::test_getset_expansion::{{closure}}::h1bc35a164ad2a2b7
                               at src/main.rs:127
  19:     0x5592e2ef752d - core::ops::function::FnOnce::call_once::h5ce974b491e02e43
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libcore/ops/function.rs:231
  20:     0x5592e2fad89e - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::he4b816e4551f57f6
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/liballoc/boxed.rs:704
  21:     0x5592e3168a59 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:85
  22:     0x5592e2fc8227 - test::run_test::run_test_inner::{{closure}}::h3c0a5c57d6a7311e
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panicking.rs:272
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panic.rs:388
                               at src/libtest/lib.rs:1468
  23:     0x5592e2fa2be4 - std::sys_common::backtrace::__rust_begin_short_backtrace::he2dc1dc77c2b7d25
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/sys_common/backtrace.rs:136
  24:     0x5592e2fa6c34 - std::panicking::try::do_call::hc7daa2c4e69eadc1
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/mod.rs:469
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panic.rs:309
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panicking.rs:293
  25:     0x5592e3168a59 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:85
  26:     0x5592e2fa7251 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h96e6ceca33de04ae
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panicking.rs:272
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panic.rs:388
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/mod.rs:468
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libcore/ops/function.rs:231
  27:     0x5592e315361e - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hd5bc1c518e080a98
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/liballoc/boxed.rs:704
  28:     0x5592e316813f - std::sys::unix::thread::Thread::new::thread_start::h5c9e12179bf11e38
                               at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/liballoc/boxed.rs:704
                               at src/libstd/sys_common/thread.rs:13
                               at src/libstd/sys/unix/thread.rs:79
  29:     0x7fa36c2386b9 - start_thread
  30:     0x7fa36bd5641c - clone
  31:                0x0 - <unknown>
thread panicked while panicking. aborting.
```

The panic inside panic happens when destructors of `TokenStream` are running. 

However, if you will try and compile tests with the compiler that is different
from the current nightly (if your nightly is of 2019-05-06, then 2019-05-05 will
do), you will see no such error - the test will be successful.

```
> RUST_BACKTRACE=1 cargo +nightly-2019-05-06

thread 'test_getset_expansion' panicked at 'not yet implemented', src/rustc_server.rs:229:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:197
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   4: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   5: std::panicking::begin_panic
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libstd/panicking.rs:408
   6: <rust_proc_macro_panic_inside_panic_expample::rustc_server::Rustc as proc_macro::bridge::server::TokenStream>::from_str
             at src/rustc_server.rs:229
   7: <proc_macro::bridge::server::MarkedTypes<S> as proc_macro::bridge::server::TokenStream>::from_str
             at src/main.rs:1
   8: <proc_macro::bridge::server::Dispatcher<proc_macro::bridge::server::MarkedTypes<S>> as proc_macro::bridge::server::DispatcherTrait>::dispatch::{{closure}}
             at src/main.rs:1
   9: core::ops::function::FnOnce::call_once
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libcore/ops/function.rs:231
  10: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libstd/panic.rs:309
  11: std::panicking::try::do_call
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libstd/panicking.rs:293
  12: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:85
  13: std::panicking::try
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libstd/panicking.rs:272
  14: std::panic::catch_unwind
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libstd/panic.rs:388
  15: <proc_macro::bridge::server::Dispatcher<proc_macro::bridge::server::MarkedTypes<S>> as proc_macro::bridge::server::DispatcherTrait>::dispatch
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libproc_macro/bridge/server.rs:113
  16: <proc_macro::bridge::server::SameThread as proc_macro::bridge::server::ExecutionStrategy>::run_bridge_and_client::{{closure}}
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libproc_macro/bridge/server.rs:149
  17: <proc_macro::bridge::closure::Closure<A,R> as core::convert::From<&mut F>>::from::call
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libproc_macro/bridge/closure.rs:19
  18: <proc_macro::TokenStream as core::str::FromStr>::from_str
             at src/libproc_macro/bridge/closure.rs:30
             at src/libproc_macro/bridge/client.rs:236
             at src/libproc_macro/bridge/client.rs:320
             at src/libproc_macro/bridge/client.rs:285
             at src/libproc_macro/bridge/scoped_cell.rs:73
             at src/libproc_macro/bridge/client.rs:283
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/local.rs:299
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/local.rs:245
             at src/libproc_macro/bridge/client.rs:282
             at src/libproc_macro/bridge/client.rs:313
             at src/libproc_macro/bridge/client.rs:229
             at src/libproc_macro/lib.rs:101
  19: core::str::<impl str>::parse
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libcore/str/mod.rs:3920
  20: test_proc_macro::make_answer_macro
             at src/lib.rs:8
  21: std::panicking::try::do_call
             at src/libproc_macro/bridge/client.rs:358
             at src/libproc_macro/bridge/scoped_cell.rs:78
             at src/libproc_macro/bridge/scoped_cell.rs:73
             at src/libproc_macro/bridge/scoped_cell.rs:78
             at src/libproc_macro/bridge/client.rs:309
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/local.rs:299
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/thread/local.rs:245
             at src/libproc_macro/bridge/client.rs:309
             at src/libproc_macro/bridge/client.rs:351
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panic.rs:309
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panicking.rs:293
  22: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:85
  23: proc_macro::bridge::client::__run_expand1
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panicking.rs:272
             at /rustc/a3404557c54ea48fb8efc805d93c450beb3364d4/src/libstd/panic.rs:388
             at src/libproc_macro/bridge/client.rs:350
  24: <proc_macro::bridge::server::SameThread as proc_macro::bridge::server::ExecutionStrategy>::run_bridge_and_client
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libproc_macro/bridge/server.rs:151
  25: proc_macro::bridge::server::run_server
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libproc_macro/bridge/server.rs:287
  26: proc_macro::bridge::server::<impl proc_macro::bridge::client::Client<fn(proc_macro::TokenStream) .> proc_macro::TokenStream>>::run
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libproc_macro/bridge/server.rs:304
  27: rust_proc_macro_panic_inside_panic_expample::test_getset_expansion
             at src/main.rs:146
  28: rust_proc_macro_panic_inside_panic_expample::test_getset_expansion::{{closure}}
             at src/main.rs:127
  29: core::ops::function::FnOnce::call_once
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libcore/ops/function.rs:231
  30: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/liballoc/boxed.rs:704
  31: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:85
  32: test::run_test::run_test_inner::{{closure}}
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libstd/panicking.rs:272
             at /rustc/597f432489f12a3f33419daa039ccef11a12c4fd/src/libstd/panic.rs:388
             at src/libtest/lib.rs:1468
test test_getset_expansion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
``` 

Now the panic is expected, because, as you can see, `src/rustc_server` contains no actual implementation of `TokenStream` 
API - it is completely stubbed for the simplicity purpose. Moreover, panic 
is correctly caught and returned as an `Error` from the `run` method.

## Notes

I've experimented with different compiler versions, and it seems 
that the only thing that triggers such panic is version alignment 
between compiler that compiled procedural macro itself, and compiler
that compiled the code that is calling `run` method. 

As long as those compilers have different versions (and they generate 
the same ABI for the procedural macros), everything works fine. Moreover, 
many popular procedural macros may be called like that (for example `serde_derive`,
`getset`, `builder_derive` etc.). 

Unfortunately, I have no idea why this panic happens (maybe it's even supposed 
to happen), and why it happens only under those strange circumstances.