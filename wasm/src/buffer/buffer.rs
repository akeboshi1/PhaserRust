use std::cmp;
use super::mutref::get_mutable_ref;

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

struct RawBuf {
    bytes: Box<[u8]>,
}
pub struct Buffer {
    buf: RawBuf,
    reserved: usize,
    pos: usize,
}

impl Buffer {
    pub fn new(mut capsize:usize) -> Self {
        if capsize == 0 {
            capsize = DEFAULT_BUF_SIZE;
        }
        let mut vec_u8 = Vec::with_capacity(capsize);
        let true_cap = vec_u8.capacity();
            unsafe {
                vec_u8.set_len(true_cap);
            };
            let raw_buf = RawBuf {
                bytes: vec_u8.into_boxed_slice(),
            };
        Self { buf:raw_buf, 
               reserved:0,
               pos : 0,
            }
    }

    pub fn bytes(&self)->&[u8]{
        &self.buf.bytes
    }

    pub fn bytes_mut(&mut self)->&mut [u8]{
        &mut self.buf.bytes
    }

    pub fn operate_len(&self) -> usize{
        self.buf.bytes.len() - self.pos
    }

    pub fn len(&self) -> usize {
        self.buf.bytes.len() - self.reserved
    }

    pub fn consume(&mut self, amt: usize) {
        self.pos = cmp::min(self.pos + amt, self.buf.bytes.len());
    }

    fn reserve(&mut self, sz: usize) -> usize {
        if sz == 0 {
            return 0;
        }
        let mut old = self.reserved;

        let new = old + sz;
        if new >= self.buf.bytes.len() {
            return 0;
        }
        self.reserved = new;
        return new
    }

    pub fn alloc(&mut self, sz: usize) -> Option<& [u8]> {
        let new = self.reserve(sz);
        if new == 0 {
            None
        } else {
            let old = new - sz;
            Some(&self.buf.bytes[old..new])
        }
    }
    
    pub fn alloc_mut(&mut self, sz: usize) -> Option<& mut [u8]> {
        let new = self.reserve(sz);
        if new == 0 {
            None
        } else {
            let self_mut = unsafe { get_mutable_ref(self) };
            let old = new - sz;
            Some(&mut self_mut.buf.bytes[old..new])
        }
    }

    pub fn clear(&self) {
        if self.pos == 0 && self.reserved == 0 { return; }

    }
}
