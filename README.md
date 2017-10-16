# morq
Write unit tests like humans

<img src="http://clipart-library.com/images/qcBAnE68i.png" width="200" />



## Grammar

Since we are using a macro here, you need to wrap the following rules in `morq!` macro.

Example:  

```rust
morq!(
  expect(3).to.a(i32);
);
```


### Chains

You use following chains to make the assertions more user friendly and readable.

 - to
 - be
 - been
 - is
 - that
 - which
 - and
 - has
 - have
 - with
 - at
 - of
 - same
 - but
 - does

### Equal

```rust
expect(30).to.equal(10 * 3);
expect(3).to.equal(1 + 2);
```

### Not

Negates the chain.


```rust
expect(30).to.not.equal(10);
expect(3).to.not.equal(1);
```

### A / An

To check the type.

```rust
expect(30).to.be.a(i32);
expect("hola".to_string()).to.not.be.a(f32);
```

## FAQ

### morq?

Means chicken in Farsi. Like a lazy chicken, you know.

### Author

Afshin Mehrabani

### License

GPLv3  

Inspired by http://chaijs.com and https://github.com/carllerche/hamcrest-rust
