use std::collections::VecDeque;

fn find_path(map: &mut Vec<Vec<i32>>, start: (usize, usize)) -> Option<Vec<Option<(usize, usize)>>> {
    let mut end = (0, 0);
    for (row, el) in map.iter().enumerate() {
        for (col, c) in el.iter().enumerate() {
            if *c == 69 {
                end = (row, col);
            }
        }
    }
    map[start.0][start.1] = 'a' as i32;
    map[end.0][end.1] = 'z' as i32;

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut via: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; map[0].len()]; map.len()];
    let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; map[0].len()]; map.len()];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];

    queue.push_back(start);
    dist[start.0][start.1] = 0;
    visited[start.0][start.1] = true;

    while queue.len() > 0 {
        let pos = queue.pop_front().unwrap();
        if pos == end {
            break;
        }

        if pos.0 < map.len() - 1 && map[pos.0 + 1][pos.1] - map[pos.0][pos.1] <= 1 && !visited[pos.0 + 1][pos.1] {
            visited[pos.0 + 1][pos.1] = true;
            dist[pos.0 + 1][pos.1] = 1 + dist[pos.0][pos.1];
            via[pos.0 + 1][pos.1] = Some(pos);
            queue.push_back((pos.0 + 1, pos.1));
        }

        if pos.0 > 0 && map[pos.0 - 1][pos.1] - map[pos.0][pos.1] <= 1 && !visited[pos.0 - 1][pos.1] {
            visited[pos.0 - 1][pos.1] = true;
            dist[pos.0 - 1][pos.1] = 1 + dist[pos.0][pos.1];
            via[pos.0 - 1][pos.1] = Some(pos);
            queue.push_back((pos.0 - 1, pos.1));
        }

        if pos.1 < map[0].len() - 1 && map[pos.0][pos.1 + 1] - map[pos.0][pos.1] <= 1 && !visited[pos.0][pos.1 + 1] {
            visited[pos.0][pos.1 + 1] = true;
            dist[pos.0][pos.1 + 1] = 1 + dist[pos.0][pos.1];
            via[pos.0][pos.1 + 1] = Some(pos);
            queue.push_back((pos.0, pos.1 + 1));
        }

        if pos.1 > 0 && map[pos.0][pos.1 - 1] - map[pos.0][pos.1] <= 1 && !visited[pos.0][pos.1 - 1] {
            visited[pos.0][pos.1 - 1] = true;
            dist[pos.0][pos.1 - 1] = 1 + dist[pos.0][pos.1];
            via[pos.0][pos.1 - 1] = Some(pos);
            queue.push_back((pos.0, pos.1 - 1));
        }
    }

    if !visited[end.0][end.1] {
        return None;
    }

    let mut pos = Some(end);
    let mut path = Vec::new();
    while pos != None {
        pos = via[pos.unwrap().0][pos.unwrap().1];
        path.push(pos);
    }
    path.reverse();
    Some(path[1..].to_vec())
}

fn part1(input: &str) -> usize {
    let mut map: Vec<Vec<i32>> = input.lines().map(|l| l.bytes().map(|x| x as i32).collect()).collect();
    let mut start = (0,0);
    for (row, el) in map.iter().enumerate() {
        for (col, c) in el.iter().enumerate() {
            if *c == 83 {
                start = (row, col);
            }
        }
    }
    find_path(&mut map, start).unwrap().len()
}

fn part2(input: &str) -> usize {
    let map: Vec<Vec<i32>> = input.replace("S", "a").lines().map(|l| l.bytes().map(|x| x as i32).collect()).collect();
    let mut dists = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 97 {
                let path = find_path(&mut map.to_vec(), (row, col));
                if let Some(p) = path {
                    dists.push(p.len());
                }
            }
        }
    }
    *dists.iter().min().unwrap()
}

fn main() {
    let input = include_str!("../input");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
