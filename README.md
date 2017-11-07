# morq
Write unit tests like humans

[![Build Status](https://travis-ci.com/afshinm/morq.svg?token=ACapzCFmBh92g8rnRsrq&branch=master)](https://travis-ci.com/afshinm/morq)

<img src="http://clipart-library.com/images/qcBAnE68i.png" width="200" />


## Grammar

Since we are using a macro here, you need to wrap the following rules in `morq!` macro.

Example:  

```rust
morq!(
  expect(3).to.be.a(i32);
);
```

### Chains

You use following chains to make the assertions more user friendly and readable.

 - to
 - be
 - have

### Equal

```rust
expect(30).to.be.equal(10 * 3);
expect(3).to.be.equal(1 + 2);
```

### Close

To compare two given `float` values

```rust
expect(3f32).to.be.close(3.0001f32);
```

### Not

Negates the chain.


```rust
expect(30).to.not.be.equal(10);
expect(3).to.not.be.equal(1);
expect(vec![1, 2, 3]).to.not.be.a(Vec<char>);
```

### A / An

To check the data type.

```rust
expect(30).to.be.an(i32);
expect("hola".to_string()).to.not.be.a(f32);
expect(vec![1, 2, 3]).to.be.a(Vec<i32>);
```

### Empty

To check and see if the iterator is empty or not

```rust
expect(vec![1, 2, 3].iter()).to.not.be.empty();
expect(0..2).to.not.be.empty();
```

## Roadmap

- Adding more chain rules
- Adding more assert (terminal) 
- Ability to add two asserts in one chain:

  ```rust
  expect("hello").to.be.equal("hello").and.not.be.a(i32);
  ```

## FAQ

### morq?

Means chicken in Farsi. Like a lazy chicken, you know.  

Artwork: clipart-library.com

### Author

Afshin Mehrabani

### License

GPLv3  

Inspired by http://chaijs.com and https://github.com/carllerche/hamcrest-rust
