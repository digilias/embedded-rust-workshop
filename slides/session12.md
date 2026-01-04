---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 12: C Library Integration

* **Goal:** Wrapping an existing C library safely in Rust

---

# Why integrate with C?

* Very battle tested complex drivers you don't want to rewrite
* Binary blobs you need to use
* Personal opinion: If you can rewrite, it's probably better to do that longer term

---

# Calling C from Rust

```rust
extern "C" {
    fn c_function(x: i32) -> i32;
}

fn rust_function() {
    let result = unsafe {
        c_function(42)
    };
}
```

* All C function calls are unsafe
* No Rust safety guarantees across FFI boundary

---

# Calling Rust from C

```rust

// As long as they match a C struct, it works
#[repr(C)] // IMPORTANT
pub struct Foo {
    val: u32,
}

#[unsafe(no_mangle)]
pub extern "C" fn add_foo(foo: *const Foo, val: u32) {
    // Dereference foo and add value
}
```

* Still no Rust safety guarantees across FFI boundary

---

# bindgen - binding generation at build time

```toml
[build-dependencies]
bindgen = "0.72.1"
```

```rust
// build.rs
fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
```

---

# Building C code with Rust

```rust
// build.rs
fn main() {
    cc::Build::new()
        .file("src/math_lib.c")
        .compile("mathlib");

    println!("cargo:rerun-if-changed=src/math_lib.c");
}
```

```toml
[build-dependencies]
cc = "1.0"
```

---

# Example: C-style error handling

```c
int do_operation(int* result) {
    if (error_condition) {
        return -1;  // Error code
    }
    *result = 42;
    return 0;  // Success
}
```

---

# Example: Wrapping C errors in Rust

```rust
fn safe_do_operation() -> Result<i32, CError> {
    let mut result: i32 = 0;
    let ret = unsafe {
        do_operation(&mut result as *mut i32)
    };

    if ret < 0 {
        Err(CError::from_code(ret))
    } else {
        Ok(result)
    }
}
```

---

# Creating safe abstractions

```rust
pub struct MathLib {
    _private: (),  // Cannot be constructed outside
}

impl MathLib {
    pub fn new() -> Self {
        unsafe {
            math_lib_init();
        }
        MathLib { _private: () }
    }

    pub fn compute(&self, x: f32) -> f32 {
        unsafe { math_lib_compute(x) }
    }
}

impl Drop for MathLib {
    fn drop(&mut self) {
        unsafe { math_lib_cleanup(); }
    }
}
```

---

# Exercise

* Make use of the lowpass_filter library in the `practice/libs` folder
