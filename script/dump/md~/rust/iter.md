源码
====

``` {.rust}
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// 可以看到调用的next方法使用的是 &mut self.为什么，因为对于 iter来说，其改变了内部的索引

```

再看iter的源码:

``` {.rust}
pub fn iter(&self) -> Iter<'_, T> {
    unsafe {
        let ptr = self.as_ptr();
        assume(!ptr.is_null());

        let end = if mem::size_of::<T>() == 0 {
            (ptr as *const u8).wrapping_add(self.len()) as *const T
        } else {
            ptr.add(self.len())
        };

        Iter {
            ptr,
            end,
            _marker: marker::PhantomData
        }
    }
}
```

可以看到，调用x.iter()方法时，其中使用的是引用，所以当使用
`for i in x.iter()` 时，其不会消耗x的所有权，只是借了一个引用，对应的
`i` 的类型也会自然的为引用类型。

其实也可以理解for也是模式匹配，如果使用 `for &i in x.iter()`
也是可以的，但是此时i就不是引用类型了。

所以通过以上信息可以了解到 `x.iter()`
返回的数据是不可变引用，如果想要获取可变引用,则需要使用 `x.iter_mut()`
那么此时迭代的数据就是可变的引用。而想要获取数据的所有权，则需要使用
`x.into_iter` 那么之后 `x` 将不可用,也即 `x.into_iter()` 消耗了对应的
`x` 的所有权。

类比 `self/&self/&mut self` ,iter对应的是
`x.into_iter()/x.iter()/x.iter_mut()`

生成迭代器
==========

通常使用 `x.iter()` 便会生成迭代器，当生成迭代器时
不会消耗资源，因为其是惰性(lazy)的，也可以通过迭代器适配器来生成其他类型的迭代器，如
`x.iter().map(|x|y)`
这种迭代器也不会消耗资源，同样的他也是惰性的，只有在调用消耗方法时，才会消耗对应的资源。
类似的迭代器适配器有: `filter=、=take_while` `zip` 等等等等

消费迭代器
==========

默认使用 `x.iter()`
时，不会耗费什么资源，因为其是惰性(lazy)的,而当你使用消费方法时，才会起作用，且会消费对应的迭代器。如
`sum` 就是消费方法， `x.iter().sum()` ，注意其消费的是迭代器，而不是 `x`
变量。如:

``` {.rust}
let v1 = vec![1, 2, 3];
let v2 = v1.iter();
let s: i32 = v2.sum();
println!("{:?}", v2); // v2已经被消费，所以此处编译不通过
```

类似的消费方法有 `collect` 、 `for_each` 等等

# 常用迭代器方法

## cloned
将迭代器每个元素的引用clone出对应的非引用值，等价于 `map(|&x| x)`
## filter
过滤，需要注意的是闭包中为true是不被过滤，为false的被过滤:
```rust
let a = [1, 4, 2, 3];
a.iter().cloned().filter(|x| x % 2 == 0) // 过滤出偶数
```
## map
将T类型的元素转变为U类型
如:
```rust
let a = [1, 4, 2, 3];
a.iter().map(|num| num.to_string())
```
## cloned
将迭代器每个元素的引用clone出对应的非引用值，等价于 `map(|&x| x)`
## filter_map
filter和map的结合体，传入的函数需要返回`Option<T>`,若返回值是`Some(T)`,则不被过滤，若返回值是`None`则被过滤。

如: 
```rust
let mut iter = a.iter().filter_map(|s| s.parse().ok());

assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(3));
assert_eq!(iter.next(), Some(5));
assert_eq!(iter.next(), None);
// 等价于
let mut iter = a.iter().map(|s| s.parse()).filter(|s| s.is_ok()).map(|s| s.unwrap());
```

## inspect
debug使用，用来打印链式操作的中间值。
如:
```rust
let sum = a.iter()
	.cloned() 
	.inspect(|x| println!("about to filter: {}", x))
	.filter(|x| x % 2 == 0)
	.inspect(|x| println!("made it through filter: {}", x))
	.fold(0, |sum, i| sum + i);
```

自定义迭代器
============

自定义迭代器只需要实现Interator的trait，如:

``` {.rust}
fn main() {
    let i = MyIter { count: 0 };
    // i就是Iterator,所以可以使用很多方法，如skip,map等
    let v: Vec<i32> = i.skip(2).collect();
    println!("{:?}", v);
}

struct MyIter {
    count: i32,
}

impl Iterator for MyIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```
