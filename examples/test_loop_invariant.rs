
use vstd::prelude::*;
verus!{
    fn vec(v: Vec<usize>, n: usize) {
        for i in 0..n
        invariant i <= n
        {
            require(i, n);
        }
    }
    
    fn require(i: usize, n: usize) 
    requires i < n
    {

    }

    struct Hole(usize);
    impl Hole {
        fn new(index: usize) -> (res: Self)
        ensures res.get_index_spec() == index
        {
            Hole(index)
        }
        fn get_index(&self) -> (res: usize)
        ensures res == self.get_index_spec() 
        {
            self.0
        }
        spec fn get_index_spec(&self) -> usize {
            self.0
        }
    }

    struct Wrapper { data: Vec<usize>}
    impl Wrapper {
        pub closed spec fn spec_len(&self) -> usize {
                self.data.len()
        }
        fn len(&self) -> (res: usize) 
        ensures res == self.spec_len()
        {
            self.data.len()
        }

        fn visit(&self) {
            if self.len() > 2 {
            let i = self.len() / 2;
            let h = Hole(i);
            // let len = self.len();
            assert(h.get_index_spec() < self.spec_len());
            }
        }

        fn looping(&self) {
            // if self.len() > 10 {
            // let len = self.len();
            for i in iter: 0..self.len()
                invariant 
                // i <= self.spec_len(),
                iter.end == self.spec_len()
            {

            }
            // }
        }
        fn end(&mut self, end: usize) 
        requires end <= old(self).spec_len()
        {
            assert(end <= self.spec_len());
        }

    fn rebuild_tail(&mut self, start: usize)
        requires start <= old(self).spec_len()
        {
        if start == self.len() {
            return;
        }

        let tail_len = self.len() - start;

        }        
        

    }

    fn array_loop(v: &[u8]) {
        let mut index = 0;
        let len = v.len();
        while index < len 
        invariant v.len() == len
        {
            // assert(index < len);
            let first = v[index];
            index += 1;
        }
    }
}
