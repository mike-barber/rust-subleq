mod program;

const MEM_SIZE: usize = 65535;

/*
void subleq(int* code) {
    int ip = 0, a, b, c, nextIP;
    char ch;

    while (0 <= ip) {
        nextIP = ip + 3;
        a = code[ip];
        b = code[ip + 1];
        c = code[ip + 2];

        if (a == -1) {
            if (scanf("%c", &ch) != EOF)
                code[b] = (int)ch;
        }
        else if (b == -1) {
            printf("%c", (char)code[a]);
        }
        else {
            code[b] -= code[a];
            if (code[b] <= 0)
                nextIP = c;
        }
        ip = nextIP;
    }
}
*/

fn subleq(code: &mut [i32]) {
    let mut ip = 0i32;
    let mut next_ip;

    while ip >= 0 {
        next_ip = ip + 3;

        let a = code[ip as usize];
        let b = code[(ip + 1) as usize];
        let c = code[(ip + 2) as usize];

        if a == -1 {
            println!("Error: input not supported");
        } else if b == -1 {
            print!("{}", code[a as usize] as u8 as char);
        } else {
            code[b as usize] -= code[a as usize];
            if code[b as usize] <= 0 {
                next_ip = c;
            }
        }
        ip = next_ip;
    }
}

fn subleq_opt(code: &mut [i32]) {
    let mut ip = 0i32;
    let mut next_ip;

    while ip >= 0 {
        next_ip = ip + 3;

        let ipu = ip as usize;
        let sl = &mut code[ipu..ipu + 3];
        let a = sl[0];
        let b = sl[1];
        let c = sl[2];

        if a == -1 {
            println!("Error: input not supported");
        } else if b == -1 {
            print!("{}", code[a as usize] as u8 as char);
        } else {
            code[b as usize] -= code[a as usize];
            if code[b as usize] <= 0 {
                next_ip = c;
            }
        }

        ip = next_ip;
    }
}

fn subleq_opt2(code: &mut [i32]) {
    let mut ip = 0i32;
    let mut next_ip;

    while ip >= 0 {
        next_ip = ip + 3;

        let ipu = ip as usize;
        let sl = &mut code[ipu..ipu + 3];
        let a = sl[0];
        let b = sl[1];
        let c = sl[2];

        match (a, b) {
            (-1, _) => println!("Error: input not supported"),
            (_, -1) => print!("{}", code[a as usize] as u8 as char),
            _ => {
                code[b as usize] -= code[a as usize];
                if code[b as usize] <= 0 {
                    next_ip = c;
                }
            }
        };
        ip = next_ip;
    }
}

fn subleq_opt3(code: &mut [i32]) {
    let mut ip = 0i32;
    let mut next_ip;

    while ip >= 0 {
        next_ip = ip + 3;

        let ipu = ip as usize;
        let sl = &mut code[ipu..ipu + 3];
        let a = sl[0];
        let b = sl[1];
        let c = sl[2];

        match (a, b) {
            (-1, _) => println!("Error: input not supported"),
            (_, -1) => print!("{}", code[a as usize] as u8 as char),
            _ => {
                let mut cb = code[b as usize];
                cb -= code[a as usize];
                code[b as usize] = cb;
                if cb <= 0 {
                    next_ip = c;
                }
            }
        };
        ip = next_ip;
    }
}

fn subleq_opt4(code: &mut [i32]) {
    let mut ip = 0i32;
    let mut next_ip;

    while ip >= 0 {
        next_ip = ip + 3;

        let ipu = ip as usize;
        let sl = &mut code[ipu..ipu + 3];
        let a = sl[0];
        let b = sl[1];
        let c = sl[2];

        match (a, b) {
            (-1, _) => println!("Error: input not supported"),
            (_, -1) => print!("{}", code[a as usize] as u8 as char),
            _ => {
                // copy ca, then use a mutable reference to cb
                // (this isn't as fast as the _opt3 approach)
                let ca = code[a as usize];
                let cb = &mut code[b as usize];
                *cb -= ca;
                if *cb <= 0 {
                    next_ip = c;
                }
            }
        };
        ip = next_ip;
    }
}

// also no faster; the slice approach works fine!
fn subleq_opt5(code: &mut [i32]) {
    use std::mem::transmute_copy;

    struct ABC {
        a: i32,
        b: i32,
        c: i32,
    }

    let mut ip = 0i32;
    let mut next_ip;

    while ip >= 0 {
        next_ip = ip + 3;

        let ipu = ip as usize;
        let v: ABC = unsafe { transmute_copy(&code[ipu]) };

        match (v.a, v.b) {
            (-1, _) => println!("Error: input not supported"),
            (_, -1) => print!("{}", code[v.a as usize] as u8 as char),
            _ => {
                let mut cb = code[v.b as usize];
                cb -= code[v.a as usize];
                code[v.b as usize] = cb;
                if cb <= 0 {
                    next_ip = v.c;
                }
            }
        };
        ip = next_ip;
    }
}

fn load() -> [i32; MEM_SIZE] {
    let loaded: Vec<i32> = program::SUBLEQ_PROGRAM
        .split(" ")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    let mut dataset = [0i32; 65535];
    dataset[0..loaded.len()].copy_from_slice(&loaded);
    dataset
}

fn test_time<F>(f: F, label: &str)
where
    F: Fn(&mut [i32]),
{
    use std::time::Instant;

    let mut dataset = load();
    let start = Instant::now();
    f(&mut dataset);
    let finish = Instant::now();
    println!(
        "{} time {:?}",
        label,
        finish.checked_duration_since(start).unwrap()
    );
}

fn main() {
    test_time(subleq, "normal");
    test_time(subleq_opt, "opt");
    test_time(subleq_opt2, "opt2");
    test_time(subleq_opt3, "opt3");
    test_time(subleq_opt4, "opt4");
    test_time(subleq_opt5, "opt5");
}
