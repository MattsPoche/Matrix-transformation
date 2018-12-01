fn rot_matrix(v: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut v_p = Vec::new();
    let (x_ubound, y_ubound) = get_bounds(v);

    for &(x, y) in v.iter() {
        let x_p = y;
        let y_p = x_ubound - x;

        v_p.push((x_p, y_p));
    }

    v_p
}

fn get_bounds(v: &Vec<(usize, usize)>) -> (usize, usize) {
    let mut x_ubound = 0;
    let mut y_ubound = 0;

    for &(x, y) in v.iter() {
        if x > x_ubound {
            x_ubound = x;
        }

        if y > y_ubound {
            y_ubound = y;
        }
    }

    (x_ubound, y_ubound)
}

fn tuplize(m: &Vec<Vec<u16>>) -> Vec<(usize, usize)> {
    let mut tup_list = Vec::new();
    for y in 0..m.len() {
        for x in 0..m[y].len() {
            if m[y][x] == 1 {
                tup_list.push((x, y));
            }
        }
    }
    tup_list
}

fn matricize(v: &Vec<(usize, usize)>) -> Vec<Vec<u16>> {
    let (x_ubound, y_ubound) = get_bounds(&v);
    let mut m = vec![vec![0; x_ubound+1]; y_ubound+1];
    for &(x, y) in v.iter() {
        m[y][x] = 1;
    }

    m
}

fn print_vec(v: &Vec<(usize, usize)>) {
    //todo
    let (x_ubound, y_ubound) = get_bounds(&v);
    for y in 0..y_ubound+1 {
        for x in 0..x_ubound+1 {
            if v.contains(&(x, y)) {
                print!("1");
            } else {
                print!("0");
            }
        }
        print!("\n");
    }
}

fn print_matrix(m: &Vec<Vec<u16>>) {
    for y in 0..m.len() {
        for x in 0..m[y].len() {
            print!("{}", m[y][x]);
        }
        print!("\n");
    }
}

fn main() {
    let m = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
    ];

    let m = tuplize(&m);

    println!("{:?}", m);

    let m_p = rot_matrix(&m);

    let m_p_p = rot_matrix(&m_p);

    let m_p_p_p = rot_matrix(&m_p_p);

    print_vec(&m);
    print!("\n");
    print_vec(&m_p);
    print!("\n");
    print_vec(&m_p_p);
    print!("\n");
    print_vec(&m_p_p_p);
}

#[test]
fn test_ubounds() {
    let v = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
    ];

    let v = tuplize(&v);

    let (x_b, y_b) = get_bounds(&v);

    assert_eq!(3, x_b);
    assert_eq!(5, y_b);
}

#[test]
fn test_matricize() {
    let v = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
    ];

    let v = tuplize(&v);

    let m = matricize(&v);

    print_matrix(&m);
}