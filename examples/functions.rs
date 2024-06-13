

pub trait HKT<A, B> {
    type URI;
    type Target;
}

impl<A, B> HKT<A, B> for Option<A> {
    type URI = Self;
    type Target = Self;
}

pub trait Chain<A, B>: HKT<A, B> {
    fn chain<F>(self, f: F) -> <Self as HKT<A, B>>::Target
        where
            F: Fn(A) -> <Self as HKT<A, B>>::Target;
}

impl<A, B> Chain<A, B> for Option<A> {
    fn chain<F>(self, f: F) -> <Self as HKT<A, B>>::Target
            where
                F: Fn(A) -> <Self as HKT<A, B>>::Target {
        self.and_then(f)
    }
}

fn main() {
    
}


#[cfg(test)]
mod test_func {

    #[test]
    fn test1() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let filter = |predicate: fn(&i32) -> bool, xs: Vec<i32>| -> Vec<i32> {
            xs.into_iter().filter(predicate).collect::<Vec<i32>>()
        };
        let result = filter(|x| x % 2 == 0, v);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test2() {
        let add_to = |x: i32| move |y: i32| x + y;
        let add_2 = add_to(2);
        assert_eq!(add_2(3), 5);

        let sub_to = |x: i32| {
            move |y: i32| x - y
        };
        let sub_2 = sub_to(2);
        assert_eq!(sub_2(3), -1);
    }

    #[test]
    fn test3() {
        fn add(x: i32) -> impl Fn(i32) -> i32 {
            move |y| x + y
        }
        let add5 = add(5);
        assert_eq!(add5(2), 7);
    }

    #[test]
    fn test4() {
        let greet = || "hello";
        assert_eq!(greet(), "hello");
    }

    #[test]
    fn test5() {
        let mut x = 5;
        {
            let mut add_to_x = |y| x += y;
            add_to_x(3);
        }
        assert_eq!(x, 8);
    }

    #[test]
    fn test6() {
        let x = vec![1, 4, 3];
        let sort = |y: &Vec<i32>| {
            let mut y = y.clone();
            y.sort();
            y
        };
        let sorted = sort(&x);
        assert_eq!(sorted, vec![1, 3, 4]);
    }

    #[test]
    fn test7() {
        macro_rules! compose {
            ($last: expr) => {$last};
            ($head: expr, $($tail: expr),+) => {
                // |x| compose!($($tail),+)(($head)(x))
                compose_two($head, compose!($($tail),+))
            };
        }

        fn compose_two<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
            where
                F: Fn(A) -> B,
                G: Fn(B) -> C,
        {
            move |x| g(f(x))
        }

        let add1 = |x| x + 1;
        let mul2 = |x| x * 2;
        let add_then_mul = compose!(add1, mul2);
        assert_eq!(add_then_mul(2), 6);
    }
}