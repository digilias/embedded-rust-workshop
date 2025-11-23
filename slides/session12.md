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

* Existing battle-tested libraries (compression, crypto, DSP)
* Hardware vendor SDKs and drivers
* Legacy code you need to reuse
* Performance-critical code already optimized

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

# bindgen - Automatic binding generation

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

# C-style error handling

```c
// C code
int do_operation(int* result) {
    if (error_condition) {
        return -1;  // Error code
    }
    *result = 42;
    return 0;  // Success
}
```

---

# Wrapping C errors in Rust

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

# Memory management challenges

```c
// C code - who owns this memory?
char* get_string(void);
void free_string(char* str);
```

```rust
// Rust wrapper
fn get_c_string() -> String {
    unsafe {
        let ptr = get_string();
        let c_str = std::ffi::CStr::from_ptr(ptr);
        let rust_string = c_str.to_string_lossy().into_owned();
        free_string(ptr);  // Must remember to free!
        rust_string
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

* Wrap a simple C math library
* Use `cc` crate to build C code
* Create safe Rust API around unsafe C functions
* Handle C error codes properly
* Test on hardware
