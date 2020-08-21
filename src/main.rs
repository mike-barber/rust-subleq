mod program;

use std::io::Read;

const MEM_SIZE: usize = 65535;

/*
// original C code
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

        // slice and constant indices is nice and fast
        let ipu = ip as usize;
        let sl = &mut code[ipu..ipu + 3];
        let a = sl[0];
        let b = sl[1];
        let c = sl[2];

        // traditional code
        if a == -1 {
            code[b as usize] = read_input();
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

fn subleq_match(code: &mut [i32]) {
    let mut ip = 0i32;
    let mut next_ip;

    while ip >= 0 {
        next_ip = ip + 3;

        // slice and constant indices is nice and fast
        let ipu = ip as usize;
        let sl = &mut code[ipu..ipu + 3];
        let a = sl[0];
        let b = sl[1];
        let c = sl[2];

        // match expression alternative
        match (a, b) {
            (-1, _) => code[b as usize] = read_input(),
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

fn load() -> [i32; MEM_SIZE] {
    let loaded: Vec<i32> = program::SUBLEQ_PROGRAM
        .split(" ")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    let mut dataset = [0i32; 65535];
    dataset[0..loaded.len()].copy_from_slice(&loaded);
    dataset
}

// probably a better way to do this; not very idiomatic
fn read_input() -> i32 {
    let mut buf = [0u8; 1];
    std::io::stdin().read_exact(&mut buf).unwrap();
    buf[0] as i32
}

fn test_time<F>(f: F, label: &str)
where
    F: Fn(&mut [i32]),
{
    use std::time::Instant;

    for i in 0..5 {
        println!();
        let mut dataset = load();
        let start = Instant::now();
        f(&mut dataset);
        let finish = Instant::now();
        println!(
            "{} iter {} time {:?}",
            label,
            i,
            finish.checked_duration_since(start).unwrap()
        );
    }
}

fn main() {
    test_time(subleq, "subleq traditional");
    test_time(subleq_match, "subleq match expression");
}
