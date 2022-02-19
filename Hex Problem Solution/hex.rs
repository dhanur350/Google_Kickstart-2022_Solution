pub mod dinitz {
    const INF: i64 = 1 << 60;
    pub struct Edge {
        pub to: usize,
        pub rev: usize,
        pub is_reversed: bool,
        pub cap: i64,
    }

    pub struct Dinitz {
        pub g: Vec<Vec<Edge>>,
    }

    impl Dinitz {
        pub fn new(v: usize) -> Dinitz {
            let mut g: Vec<Vec<Edge>> = Vec::new();
            for _ in 0..v {
                g.push(Vec::new());
            }
            Dinitz { g }
        }


        pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
            let to_len = self.g[to].len();
            let from_len = self.g[from].len();
            self.g[from].push(Edge {
                to,
                rev: to_len,
                is_reversed: false,
                cap,
            });
            self.g[to].push(Edge {
                to: from,
                rev: from_len,
                is_reversed: true,
                cap: 0,
            });
        }

        fn dfs(
            &mut self,
            v: usize,
            sink: usize,
            flow: i64,
            level: &[i32],
            iter: &mut [usize],
        ) -> i64 {
            if v == sink {
                return flow;
            }
            while iter[v] < self.g[v].len() {
                let flow = std::cmp::min(flow, self.g[v][iter[v]].cap);
                let to = self.g[v][iter[v]].to;
                if flow > 0 && level[v] < level[to] {
                    let flowed = self.dfs(to, sink, flow, level, iter);
                    if flowed > 0 {
                        let rev = self.g[v][iter[v]].rev;
                        self.g[v][iter[v]].cap -= flowed;
                        self.g[to][rev].cap += flowed;
                        return flowed;
                    }
                }
                iter[v] += 1;
            }
            0
        }

        fn bfs(&self, s: usize) -> Vec<i32> {
            let v = self.g.len();
            let mut level = vec![-1; v];
            level[s] = 0;
            let mut deque = std::collections::VecDeque::new();
            deque.push_back(s);
            while let Some(v) = deque.pop_front() {
                for e in self.g[v].iter() {
                    if e.cap > 0 && level[e.to] < 0 {
                        level[e.to] = level[v] + 1;
                        deque.push_back(e.to);
                    }
                }
            }
            level
        }

        pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
            let v = self.g.len();
            let mut flow: i64 = 0;
            loop {
                let level = self.bfs(s);
                if level[t] < 0 {
                    return flow;
                }
                let mut iter = vec![0; v];
                loop {
                    let f = self.dfs(s, t, INF, &level, &mut iter);
                    if f == 0 {
                        break;
                    }
                    flow += f;
                }
            }
        }
    }
}

fn in_hash(n: usize, i: usize, j: usize) -> usize {
    return i*n + j;
}

fn out_hash(n: usize, i: usize, j: usize) -> usize {
    return n*n + i*n + j;
}

fn add(dinitz: &mut dinitz::Dinitz, n: usize, i: usize, j: usize, i2: usize, j2: usize) {
    // helper to make it bidirectional with cap one
    dinitz.add_edge(out_hash(n, i, j), in_hash(n, i2, j2), 1);
    dinitz.add_edge(out_hash(n, i2, j2), in_hash(n, i, j), 1);
}

fn get_flow(n: usize, c: char, board: &Vec<Vec<char>>) -> i64 {
    let node_count = n*n*2;
    let source = node_count;
    let source_source = source+1;
    let sink = source_source + 1;
    let sink_sink = sink + 1;
    let mut dinitz = dinitz::Dinitz::new(sink_sink+1);
    for i in 0..n {
        for j in 0..n {
            if board[i][j] == c {
                dinitz.add_edge(in_hash(n, i, j), out_hash(n, i, j), 1);
            }
        }
    }
    for i in 0..n {
        for j in 0..n-1 {
            if board[i][j] == c && board[i][j+1] == c {
                add(&mut dinitz, n, i, j, i, j+1);
            }
            if board[j][i] == c && board[j+1][i] == c {
                add(&mut dinitz, n, j, i, j+1, i);
            }
            if i > 0 {
                if board[i][j] == c && board[i-1][j+1] == c {
                    add(&mut dinitz, n, i, j, i-1, j+1);
                }
            }
        }
    }
    if c == 'B' {
        for i in 0..n {
            if board[i][0] == 'B' {
                dinitz.add_edge(source, in_hash(n, i, 0), 1);
            }
            if board[i][n-1] == 'B' {
                dinitz.add_edge( out_hash(n, i, n-1), sink, 1);
            }
        }
    } else {
        for j in 0..n {
            if board[0][j] == 'R' {
                dinitz.add_edge(source, in_hash(n, 0, j), 1);
            }
            if board[n-1][j] == 'R' {
                dinitz.add_edge(out_hash(n, n-1, j), sink, 1);
            }
        }
    }
    dinitz.add_edge(source_source, source, 2);
    dinitz.add_edge(sink, sink_sink, 2);
    dinitz.max_flow(source_source, sink_sink)
}

fn solve(m: Vec<Vec<char>>, n: usize) -> &'static str {
    let mut b_count = 0i32;
    let mut r_count = 0i32;
    for i in 0..n {
        for j in 0..n {
            if m[i][j] == 'B' {
                b_count += 1;
            } else if m[i][j] == 'R' {
                r_count += 1;
            }
        }
    }
    if (b_count-r_count).abs() > 1 {
        return "Impossible";
    }
    let blue_flow = get_flow(n, 'B', &m);
    if blue_flow == 2 {
        return "Impossible";
    }
    let blue_wins = blue_flow == 1;
    let red_flow = get_flow(n, 'R', &m);
    if red_flow == 2 {
        return "Impossible";
    }
    let red_wins = red_flow == 1;

    if blue_wins && r_count > b_count {
        return "Impossible";
    }

    if red_wins && b_count > r_count {
        return "Impossible";
    }

    match (blue_wins, red_wins) {
        (true, true) => {
            "Impossible"
        },
        (true, false) => {
            "Blue wins"
        },
        (false, true) => {
            "Red wins"
        },
        _ => {
            "Nobody wins"
        }
    }
}

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let t: usize = sc.read();
    for case_num in 1..=t {
        let n: usize = sc.read();
        let m: Vec<Vec<char>> = (0..n).map(|_| sc.chars()).collect();
        sc.write(
            format!(
                "Case #{}: {}\n",
                case_num,
                solve(m, n)
            )
        );
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        self.1.write_all(s.to_string().as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn usize0(&mut self) -> usize {
        self.read::<usize>() - 1
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }

    pub fn binary_vec(&mut self) -> Vec<u8> {
        self.read::<String>()
            .bytes()
            .map(|b| b - b'0')
            .collect()
    }
}