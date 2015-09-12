#[cfg(test)]
use npuzzle::{NPuzzle};

#[cfg(test)]
fn check_one(s: &str, is_solvable: bool) {
	let np = NPuzzle::new_from_str(&s.to_string()).unwrap();
	// println!("{}", np.get_initial_state());
	// if np.is_solvable() == is_solvable {
	// 	println!("+++Ok is_solvable is {}\n\n", is_solvable);
	// } else {
	// 	println!("!!!Error is_solvable should be {}\n\n", is_solvable);
	// }
	assert!(np.is_solvable() == is_solvable);
}

#[test]
fn test_check_solvability_odd() {
	let str1 = r#"
# This puzzle is unsolvable
3
6 3 1
5 0 7
4 2 8
	"#;
	check_one(str1, false);
	let str1 = r#"
# This puzzle is solvable
3
1 5 0
3 6 8
4 7 2
	"#;
	check_one(str1, true);
	let str1 = r#"
# This puzzle is solvable
3
0 4 8
5 3 1
2 7 6
	"#;
	check_one(str1, true);
	let s = r#"
# This puzzle is unsolvable
3
0 1 3
4 6 2
7 8 5
 	"#;
	check_one(s, false);
	let str2 = r#"
# This puzzle is unsolvable
3
3 2 4
1 8 7
6 5 0
	"#;
	check_one(str2, false);
	let str3 = r#"
# This puzzle is solvable
3
3 5 4
7 2 6
8 1 0
	"#;
	check_one(str3, true);
	let str4 = r#"
# This puzzle is unsolvable
3
0 4 7
2 5 6
3 1 8
	"#;
	check_one(str4, false);
	let str5 = r#"
# This puzzle is solvable
3
0 4 2
6 8 3
7 5 1
	"#;
	check_one(str5, true);
	let str5 = r#"
# This puzzle is unsolvable
3
7 4 1
3 2 6
8 5 0
	"#;
	check_one(str5, false);
	let str6 = r#"
# This puzzle is unsolvable
7
32 43 48 12 19 39 34
16  4 42 23  7 14 33
 0 44 37 47  5 25 40
41 29  9 24 10  1  8
13  3 30 45 35  6 15
46 11 21 31 36 18  2
27 20 22 38 26 17 28
	"#;
	check_one(str6, false);
	let str7 = r#"
# This puzzle is solvable
7
15 28 48 43  8 35 30
 2 14 19 27 38 47  7
39  6  1 40 12  5 44
34 11 25 10 21 26 45
17 33 41 16 13  3 29
22 24 36 18 37  9 46
31 20  4 32  0 23 42
	"#;
	check_one(str7, true);
}

#[test]
fn test_check_solvability_even() {
	let str1 = r#"
# This puzzle is solvable
4
 1  7  6  8
 0 12  2 10
 4 13 15 14
 9 11  3  5
	"#;
	check_one(str1, true);
	let str1 = r#"
# This puzzle is solvable
4
 7  1  9 11
 3 13 12  2
 8  6  5  0
10 15 14  4
	"#;
	check_one(str1, true);
	let str1 = r#"
# This puzzle is unsolvable
4
 3  7  4 13
 1  2  0  6
15  5  9  8
11 12 10 14
	"#;
	check_one(str1, false);
	let s = r#"
# This puzzle is unsolvable
4
 1  4 15  2
11 13  8  7
14  3  5  6
 0 12 10  9
 	"#;
	check_one(s, false);
	let s = r#"
# This puzzle is solvable
6
19 25 23 11  4 33
21 10  1 13 17  6
 9  5 16 27 20  7
14 15 31 29 26 35
 2  8 32 18  3  0
22 12 34 24 30 28
 	"#;
	check_one(s, true);
	let s = r#"
# This puzzle is unsolvable
6
 3  4 30  0 18 19
 1 20 33 16 17 10
 7 22 35 11 25 26
 2 29  5  6 24 14
 8 12 34 23 13 27
31 32 28 15  9 21
 	"#;
	check_one(s, false);
}
