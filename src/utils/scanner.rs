pub struct Scanner<R> {
    reader: R,
    buffer: std::collections::VecDeque<String>,
}

impl<R: std::io::BufRead> Scanner<R> {
    const DEFAULT_BUF_SIZE : usize = 4096;

    pub fn new(reader : R) -> Self {
        Self {
            reader,
            buffer: std::collections::VecDeque::with_capacity(Self::DEFAULT_BUF_SIZE),
        }
    }

    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if !self.buffer.is_empty() {
                return self.buffer
                    .pop_front()
                    .unwrap()
                    .parse()
                    .ok()
                    .expect("Failed Parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input
                .split_whitespace()
                .map(String::from)
                .collect();
        }
    }

    pub fn read_vec<T: std::str::FromStr>(&mut self, n : usize) -> Vec<T> {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(self.read::<T>());
        }
        return v;
    }

    pub fn read_string(&mut self) -> String { self.read::<String>() }
    pub fn read_usize(&mut self) -> usize { self.read::<usize>() }
    pub fn read_i32(&mut self) -> i32 { self.read::<i32>() }
    pub fn read_i64(&mut self) -> i64 { self.read::<i64>() }
    pub fn read_f32(&mut self) -> f32 { self.read::<f32>() }
    pub fn read_f64(&mut self) -> f64 { self.read::<f64>() }

    pub fn read_vec_string(&mut self, n : usize) -> Vec<String> { self.read_vec::<String>(n) }
    pub fn read_vec_i32(&mut self, n : usize) -> Vec<i32> { self.read_vec::<i32>(n) }
    pub fn read_vec_i64(&mut self, n : usize) -> Vec<i64> { self.read_vec::<i64>(n) }

}
