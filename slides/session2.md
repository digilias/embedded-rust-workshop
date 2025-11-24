---
marp: true
theme: default
paginate: true
backgroundColor: #fff
color: #333
---

# Session 2: PAC

* **Goal:** Introducing using peripherals with the Peripheral Access Crate (PAC) concept and using that to work with registers.

---
# PACs in the wild

* [stm32-rs](https://github.com/stm32-rs)
  - Oldest
  - Separate crates for each chip family
  - Level of features supported vary greatly
  - Using svd2rust
* [stm32-metapac](https://crates.io/crates/stm32-metapac)
  - Supports all STM32 variants
  - Chip variant selected using feature flag
  - Used by the embassy-stm32 HAL
  - Using chiptool
   

---
# Source of truth

* [stm32-data](https://github.com/embassy-rs/stm32-data)
* Scripts for downloading metadata from vendors
* Rust build scripts for generating Rust code from vendor sources
* Applies transforms to treat arrays, fields in a specific way

---
# Transformers

```toml
  - !MakeFieldArray
    fieldsets: IBIDR
    from: ^(IBIDB)\d$
    to: $1

  - !MergeFieldsets
    from: ^(DEVR)[1-4]$
    to: $1

  - !MakeRegisterArray
    blocks: I3C
    from: ^(DEVR)[1-4]$
    to: $1

  - !RenameFields
    fieldset: ^MAX[RW]LR$
    from: ^M[RW]L$
    to: ML
```


---

# Output

* Generated data -> [stm32-data-generated](https://github.com/embassy-rs/stm32-data-generated)
* A feature support table 
* JSON description of chip ([example](https://github.com/embassy-rs/stm32-data-generated/blob/main/data/chips/STM32H563ZI.json))

---

# 🤯

* "Do I need to understand all this?!"
* No
* But, these are (so far) maintained by community
* So if you need support for some peripheral, this is where you go to add it.

---

# The meatpack

* [stm32-metapac](https://crates.io/crates/stm32-metapac)
* In a nutshell: An advanced Rust build-time dependency (build.rs) that reads JSON and generates Rust code from it.

---

# Cargo.toml

```toml
stm32-metapac = { version = "18", features = ["stm32h563zi"] }
```

---
# Where's the code?

* `find target -type f -name pac.rs`
* Works with IDE completion

---

# General info

---

# The workshop project

* Located in the `practice/workshop` folder
* We will use this throughout the workshop and update it
* Contains necessary deps in `Cargo.toml`
* Help snippets can be found in `practice/workshop` - please use these if you get stuck
* README.md contains references to relevant docs for each session

---

# Exercise

* Start with the workshop project
* Create an instance of the embassy stm32 HAL
* Create an instance of i2c
* Read the who am i register
* Run on devkit
