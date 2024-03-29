pub mod generality {
    use std::ops::Add;

    fn add<T: Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    pub fn run_add() {
        let x = add(7.9, 5.9);
        println!("the x is {}", x);
    }

    #[derive(PartialEq, Debug)]
    pub struct Foo;
    #[derive(PartialEq, Debug)]
    pub struct Bar;
    #[derive(PartialEq, Debug)]
    pub struct FooBar;
    #[derive(PartialEq, Debug)]
    pub struct BarFoo;

    // 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
    impl std::ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            println!("{:?}", self);
            FooBar
        }
    }
    // 下面的代码实现了自定义类型的相减： Bar - Foo = BarFoo
    impl std::ops::Sub<Foo> for Bar {
        type Output = BarFoo;

        fn sub(self, _rhs: Foo) -> BarFoo {
            println!("{:?}", self);
            BarFoo
        }
    }

    pub fn run_struct_add_sub() {
        println!("{:?}", Foo + Bar);
        println!("{:?}", Bar - Foo);
        println!("{:?}", FooBar);
        println!("{:?}", BarFoo);
    }

    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    pub fn run_mixup() {
        let p1 = Point { x: 145, y: 5 };
        let p2 = Point { x: "hello", y: 'u' };
        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
    // 1
    // fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    //     println!("{:?}", arr);
    // }
    // 2
    fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }

    pub fn run_display_array() {
        let arr = [1, 2, 3];
        display_array(arr);

        let arr = [4, 5];
        display_array(arr);
    }
    // 泛型
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
}

pub mod feature {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    pub struct Post {
        pub title: String,   // 标题
        pub author: String,  // 作者
        pub content: String, // 内容
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }

    #[derive(Debug)]
    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }

    // 返回 Summary 特征 由不同struct 实现 返回不同实体
    pub fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
        if switch {
            Box::new(Post {
                title: "test".to_string(),
                author: "test".to_string(),
                content: "test".to_string(),
            })
        } else {
            Box::new(Weibo {
                username: "auth".to_string(),
                content: "test".to_string(),
            })
        }
    }
}
