# Rust_hw3

由于`hash_map`声明式宏的实现和`my_stack`关于`RefCell`的应用思路较为直接明了，在此只阐述`MyRc`的实现思路

## MyRc

由于涉及到多个变量对同一值的所有权，因此需要引入指针来实现相关功能，如此可以实现较全面的功能——其中一个有相应所有权的变量离开作用域后所有具有所有权的变量的`strong_count`都会减一（具体见`my_rc/main.rs`中的测试）。

具体地，通过`Box::into_raw()`实现指针的操作，因为涉及到`raw pointer`，是`unsafe Rust`的语法，相关操作需要在`unsafe{}`块中实现。

```rust
pub struct MyRc<T> {
    value: *mut T,
    strong_count: *mut usize,
}
```

------

### Method

#### fn new(value: T) -> Self

传入值，返回`MyRc`变量

```rust
pub fn new(value: T) -> Self {
    let value = Box::new(value);
    let strong_count = Box::new(1);

    Self {
        value: Box::into_raw(value),
        strong_count: Box::into_raw(strong_count),
    }
}
```

#### fn strong_count(&self) -> usize

返回`strong_count`成员变量

```rust
pub fn strong_count(&self) -> usize {
    unsafe { *self.strong_count }
}
```

------

### Trait

#### Deref

因此可以使用`*`解引用符

```rust
impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}
```

#### Drop

实现`strong_count`减一

```rust
impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            *self.strong_count -= 1;
        }
    }
}
```

#### Clone

实现`strong_count`加一,并且返回与本身成员完全相同的`MyRc`变量

```rust
impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            *self.strong_count += 1;
        }
        Self { ..*self }
    }
}
```

------

## MyRc_without_pointer

- 比起使用裸指针的实现，对`strong_count`的修改不同步，即修改的不是一个值，测试代码有变。
- 利用`RefCell`实现，因为`Clone trait`对应的`fn clone(&self)->Self`的参数不可变，但要实现`strong_count`成员变量加一操作
- 对应的变量要求有`Clone trait`，因为`Clone trait`对应的`fn clone(&self)->Self`的参数是引用，返回所有权

