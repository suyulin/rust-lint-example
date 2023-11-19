# Install
- [lefthook](https://github.com/evilmartians/lefthook/)  
- [commitlint-rs](https://keisukeyamashita.github.io/commitlint-rs/)


## Run
Run `lefthook install` in the workspace  
Test commit
```
❯ git commit -a -m "feat: add sub fun"
╭───────────────────╮
│ 🥊 lefthook v1.5.2  hook: pre-commit │
╰───────────────────╯
┃  clippy ❯

    Checking hello_lint v0.1.0 (/Users/suyulin/work/github/test/hello_lint)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s

┃  rustfmt ❯


┃  test ❯

   Compiling hello_lint v0.1.0 (/Users/suyulin/work/github/test/hello_lint)
    Finished test [unoptimized + debuginfo] target(s) in 0.43s
     Running unittests src/main.rs (target/debug/deps/hello_lint-c2928c8b777d40ff)

running 2 tests
test test_sub ... ok
test test_add ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s



  ──────────────────
summary: (done in 1.50 seconds)
✔️  clippy
✔️  rustfmt
✔️  test
╭───────────────────╮
│ 🥊 lefthook v1.5.2  hook: commit-msg │
╰───────────────────╯
┃  commitlint ❯



  ──────────────────
summary: (done in 0.01 seconds)
✔️  commitlint
[main b4c7995] feat: add sub fun
 1 file changed, 9 insertions(+), 1 deletion(-)
```

