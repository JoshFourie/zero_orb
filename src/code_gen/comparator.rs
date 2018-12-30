use std::fs::File;
use std::io::{LineWriter, Write};
use std::path::Path;

pub fn new(tag: &'static str, path: &Path) {
    let file = LineWriter::new(
        File::create(path).unwrap()
    );
    match tag {
        "32 RANGE" => build(3, 32, file),
        "16 RANGE" => build(3, 16, file),
        "8 RANGE" => build(3, 8, file),
        "32 COMP" => build(2, 32, file),
        "16 COMP" => build(2, 16, file),
        "8 COMP" => build(2, 8, file),
        &_ => panic!("bad tag"),
    }
}

fn build(quantity: usize, bits: usize, mut file: LineWriter<File>) {

    file.write_all(b"(").unwrap();
        file.write_all(b"in").unwrap();
            for num in 0..quantity {
                if num == 0 {
                    for a in 0..bits {
                        write!(file, " a{}", a).unwrap();
                    }
                } if num == 1 {
                    for b in 0..bits {
                        write!(file, " b{}", b).unwrap();
                    }
                } if num == 2 {
                    for c in 0..bits {
                        write!(file, " c{}", c).unwrap();
                    }
                }
            }
    file.write_all(b")\n").unwrap();

    file.write_all(b"(").unwrap();
        file.write_all(b"out").unwrap();
        for num in 0..quantity {
            if num == 1 {
                write!(file, " resB_1").unwrap();
            } if num == 2 {
                write!(file, " resC_1").unwrap();
            }
        }
    file.write_all(b")\n").unwrap();


    file.write_all(b"(").unwrap();
        file.write_all(b"verify").unwrap();
        for num in 0..quantity {
            if num == 1 {
                write!(file, " resB_1").unwrap();
                for b in 0..bits {
                    write!(file, " b{}", b).unwrap();
                }
            } if num == 2 {
                write!(file, " resC_1").unwrap();
                for c in 0..bits {
                    write!(file, " c{}", c).unwrap();
                }
            }
        }
    file.write_all(b")\n").unwrap();

    file.write_all(b"(program\n").unwrap();
   
    for num in 0..quantity {
        if num == 1 {
            for val in 0..bits {
                write!(
                    file, 
                    "   (= diffB_{} (* (+ a{} (* 250 b{})) (+ a{} (* 250 b{}))))\n", 
                    val, val, val, val, val
                ).unwrap();
            }
        } if num == 2 {
            for val in 0..bits {
                write!(
                    file, 
                    "   (= diffC_{} (* (+ a{} (* 250 c{})) (+ a{} (* 250 c{}))))\n", 
                    val, val, val, val, val
                ).unwrap();
            }
        }
    }

    for num in 0..quantity {
        if num == 1 {
            for val in (0..bits).rev() {
                if val == bits - 1 {
                    write!(
                        file,
                        "   (= accB_{} (* 1 diffB_{}))\n",
                        val, val  
                    ).unwrap();
                } else {
                    write!(
                        file,
                        "   (= accB_i{} (* accB_{} diffB_{})) (= accB_{} (* 1 (+ accB_{} diffB_{} (* 250 accB_i{}))))\n",
                        val, val + 1, val, val, val + 1, val, val
                    ).unwrap();
                }
            }
        } if num == 2 {
            for val in (0..bits).rev() {
                if val == bits - 1 {
                    write!(
                        file,
                        "   (= accC_{} (* 1 diffC_{}))\n",
                        val, val  
                    ).unwrap();
                } else {
                    write!(
                        file,
                        "   (= accC_i{} (* accC_{} diffC_{})) (= accC_{} (* 1 (+ accC_{} diffC_{} (* 250 accC_i{}))))\n",
                        val, val + 1, val, val, val + 1, val, val
                    ).unwrap();
                }
            }
        }
    }

    for num in 0..quantity {
        if num == 1 {
            for val in (0..bits).rev() {
                if val == bits - 1 {
                    write!(
                        file,
                        "   (= fdiffB_{} (* 1 accB_{}))\n",
                        val, val
                    ).unwrap();
                } else {
                    write!(
                        file,
                        "   (= fdiffB_{} (* (+ accB_{} (* 250 accB_{})) (+ accB_{} (* 250 accB_{}))))\n",
                        val, val + 1, val, val + 1, val
                    ).unwrap();
                }
            }
        } if num == 2 {
            for val in (0..bits).rev() {
                if val == bits - 1 {
                    write!(
                        file,
                        "   (= fdiffC_{} (* 1 accC_{}))\n",
                        val, val
                    ).unwrap();
                } else {
                    write!(
                        file,
                        "   (= fdiffC_{} (* (+ accC_{} (* 250 accC_{})) (+ accC_{} (* 250 accC_{}))))\n",
                        val, val + 1, val, val + 1, val
                    ).unwrap();
                }
            }
        } 
    }

    for num in 0..quantity {
        if num == 1 {
            for val in 0..bits {
                write!(
                    file,
                    "   (= chkaB_{} (* b{} fdiffB_{}))\n",
                    val, val, val
                ).unwrap();
            }
        } if num == 2 {
            for val in 0..bits {
                write!(
                    file,
                    "   (= chkaC_{} (* c{} fdiffC_{}))\n",
                    val, val, val
                ).unwrap();
            }
        }
    }
    for num in 0..quantity {
        if num == 1 {
            for val in (1..bits).rev() {
                if val == bits - 1 {
                    write!(
                        file,
                        "   (= resB_{} (* 1 chkaB_{}))\n",
                        val, val
                    ).unwrap();
                } else {
                    write!(
                        file,
                        "   (= resB_i{} (* resB_{} chkaB_{}))\n   (= resB_{} (* 1 (+ resB_{} chkaB_{} (* 250 resB_i{}))))\n",
                        val, val + 1, val, val, val + 1, val, val
                    ).unwrap();
                }
            }
        } if num == 2 {
            for val in (1..bits).rev() {
                if val == bits - 1 {
                    write!(
                        file,
                        "   (= resC_{} (* 1 chkaC_{}))\n",
                        val, val
                    ).unwrap();
                } else {
                    write!(
                        file,
                        "   (= resC_i{} (* resC_{} chkaC_{}))\n   (= resC_{} (* 1 (+ resC_{} chkaC_{} (* 250 resC_i{}))))\n",
                        val, val + 1, val, val, val + 1, val, val
                    ).unwrap();
                }
            }
        }
    }
}
