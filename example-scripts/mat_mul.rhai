const SIZE = 50;

fn new_mat(x, y) {
	let row = [];
	row.pad(y, 0.0);
	
	let matrix = [];
	matrix.pad(x, row);
	
	matrix
}

fn mat_gen(n) {
    let m = new_mat(n, n);
    let tmp = 1.0 / n.to_float() / n.to_float();

    for i in range(0, n) {
        for j in range(0, n) {
            m[i][j] = tmp * (i.to_float() - j.to_float()) * (i.to_float() + j.to_float());
        }
    }
	
    m
}

fn mat_mul(a, b) {
    let m = a.len;
    let n = a[0].len;
    let p = b[0].len;

    let b2 = new_mat(n, p);
	
    for i in range(0, n) {
        for j in range(0, p) {
            b2[j][i] = b[i][j];
        }
    }

    let c = new_mat(m, p);

	for i in range(0, c.len) {
		let ci = c[i];
		for j in range(0, ci.len) {
			let b2j = b2[j];
			ci[j] = 0.0;
			
			for z in range(0, a[i].len) {
				let x = a[i][z];
				let y = b2j[z];
				ci[j] += x * y;
			}
        }
		c[i] = ci;
    }

    c
}

let now = timestamp();

let a = mat_gen(SIZE);
let b = mat_gen(SIZE);
let c = mat_mul(a, b);

/*
for i in range(0, SIZE) {
	print(c[i]);
}
*/

print(`Finished. Run time = ${now.elapsed} seconds.`);
