use std::marker::PhantomData;
use std::{ptr, ptr::NonNull, mem};
use std::alloc::{self, Layout};
use std::ops::{Deref, DerefMut};

struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}
unsafe impl<T: Send> Send for RawVec<T> {}
unsafe impl<T: Sync> Sync for RawVec<T> {}

impl<T> RawVec<T> {
    fn new() -> Self {
        // assert!(mem::size_of::<T>() != 0, "we are not ready to handle ZSTs");
        // 在编译期间就能够确定cap的结果
        let cap = if mem::size_of::<T>() == 0 { usize::MAX } else { 0 };

        RawVec { ptr: NonNull::dangling(), cap: cap, }
    }

    fn grow(&mut self) {
        // 初始化时T的尺寸为0，设置cap为usize::MAX, 溢出
        assert!(mem::size_of::<T>() != 0, "capacity overflow");

        let (new_cap, new_layout) = if self.cap == 0  { 
            (1, Layout::array::<T>(1).unwrap()) 
        } else { 
            let new_cap = 2 * self.cap;
            // `Layout::array`会检查申请的空间是否小于等于usize::MAX, 但是因为old_layout.size() <= isize::MAX,
            // 所以unwrap永远不可能失败
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // 保证新申请的内存没有超过`isize::MAX`字节大小
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe {alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };
        // 如果分配失败，`new_ptr`就会成为空指针，需要对应的abort操作
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();
        if self.cap != 0 && elem_size != 0 {
            unsafe {
                alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8, 
                    Layout::array::<T>(self.cap).unwrap(),
                );
            }
        }
    }
}

pub struct Vec<T> {
    buf: RawVec<T>,
    len: usize,  // 数据长度
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec { 
            buf: RawVec::new(), 
            len: 0,
        }
    }
    
    pub fn cap(&self) -> usize{
        self.buf.cap
    }

    pub fn len(&self) -> usize{
        self.len
    }

    pub fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buf.grow();
        }
        unsafe {
            // 拒绝使用索引，防止在旧值上drop
            ptr::write(self.ptr().add(self.len), elem);
        }
        // 不可能出错，出错之前一定会OOM
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                // 使用`ptr::read`防止原始位置的内存被drop
                Some(ptr::read(self.ptr().add(self.len)))
            }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bound");
        // 容量不够，需要扩容
        if self.cap() == self.len { self.buf.grow(); }

        unsafe {
            // ptr::copy(src, dst, count), 从src复制连续的count个元素到dst
            // 会正确处理源和目标重叠的情况
            ptr::copy(
                self.ptr().add(index), 
                self.ptr().add(index + 1), 
                self.len - index
            );
            ptr::write(self.ptr().add(index), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        self.len -= 1;
        unsafe {
            let result = ptr::read(self.ptr().add(index));
            ptr::copy(
                self.ptr().add(index + 1), 
                self.ptr().add(index), 
                self.len - index
            );
            result
        }
    }

}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.cap() != 0 {
            while let Some(_) = self.pop() {}
        }
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr(), self.len)
        }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr(), self.len)
        }
    }
}

struct RawValIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> RawValIter<T> {
    // 构建RawValIter是不安全的，因为RawValIter没有关联的生命周期。
    // 将RawValIter存储在与他实际分配相同的结构体上中是十分必要的，
    // 细节不向外公开
    unsafe fn new(slice: &[T]) -> Self {
        RawValIter {
            start: slice.as_ptr(),
            end: if mem::size_of::<T>() == 0 {
                ((slice.as_ptr() as usize) + slice.len()) as *const _
            } else if slice.len() == 0 {
                // 长度为0避免使用offset
                slice.as_ptr()
            } else {
                slice.as_ptr().add(slice.len())
            }
        }
    }
}

impl<T> Iterator for RawValIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    self.start = (self.start as usize + 1) as *const _;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    let old_ptr = self.start;
                    self.start = self.start.offset(1);
                    Some(ptr::read(old_ptr))
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<T>();
        let len = (self.end as usize - self.start as usize) 
        / if elem_size == 0 { 1 } else { elem_size };
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for RawValIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    self.end = (self.end as usize - 1) as *const _;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    self.end = self.end.offset(-1);
                    Some(ptr::read(self.end))
                }
            }
        }
    }
}
pub struct IntoIter<T> {
    _buf: RawVec<T>,
    iter: RawValIter<T>,
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            // RawVec没有实现Copy
            let iter = RawValIter::new(&self);
            let buf = ptr::read(&self.buf);
            mem::forget(self);
            Self::IntoIter {
                _buf: buf,
                iter: iter,
            }
        }
    }
}

impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // 读取之后会被自动清理
        for _ in &mut *self {}
    }
}

pub struct Drain<'a, T: 'a> {
    vec: PhantomData<&'a mut Vec<T>>,
    iter: RawValIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> { self.iter.next_back() }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

impl<T> Vec<T> {
    pub fn drain(&mut self) -> Drain<T> {
        unsafe {
            let iter = RawValIter::new(&self);

            // 这里事关 mem::forget 的安全。
            // 如果 Drain 被 forget，我们就会泄露整个 Vec 的内存，
            // 既然我们始终要做这一步，为何不在这里完成呢？
            self.len = 0;

            Drain {
                iter: iter,
                vec: PhantomData,
            }
        }
    }
}


#[cfg(test)]
mod test_vec {
    use super::*;

    #[test]
    fn test_vec() {
        let mut vec = Vec::new();
        assert_eq!(vec.len(), 0);
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.len(), 2);
        vec.push(-1);
        let mut sum = 0;
        for val in vec.iter() {
            sum += val;
        }
        vec.insert(2, 3);
        vec.remove(2);

        assert_eq!(sum, 2);
        vec.pop();
        assert_eq!(vec.len(), 2);
    }
}