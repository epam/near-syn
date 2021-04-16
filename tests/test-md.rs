use assert_cmd::Command;
use input::rust_test_files;

mod input;

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

*This documentation was generated with* **near-md v{}** <{}>**
"#,
        text,
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_REPOSITORY"),
    )
}

fn near_md() -> Command {
    let mut cmd = Command::cargo_bin("near-md").unwrap();
    cmd.arg("--no-now");
    cmd
}

#[test]
fn check_version() {
    let mut cmd = near_md();
    cmd.arg("--version")
        .assert()
        .code(0)
        .stdout(format!("near-md {}\n", env!("CARGO_PKG_VERSION")));
}

#[test]
fn transpile_zero_rust_files_to_doc() {
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
