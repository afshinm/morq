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

### LengthOf

To check the count of elements in an iterator

```rust
expect(vec![1, 2, 3].iter()).to.not.have.lengthOf(1usize);
expect(0..3).to.have.lengthOf(3usize);
```

### Contain

Given iterator must contain the element

```rust
expect(vec![1, 2, 3].iter()).to.contain(&2);
expect(vec![false, false].iter()).to.not.contain(&true);
```

### Ok / Err

To check a Result enum

```rust
let res: Result<String, String> = Ok(format!("boo"));

morq!(
    expect(res).to.be.ok();
);
```

```rust
let res: Result<String, String> = Err(format!("boo"));

morq!(
    expect(res).to.be.err();
);
```

Of course, you can combine it with `not`:


```rust
let res: Result<String, String> = Err(format!("boo"));

morq!(
    expect(res).to.not.be.ok();
);
```

## Roadmap

- Adding more chain rules
- Adding more assert (terminal) 
- Adding alias for existing terminal rules (e.g. `close_to` alias for `close`)
- Ability to add two or more asserts in one chain:

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

MIT

Inspired by http://chaijs.com and https://github.com/carllerche/hamcrest-rust
