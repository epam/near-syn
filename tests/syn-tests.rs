use assert_cmd::Command;
use std::io::Write;
use tempfile::{NamedTempFile, TempPath};

fn rust_test_files() -> Vec<TempPath> {
    vec![
        r#"

type AType = i32;

/// Doc-comments for a type def
type BType = i32;

/// Doc-comment line 1 for A
/// Doc-comment line 2 for A
/// Doc-comment line 3 for A
#[derive(Serialize)]
struct A {
    // No doc-comment for this field
    a1_field: U64,
    a2_field: U64,

    /// Line for a3
    /// Line for a2, then blank line
    ///
    /// Some markdown
    /// ```
    /// const a = [];
    /// const b = "";
    /// ```
    a3_field: U128,
}

// No doc-comment for this struct
#[derive(Serialize)]
struct B {
    b: U64,
}

/// non-serde enums are not exported
enum E0 {
    V1,
    V2,
}

/// doc-comment for enum
#[derive(Serialize)]
enum E {
    V1,
    V2,
}

#[near_bindgen]
struct C {
    f128: U128,
}

#[near_bindgen]
impl C {
    /// init func
    #[init]
    pub fn init_here(f128: U128) -> Self {
        Self {
            f128,
        }
    }

    /// Line 1 for get_f128 first
    /// Line 2 for get_f128 second
    pub fn get_f128(&self) -> U128 {
        self.f128
    }

    // Regular comments are not transpiled
    /// Set f128.
    pub fn set_f128(&mut self, value: U128) {
        self.f128 = value;
    }

    pub fn get_f128_other_way(&self, key: U128) -> U128 {
        self.f128 + key
    }

    pub fn more_types(&mut self, key: U128, tuple: (String, BTreeSet<i32>) ) -> () {
        self.f128 = key;
    }

    /// Pay to set f128.
    #[payable]
    pub fn set_f128_with_sum(&mut self, a_value: U128, other_value: U128) {
        self.f128 = a_value + other_value;
    }

    #[private]
    pub fn marked_as_private(&mut self) {
    }

    fn private_method_not_exported(&self, value: U128) -> U128 {
        self.f128
    }

    fn private_mut_method_not_exported(&mut self, value: U128) {
        self.f128 = value;
    }

}


#[near_bindgen]
impl C {
    /// another impl
    pub fn another_impl(&self, f128: U128) -> U128 {
        f128
    }
}

// All methods for traits are public, and thus exported
#[near_bindgen]
impl I for C {
    /// Single-line comment for get
    fn get(&self) -> U128 {
        self.f128
    }
}

// Omitted since near-bindgen is not present, methods not exported
impl J for C {
    fn m() {

    }
}

// Omitted since even near-bindgen is present, methods are private
#[near_bindgen]
impl K for C {
    #[private]
    fn p() {

    }
}

mod inner_mod {
    type A_in_mod = u32;
}

"#,
        r#"
#[derive(Serialize)]
struct S {
    f: i32
}
        "#,
        r#"
#[near_bindgen]
impl S {
    pub fn get(&self) -> i32{
        self.f
    }
}

#[derive(Serialize)]
struct T(u32, bool);

#[derive(Serialize)]
struct U(AccountId);

        "#,
    ]
    .into_iter()
    .map(|content| {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", content).unwrap();
        file.flush().unwrap();
        file.into_temp_path()
    })
    .collect()
}

#[test]
fn check_version() {
    let mut cmd = Command::cargo_bin("near-syn").unwrap();
    cmd.arg("--version")
        .assert()
        .code(0)
        .stdout(format!("near-syn {}\n", env!("CARGO_PKG_VERSION")));
}

mod ts {

    use super::rust_test_files;
    use assert_cmd::Command;

    fn output(defs: &str, name: &str, view_methods: &str, change_methods: &str) -> String {
        format!(
            r#"// TypeScript bindings generated with near-syn v{} {}

// Exports common NEAR Rust SDK types
export type U64 = string;
export type I64 = string;
export type U128 = string;
export type I128 = string;
export type AccountId = string;
export type ValidAccountId = string;
{}
export const {}Methods = {{
    viewMethods: [{}
    ],
    changeMethods: [{}
    ],
}};
"#,
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_REPOSITORY"),
            defs,
            name,
            view_methods
                .split_terminator(",")
                .map(|s| format!("\n        {:?},", s))
                .collect::<Vec<String>>()
                .join(""),
            change_methods
                .split_terminator(",")
                .map(|s| format!("\n        {:?},", s))
                .collect::<Vec<String>>()
                .join(""),
        )
    }

    fn near_ts() -> Command {
        let mut cmd = Command::cargo_bin("near-syn").unwrap();
        cmd.arg("ts");
        cmd.arg("--no-now");
        cmd
    }

    #[test]
    fn transpile_zero_rust_files_to_ts() {
        let mut cmd = near_ts();
        cmd.assert().code(0).stdout(output("", "", "", ""));
    }

    #[test]
    fn transpile_single_rust_file_to_ts() {
        let paths = rust_test_files();

        let mut cmd = near_ts();
        cmd.arg(paths[0].to_str().unwrap())
        .assert()
        .code(0)
        .stdout(output(
            r#"
/**
 */
export type AType = number;

/**
 *  Doc-comments for a type def
 */
export type BType = number;

/**
 *  Doc-comment line 1 for A
 *  Doc-comment line 2 for A
 *  Doc-comment line 3 for A
 */
export type A = {
    /**
     */
    a1_field: U64;

    /**
     */
    a2_field: U64;

    /**
     *  Line for a3
     *  Line for a2, then blank line
     * 
     *  Some markdown
     *  ```
     *  const a = [];
     *  const b = "";
     *  ```
     */
    a3_field: U128;

}

/**
 */
export type B = {
    /**
     */
    b: U64;

}

/**
 *  doc-comment for enum
 */
export enum E {
    /**
     */
    V1,

    /**
     */
    V2,

}

/**
 */
export interface C {
    /**
     *  init func
     */
    init_here: { f128: U128 };

    /**
     *  Line 1 for get_f128 first
     *  Line 2 for get_f128 second
     */
    get_f128(): Promise<U128>;

    /**
     *  Set f128.
     */
    set_f128(args: { value: U128 }, gas?: any): Promise<void>;

    /**
     */
    get_f128_other_way(args: { key: U128 }): Promise<U128>;

    /**
     */
    more_types(args: { key: U128, tuple: [string, number[]] }, gas?: any): Promise<void>;

    /**
     *  Pay to set f128.
     */
    set_f128_with_sum(args: { a_value: U128, other_value: U128 }, gas?: any, amount?: any): Promise<void>;

}

/**
 */
export interface C {
    /**
     *  another impl
     */
    another_impl(args: { f128: U128 }): Promise<U128>;

}

/**
 */
export interface I {
    /**
     *  Single-line comment for get
     */
    get(): Promise<U128>;

}

/**
 */
export type A_in_mod = number;

export interface C extends I {}
"#,
            "C",
            "get_f128,get_f128_other_way,another_impl,get",
            "set_f128,more_types,set_f128_with_sum",
        ));

        paths.into_iter().for_each(|path| path.close().unwrap());
    }

    #[test]
    fn transpile_multiple_rust_files_to_ts() {
        let paths = rust_test_files();

        let mut cmd = near_ts();
        cmd.args(&paths[1..]).assert().code(0).stdout(output(
            r#"
/**
 */
export type S = {
    /**
     */
    f: number;

}

/**
 */
export interface S {
    /**
     */
    get(): Promise<number>;

}

/**
 */
export type T = [number, boolean];

/**
 */
export type U = AccountId;
"#,
            "S",
            "get",
            "",
        ));

        paths.into_iter().for_each(|path| path.close().unwrap());
    }
}
mod md {

    use super::rust_test_files;
    use assert_cmd::Command;

    fn output(text: &str) -> String {
        format!(
            r#"<!-- AUTOGENERATED doc, do not modify! -->
# Contract

{}
---

References

- :rocket: Initialization method. Needs to be called right after deployment.
- :eyeglasses: View only method, *i.e.*, does not modify the contract state.
- :writing_hand: Call method, i.e., does modify the contract state.
- &#x24C3; Payable method, i.e., call needs to have an attached NEAR deposit.

---

*This documentation was generated with* **near-syn v{}** <{}>
"#,
            text,
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_REPOSITORY"),
        )
    }

    fn near_md() -> Command {
        let mut cmd = Command::cargo_bin("near-syn").unwrap();
        cmd.arg("md");
        cmd.arg("--no-now");
        cmd
    }

    #[test]
    fn transpile_zero_rust_files_to_md() {
        let mut cmd = near_md();
        cmd.assert().code(0).stdout(output(""));
    }

    #[test]
    fn transpile_single_rust_file_to_doc() {
        let paths = rust_test_files();

        let mut cmd = near_md();
        cmd.arg(paths[0].to_str().unwrap())
        .assert()
        .code(0)
        .stdout(output(
            r#"
## Methods for C

### :rocket: `init_here` (*constructor*)

```typescript
init_here: { f128: U128 };
```

init func

### :eyeglasses: `get_f128`

```typescript
get_f128(): Promise<U128>;
```

Line 1 for get_f128 first
Line 2 for get_f128 second

### :writing_hand: `set_f128`

```typescript
set_f128(args: { value: U128 }, gas?: any): Promise<void>;
```

Set f128.

### :eyeglasses: `get_f128_other_way`

```typescript
get_f128_other_way(args: { key: U128 }): Promise<U128>;
```


### :writing_hand: `more_types`

```typescript
more_types(args: { key: U128, tuple: [string, number[]] }, gas?: any): Promise<void>;
```


### &#x24C3; `set_f128_with_sum`

```typescript
set_f128_with_sum(args: { a_value: U128, other_value: U128 }, gas?: any, amount?: any): Promise<void>;
```

Pay to set f128.

## Methods for C

### :eyeglasses: `another_impl`

```typescript
another_impl(args: { f128: U128 }): Promise<U128>;
```

another impl

## Methods for `I` interface

### :eyeglasses: `get`

```typescript
get(): Promise<U128>;
```

Single-line comment for get
"#,
        ));

        paths.into_iter().for_each(|path| path.close().unwrap());
    }
}
