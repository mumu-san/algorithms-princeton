use std::iter::FromIterator;
pub trait Ufmain {
    fn connected(&mut self, a: usize, b: usize) -> bool;
    fn union(&mut self, p: usize, q: usize);
}

//QuickFindUf
pub struct QuickFindUf {
    id: Box<[usize]>,
}
impl QuickFindUf {
    pub fn new(n: usize) -> QuickFindUf {
        QuickFindUf {
            id: Vec::from_iter(0..n).into_boxed_slice(),
        }
    }
}
impl Ufmain for QuickFindUf {
    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.id[a] == self.id[b]
    }
    fn union(&mut self, p: usize, q: usize) {
        let pid = self.id[p];
        let qid = self.id[q];

        for i in &mut self.id.iter_mut() {
            if *i == pid {
                *i = qid;
            }
        }
    }
}
//

//QuickUnionUf
pub struct QuickUnionUf {
    id: Box<[usize]>,
}
impl QuickUnionUf {
    pub fn new(n: usize) -> QuickUnionUf {
        QuickUnionUf {
            id: Vec::from_iter(0..n).into_boxed_slice(),
        }
    }
    fn root(&self, mut e: usize) -> usize {
        while e != self.id[e] {
            e = self.id[e];
        }
        e
    }
}
impl Ufmain for QuickUnionUf {
    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }
    fn union(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);
        self.id[i] = j;
    }
}
//

//Weighed_QU
pub struct WeighedQu {
    id: Box<[usize]>,
    sz: Box<[usize]>,
}
impl WeighedQu {
    pub fn new(n: usize) -> WeighedQu {
        WeighedQu {
            id: Vec::from_iter(0..n).into_boxed_slice(),
            sz: vec![1; n].into_boxed_slice(),
        }
    }
    fn root(&self, mut e: usize) -> usize {
        while e != self.id[e] {
            e = self.id[e];
        }
        e
    }
}
impl Ufmain for WeighedQu {
    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }
    fn union(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);
        if i == j {
            return;
        }
        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }
    }
}
//

//PathCompress_QU
pub struct PathCompressQu {
    id: Box<[usize]>,
}
impl PathCompressQu {
    pub fn new(n: usize) -> PathCompressQu {
        PathCompressQu {
            id: Vec::from_iter(0..n).into_boxed_slice(),
        }
    }
    fn root(&mut self, mut e: usize) -> usize {
        while e != self.id[e] {
            self.id[e] = self.id[self.id[e]];
            e = self.id[e];
        }
        e
    }
}
impl Ufmain for PathCompressQu {
    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }
    fn union(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);
        self.id[i] = j;
    }
}
//

//WQUPC
pub struct WQUPC {
    id: Box<[usize]>,
    sz: Box<[usize]>,
}
impl WQUPC {
    pub fn new(n: usize) -> WQUPC {
        WQUPC {
            id: Vec::from_iter(0..n).into_boxed_slice(),
            sz: vec![1; n].into_boxed_slice(),
        }
    }
    fn root(&mut self, mut e: usize) -> usize {
        while e != self.id[e] {
            self.id[e] = self.id[self.id[e]];
            e = self.id[e];
        }
        e
    }
}
impl Ufmain for WQUPC {
    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }
    fn union(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);
        if i == j {
            return;
        }
        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }
    }
}
//
