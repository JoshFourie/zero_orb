pub mod into_field;
pub mod wrapped_groth;

// unit tests for IntoField.
#[test]
fn test_collect_nums() {
    use crate::transform::into_field::IntoField;
    use zksnark::groth16::fr::FrLocal;
    use serde_json::to_string;

    // expected results are that x_ == y_. and that on a vec len == 0 the fn returns a None.
    let x_none: Vec<usize> = Vec::new();
    assert!(to_string(&x_none.collect_nums::<FrLocal>()).unwrap().contains("null")); 

    let x_8: Vec<usize> = vec![10, 13]; 
    let y_8: Vec<FrLocal> = vec![
        FrLocal::from(10), FrLocal::from(13)
    ];
    match x_8.collect_nums::<FrLocal>() {
        Some(val) => {
            match val == y_8 {
                true => {},
                false => panic!("IntoField::x_8.collect_nums(): left != right"),
            }
        },
        None => panic!("IntoField::x_8.collect_nums() returned a None value"),
    }

    let x_16: Vec<usize> = vec![100, 120]; 
    let y_16: Vec<FrLocal> = vec![
        FrLocal::from(100), FrLocal::from(120)
    ];
    match x_16.collect_nums::<FrLocal>() {
        Some(val) => {
            match val == y_16 {
                true => {},
                false => panic!("IntoField::x_16.collect_nums(): left != right"),
            }
        },
        None => panic!("IntoField::x_16.collect_nums() returned a None value"),
    }

    let x_32: Vec<usize> = vec![1301, 1190]; 
    let y_32: Vec<FrLocal> = vec![
        FrLocal::from(1301), FrLocal::from(1190)
    ];
    match x_32.collect_nums::<FrLocal>() {
        Some(val) => {
            match val == y_32 {
                true => {},
                false => panic!("IntoField::x_32.collect_nums(): left != right"),
            }
        },
        None => panic!("IntoField::x_32.collect_nums() returned a None value"),
    }
}

#[test]
fn test_collect_bits() {
    use crate::transform::into_field::IntoField;
    use zksnark::groth16::fr::FrLocal;
    use serde_json::to_string;

    // expected results are that x_ == y_. and that on a vec len == 0 the fn returns a None.
    let x_none: Vec<usize> = Vec::new();
    assert!(to_string(&x_none.collect_bits::<FrLocal>(&"".to_string())).unwrap().contains("null")); 
    
    let x_8: Vec<usize> = vec![15];
    let y_8: Vec<FrLocal> = vec![
        FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0)
    ];
    
    assert_eq!(y_8.len(), 8);
    match x_8.collect_bits::<FrLocal>(&"u8".to_string()) {
        Some(val) => {
            match val == y_8 {
                true => {},
                false => panic!("IntoField::x_8.collect_bits: left != right"),
            };
        },
        None => panic!("IntoField::x_8.collect_bits() returned a None value")
    }

    let x_16: Vec<usize> = vec![1001];
    let y_16: Vec<FrLocal> = vec![
        FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(1), FrLocal::from(0), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0),
    ];
    assert_eq!(y_16.len(), 16);
    match x_16.collect_bits::<FrLocal>(&"u16".to_string()) {
        Some(val) => {
            match val == y_16 {
                true => {},
                false => panic!("IntoField::x_16.collect_bits: left != right"),
            };
        },
        None => panic!("IntoField:: x_16.collect_bits() returned a None value")
    }

    let x_32: Vec<usize> = vec![30]; 
    let y_32: Vec<FrLocal> = vec![
        FrLocal::from(0), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0),
    ];
    assert_eq!(y_32.len(), 32);
    match x_32.collect_bits::<FrLocal>(&"u32".to_string()) {
        Some(val) => {
            match val == y_32 {
                true => {},
                false => panic!("IntoField::x_32.collect_bits: left != right"),
            };
        },
        None => panic!("IntoField:: x_32.collect_bits() returned a None value")
    }

    let x_64: Vec<usize> = vec![32];
    let y_64: Vec<FrLocal> = vec![
        FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0),
    ];
    assert_eq!(y_64.len(), 64);
    match x_64.collect_bits::<FrLocal>(&"u64".to_string()) {
        Some(val) => {
            match val == y_64 {
                true => {},
                false => panic!("IntoField::x_64.collect_bits: left != right"),
            };
        },
        None => panic!("IntoField:: x_64.collect_bits() returned a None value")
    }
}