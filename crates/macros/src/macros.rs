
#[macro_export]
macro_rules! mul_add {
    ($a: expr, $b: expr, $c: expr) => {
        $a * ($b + $c)
    };
    ($a: expr, $b: expr, $c: expr, $d: expr) => {
        $a * ($b + $c) + $d
    };
}




// 一个反复捕获的例子
#[macro_export]
macro_rules! vec_strs {
    (
        // 开始反复捕获
        $(
            // 每个反复必须包含一个表达式
            $element:expr
        )
        // 由逗号分隔
        ,
        // 0 或多次
        *
    ) => {
        // 在这个块内用大括号括起来，然后在里面写多条语句
        {
            let mut v = Vec::new();
            // 开始反复捕获
            $(
                // 每个反复会展开成下面表达式，其中 $element 被换成相应被捕获的表达式
                v.push(format!("{}", $element));
            )*
            v
        }
    };
}

#[macro_export]
macro_rules! my_vec_strs {
    // 被反复捕获参数的地方
    ($($element:expr), *) => {
        {
            let mut v = Vec::new();

            // 去反复捕获的地方
            $(v.push(format!("{}", $element));)*
            v
        }
    };
}

#[macro_export]
macro_rules! repeat_two {
    ($($i1: ident)*, $($i2: ident)*) => {
        {
            let mut v1 = Vec::new();
            let mut v2 = Vec::new();
            $(
                v1.push(format!("{}", $i1));
                v2.push(format!("{}", $i2));
            )* 
            (v1, v2)
        }
    }
}


// (1..=num).fold(1, |acc, x| acc * x)
pub fn fibonacci() -> Box<dyn Iterator<Item = u64>>{
    use std::ops::Index;

    struct Recurrence {
        mem: [u64; 2],
        pos: usize,
    }

    struct IndexOffset<'a> {
        slice: &'a [u64; 2],
        offset: usize,
    }

    impl<'a> Index<usize> for IndexOffset<'a> {
        type Output = u64;

        #[inline]
        fn index<'b>(&'b self, index: usize) -> &'b Self::Output {
            use std::num::Wrapping;  // 防止算术运算时溢出

            let index = Wrapping(index);
            let offset = Wrapping(self.offset);
            let window = Wrapping(2);

            let real_index = index - offset + window;
            &self.slice[real_index.0]
        }
    }
    
    impl Iterator for Recurrence {
        type Item = u64;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            if self.pos < 2 {
                let next_val = self.mem[self.pos];
                self.pos += 1;
                Some(next_val)
            } else {
                let next_val = {
                    let n = self.pos;
                    let a = IndexOffset {slice: &self.mem, offset: n};
                    a[n - 1] + a[n - 2]
                };

                {
                    use std::mem::swap;
                    let mut swap_temp = next_val;
                    for i in (0..2).rev() {
                        swap(&mut swap_temp, &mut self.mem[i]);
                    }
                }
                self.pos += 1;
                Some(next_val)
            }
        }
    }
    Box::new(Recurrence {mem: [0, 1], pos: 0})
}

#[macro_export]
macro_rules! count_exprs {
    () => (0);
    ($e: expr) => (1);
    ($e1: expr, $($e2: expr), *) => (1 + count_exprs!($($e2), *));
    
}

#[macro_export]
// let fib = recurrence![a[n]: u64 = 0, 1; ...; a[n - 2] + a[n - 1]]
// (1..=num).fold(1, |acc, x| acc * x)
macro_rules! recurrence {
    ($seq:ident[$ind:ident]: $sty: ty = $($inits: expr), + ; ...; $recur: expr) => {
        {
            use std::ops::Index;

            // 需要根据MEM_SIZE个前置值，才能推导出整个递归序列
            const MEM_SIZE: usize = count_exprs!($($inits), +);

            struct Recurrence {
                mem: [$sty; MEM_SIZE],
                pos: usize,
            }

            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
                offset: usize,
            }

            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;

                #[inline]
                fn index<'b>(&'b self, index: usize) -> &'b Self::Output {
                    use std::num::Wrapping;  // 防止算术运算时溢出

                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);

                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }
            
            impl Iterator for Recurrence {
                type Item = $sty;

                #[inline]
                fn next(&mut self) -> Option<Self::Item> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let $ind = self.pos;
                            let $seq = IndexOffset {slice: &self.mem, offset: $ind};
                            $recur
                        };

                        {
                            use std::mem::swap;
                            let mut swap_temp = next_val;
                            for i in (0..MEM_SIZE).rev() {
                                swap(&mut swap_temp, &mut self.mem[i]);
                            }
                        }
                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }
            Recurrence {mem: [$($inits), +], pos: 0}
        }
    }
}

#[cfg(test)]
mod tests_macros {
    use super::fibonacci;

    #[test]
    fn test_mul_add() {
        assert_eq!(mul_add!(2, 3, 4), 14);
        assert_eq!(mul_add!(2.0, 3.0, 4.0), 14.0);
    }

    #[test]
    fn test_mul_add2() {
        assert_eq!(mul_add!(2, 3, 4, 5), 19);
    }

    #[test]
    fn test_vec_strs() {
        let s = vec_strs![1, "a", true, 3.14159f32];
        assert_eq!(s, &["1", "a", "true", "3.14159"]);
    }

    #[test]
    fn test_my_vec_strs() {
        let s = my_vec_strs![1, "a", true, 3.14159];
        assert_eq!(s, &["1", "a", "true", "3.14159"]);
    }

    #[test]
    fn test_repeat_two() {
        let a = 1 + 2;
        let b = 1 + 2;
        let c = 1 + 2;
        let d = 1 + 2;
        let e = 1 + 2;
        let f = 1 + 2;
        let u = 1 + 2;
        let v = 1 + 2;
        let w = 1 + 2;
        let x = 1 + 2;
        let y = 1 + 2;
        let z = 1 + 2;
        let (v1, v2) = repeat_two!(a b c d e f, u v w x y z);
        assert_eq!(v1, &["3", "3", "3", "3", "3", "3"]);
        assert_eq!(v2, &["3", "3", "3", "3", "3", "3"]);
    }

    #[test]
    fn test_fibonacci() {
        let mut fib = fibonacci();
        assert_eq!(fib.next(), Some(0));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        assert_eq!(fib.next(), Some(8));
        assert_eq!(fib.next(), Some(13));
        assert_eq!(fib.next(), Some(21));
        assert_eq!(fib.next(), Some(34));
    }

    #[test]
    fn test_fibonacci_macro() {
        let mut fib = recurrence![a[n]: u64 = 0, 1; ...; a[n - 1] + a[n - 2]];
        assert_eq!(fib.next(), Some(0));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        assert_eq!(fib.next(), Some(8));
        assert_eq!(fib.next(), Some(13));
        assert_eq!(fib.next(), Some(21));
        assert_eq!(fib.next(), Some(34));

        for e in recurrence!(f[i]: f64 = 1.0; ...; f[i - 1] * i as f64).take(10) {
            println!("{}", e);
        }
    }
}