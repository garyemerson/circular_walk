use std::io;

fn main() {
    let mut temp = get_line()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let (n, s, t) = (temp[0], temp[1], temp[2]);
    temp = get_line()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let (r0, g, seed, p) = (temp[0], temp[1], temp[2], temp[3]);
    let mut r: Vec<usize> = Vec::with_capacity(n);
    r.push(r0);
    for i in 1..n {
        let x = (r[i - 1] * g + seed) % p;
        r.push(x);
    }
    let result = match get_min_hops(&r, n, s, t) {
        Some(h) => { h.to_string() },
        None => { "-1".to_string() }
    };
    println!("{}", result);

    // println!("{}", get_neighbours(6, 100, 4).map(|i| i.to_string()).collect::<Vec<String>>().join(" "));
}

fn get_min_hops(r: &Vec<usize>, n: usize, s: usize, t: usize) -> Option<usize> {
    let mut visited = vec![false; r.len()];
    let mut dist: Vec<Option<usize>> = vec![None; r.len()];
    // let mut prev: Vec<Option<usize>> = vec![None; r.len()];
    dist[s] = Some(0);
    for nb in get_neighbours(s, n, r[s]) {
        if nb != s {
            dist[nb] = Some(1);
        }
    }

    while let Some(curr) = get_next_visit(n, &visited, &dist) {
        // println!("neighbours of {} are {:?}", curr, get_neighbours(curr, n, r[curr]).collect::<Vec<usize>>());
        for nb in get_neighbours(curr, n, r[curr]) {
            if dist[nb].is_none() || dist[curr].unwrap() + 1 < dist[nb].unwrap() {
                // println!("{} from {} is {} rather than {:?}", nb, curr, dist[curr].unwrap() + 1, dist[nb]);
                dist[nb] = Some(dist[curr].unwrap() + 1);
            }
        }
        visited[curr] = true;
    }
    dist[t]
}

fn get_next_visit(n: usize, visited: &Vec<bool>, dist: &Vec<Option<usize>>) -> Option<usize> {
    let mut min = None;
    for node in 0..n {
        if !visited[node] && 
           (dist[node].is_some() && (min.is_none() || dist[node].unwrap() < min.unwrap()))
        {
            min = Some(node)
        }
    }
    min
}

fn get_neighbours(node: usize, n: usize, reach: usize) -> Box<Iterator<Item=usize>> {
    if reach >= n / 2 { 
        return Box::new(0..n);
    }

    let start;
    if reach > node {
        start = n - (reach - node);
    } else {
        start = node - reach;
    }
    let end = (node + reach) % n;

    if start > end {
        Box::new((start..n).chain((0..(end + 1))))
    } else {
        Box::new((start..(end + 1)))
    }
}

fn get_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}

#[test]
fn back_and_forth() {
    let r = vec![4, 1, 1, 1, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    assert_eq!(get_min_hops(&r, 19, 1, 13), Some(3));
}

#[test]
fn no_path() {
    let r = vec![4, 0, 1, 1, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    assert_eq!(get_min_hops(&r, 19, 1, 13), None);
}

#[test]
fn no_path2() {
    let r = vec![4, 0, 1, 0, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    assert_eq!(get_min_hops(&r, 19, 2, 13), None);
}

#[test]
fn alread_there() {
    let r = vec![4, 0, 1, 0, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    assert_eq!(get_min_hops(&r, 19, 2, 2), Some(0));
}
