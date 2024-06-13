extern crate rand;

use rand::Rng;
use rand::distributions::Uniform;

/// 蒙特卡洛随机数生成算法。
/// 
/// 随机生成0-1之间的数，值越大生成的概率越大
pub fn montecarlo() -> f32 {
    let mut rng = rand::thread_rng();
    loop {
        let r1: f32 = rng.gen();
        let probability = r1;
        let r2: f32 = rng.gen();
        if r2 < probability {
            return r1;
        }
    }
}

/// 随机生成范围内的值(i32, f32, ...), 默认使用均匀分布
pub fn random_range_f32(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

/// 按照指定的分布生成随机数
/// 
/// 生成的随机数符合均匀分布的例子：
pub fn random_uniform_f32(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(min..max);
    rng.sample(die)
    // die.sample(&mut rng)  // 也可以这样写, use distributions::Distribution;
}

#[cfg(test)]
mod tests_random {
    use super::*;
    
    #[test]
    fn test_montecarlo() {
        let mut cap = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for _ in 0..100000 {
            let index = (montecarlo() * 10.0) as usize;
            cap[index] += 1;
         }
        println!("{:?}", cap);
        for i in 0..(cap.len() - 1) {
            assert!(cap[i + 1] > cap[i]);
        }
    }

    #[test]
    fn test_rand() {
        let mut rng = rand::thread_rng();
        println!("float: {}", rng.gen::<f32>());  // 随机生成一个浮点数【0, 1），默认使用均匀分布

        // rng.gen_range(0.0, 1.0);  // 早期版本的写法
        println!("float: {}", rng.gen_range(0.0..1.0));  // 随机生成一个浮点数【0, 1）
        println!("int: {}", rng.gen_range(0..10));  // 随机生成一个整数【0, 10）

        let die = Uniform::from(0..10);  // 指定均匀分布
        println!("int: {}", rng.sample(die));  // 随机生成一个整数【0, 10）
    }
}
