#[derive(Clone)]
struct Edge<T: Clone> {
    to: usize,
    cap: T,
    rev: usize,
}

pub struct Graph<T: Clone> {
    edges: Vec<Vec<Edge<T>>>,
}

impl<T: Clone> Graph<T> {
    pub fn new(n: usize) -> Self {
        Graph {
            edges: vec![Vec::new(); n],
        }
    }
}

impl Graph<i64> {
    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let flen = self.edges[from].len();
        let tlen = self.edges[to].len();
        self.edges[from].push(Edge { to, cap, rev: tlen });
        self.edges[to].push(Edge {
            to: from,
            cap: 0,
            rev: flen,
        });
    }

    fn dfs(&mut self, v: usize, t: usize, f: i64, used: &mut Vec<bool>) -> i64 {
        if v == t {
            return f;
        }
        used[v] = true;
        for i in 0..self.edges[v].len() {
            let eto = self.edges[v][i].to;
            let ecap = self.edges[v][i].cap;
            let erev = self.edges[v][i].rev;
            if !used[eto] && ecap > 0 {
                let d = self.dfs(eto, t, std::cmp::min(f, ecap), used);
                if d > 0 {
                    self.edges[v][i].cap -= d;
                    self.edges[eto][erev].cap += d;
                    return d;
                }
            }
        }
        0
    }

    pub fn max_flow(&mut self, s: usize, t: usize, inf: i64) -> i64 {
        let mut flow = 0;
        let mut used = vec![false; self.edges.len()];
        loop {
            used.iter_mut().for_each(|x| *x = false);
            let f = self.dfs(s, t, inf, &mut used);
            if f == 0 {
                return flow;
            }
            flow += f;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_match_test() {
        let mut graph = Graph::new(8);

        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 1);
        graph.add_edge(0, 3, 1);

        graph.add_edge(4, 7, 1);
        graph.add_edge(5, 7, 1);
        graph.add_edge(6, 7, 1);

        graph.add_edge(1, 4, 1);
        graph.add_edge(1, 5, 1);
        graph.add_edge(2, 4, 1);
        graph.add_edge(3, 5, 1);

        assert_eq!(2, graph.max_flow(0, 7, 100));
    }
}
