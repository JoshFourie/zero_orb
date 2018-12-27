use std::fs::File;
use std::io::{LineWriter, Write};
use std::path::Path;

pub fn build_comparator(quantity: usize, path: &Path) {
    let file = File::create(path).unwrap();
    let mut file = LineWriter::new(file);
    let quantity = quantity;
    initialise_zk(
        quantity,
        insert_diff(
            quantity,
            insert_acc(
                quantity,
                insert_fdiff(
                    quantity,
                    insert_acc(
                        quantity,
                        insert_res(
                            quantity,
                            &mut file
                        )
                    )
                )
            )
        )
    );
    file.write_all(b")");
}

fn insert_res(quantity: usize, file: &mut LineWriter<File>) -> &mut LineWriter<File> {
    for num in 0..quantity {
        if num == 1 {
            for val in (1..32).rev() {
                if val == 32 {
                    write!(
                        file,
                        "   (= resB_{} (* 1 chkaB_{}))\n",
                        val, val
                    );
                } else {
                    write!(
                        file,
                        "   (= resB_i{} (* resB_{} chkaB_{})) (= resB_{} (* 1 (+ resB_{} chkaB_{} (* 250 resB_i{}))))\n",
                        val, val + 1, val, val, val + 1, val, val
                    );
                }
            }
        } if num == 2 {
            for val in (1..32).rev() {
                if val == 32 {
                    write!(
                        file,
                        "   (= resC_{} (* 1 chkaC_{}))\n",
                        val, val
                    );
                } else {
                    write!(
                        file,
                        "   (= resC_i{} (* resC_{} chkaC_{})) (= resC_{} (* 1 (+ resC_{} chkaC_{} (* 250 resC_i{}))))\n",
                        val, val + 1, val, val, val + 1, val, val
                    );
                }
            }
        }
    }
    file
}

fn insert_chka(quantity: usize, file: &mut LineWriter<File>) -> &mut LineWriter<File> {
    for num in 0..quantity {
        if num == 1 {
            for val in 0..32 {
                write!(
                    file,
                    "   (= chkaB_{} (* aB_{} fdiffB_{}))\n",
                    val, val, val
                );
            }
        } if num == 2 {
            for val in 0..32 {
                write!(
                    file,
                    "   (= chkaC_{} (* aC_{} fdiffC_{}))\n",
                    val, val, val
                );
            }
        }
    }
    file
}

fn insert_fdiff(quantity: usize, file: &mut LineWriter<File>) -> &mut LineWriter<File> {
    for num in 0..quantity {
        if num == 1 {
            for val in (0..32).rev() {
                if val == 32 {
                    write!(
                        file,
                        "   (= fdiffB_{} (* 1 accB_{}))\n",
                        val, val
                    );
                } else {
                    write!(
                        file,
                        "   (= fdiffB_{} (* (+ accB_{} (* 250 accB_{})) (+ accB_{} (* 250 accB_{}))))\n",
                        val, val + 1, val, val + 1, val
                    );
                }
            }
        } if num == 2 {
            for val in (0..32).rev() {
                if val == 32 {
                    write!(
                        file,
                        "   (= fdiffC_{} (* 1 accC_{}))\n",
                        val, val
                    );
                } else {
                    write!(
                        file,
                        "   (= fdiffC_{} (* (+ accC_{} (* 250 accC_{})) (+ accC_{} (* 250 accC_{}))))\n",
                        val, val + 1, val, val + 1, val
                    );
                }
            }
        } 
    }
    file
}

fn insert_acc(quantity: usize, file: &mut LineWriter<File>) -> &mut LineWriter<File> {
    for num in 0..quantity {
        if num == 1 {
            for val in (0..32).rev() {
                if val == 32 {
                    write!(
                        file,
                        "   (= accB_{} (* 1 diffB_{}))\n",
                        val, val
                    );
                } else {
                    write!(
                        file,
                        "   (= accB_i{} (* accB_{} diffB_{})) (= accB_{} (* 1 (+ accB_{} diffB_{} (* 250 accB_i{}))))\n",
                        val, val + 1, val, val, val + 1, val, val
                    );
                }
            }
        } if num == 2 {
            for val in (0..32).rev() {
                if val == 32 {
                    write!(
                        file,
                        "   (= accC_{} (* 1 diffC_{}))\n",
                        val, val
                    );
                } else {
                    write!(
                        file,
                        "   (= accC_i{} (* accC_{} diffC_{})) (= accC_{} (* 1 (+ accC_{} diffC_{} (* 250 accC_i{}))))\n",
                        val, val + 1, val, val, val + 1, val, val
                    );
                }
            }
        }
    }
    file   
}

fn insert_diff(quantity: usize, file: &mut LineWriter<File>) -> &mut LineWriter<File>{
    
    //  (= diffB_0 (* (+ aB_0 (* 250 bB_0)) (+ aB_0 (* 250 bB_0))))
    for num in 0..quantity {
        if num == 1 {
            for val in 0..32 {
                write!(
                    file, 
                    "   (= diffB_{} (* (+ aB_{} (* 250 bB_{})) (+ aB_{} (* 250 bB_{}))))\n", 
                    val, val, val, val, val
                );
            }
        } if num == 2 {
            for val in 0..32 {
                write!(
                    file, 
                    "   (= diffC_{} (* (+ aC_{} (* 250 bC_{})) (+ aC_{} (* 250 bC_{}))))\n", 
                    val, val, val, val, val
                );
            }
        }
    }
    file
}
fn initialise_zk(quantity: usize,  file: &mut LineWriter<File>) -> &mut LineWriter<File>{    
    file.write_all(b"(");
        file.write_all(b"in");
            for num in 0..quantity {
                if num == 0 {
                    for a in 0..32 {
                        write!(file, " a{}", a);
                    }
                } if num == 1 {
                    for b in 0..32 {
                        write!(file, " b{}", b);
                    }
                } if num == 2 {
                    for c in 0..32 {
                        write!(file, " c{}", c);
                    }
                }
            }
    file.write_all(b")\n");

    file.write_all(b"(");
        file.write_all(b"out");
        for num in 0..quantity {
            if num == 1 {
                write!(file, " res_b");
            } if num == 2 {
                write!(file, " res_c");
            }
        }
    file.write_all(b")\n");


    file.write_all(b"(");
        file.write_all(b"verify");
        for num in 0..quantity {
            if num == 1 {
                write!(file, " res_b");
                for b in 0..32 {
                    write!(file, " b{}", b);
                }
            } if num == 2 {
                write!(file, " res_c");
                for c in 0..32 {
                    write!(file, " c{}", c);
                }
            }
        }
    file.write_all(b")\n");

    file.write_all(b"(program\n");
    file
}