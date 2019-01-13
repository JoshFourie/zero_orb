pub mod into_field;
pub mod wrapped_groth;

// unit tests for IntoField.
#[test]
fn test_collect_nums() {
    use crate::transform::into_field::IntoField;
    use zksnark::groth16::fr::FrLocal;

    let x_8: Vec<usize> = vec![10, 13]; 
    let y_8 = vec![
        FrLocal::from(10), FrLocal::from(13)
    ];
    assert!(x_8.collect_nums::<FrLocal>() == y_8);

    let x_16: Vec<usize> = vec![100, 120]; 
    let y_16 = vec![
        FrLocal::from(100), FrLocal::from(120)
    ];
    assert!(x_16.collect_nums::<FrLocal>() == y_16);


    let x_32: Vec<usize> = vec![1301, 1190]; 
    let y_32 = vec![
        FrLocal::from(1301), FrLocal::from(1190)
    ];
    assert!(x_32.collect_nums::<FrLocal>() == y_32);
}

#[test]
fn test_collect_bits() {
    use crate::transform::into_field::IntoField;
    use zksnark::groth16::fr::FrLocal;
    
    let x_8: Vec<usize> = vec![15];
    let y_8 = vec![
        FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0)
    ];
    
    assert_eq!(y_8.len(), 8);
    assert!(x_8.collect_bits::<FrLocal>(&"u8".to_string()) == y_8);

    let x_16: Vec<usize> = vec![1001];
    let y_16 = vec![
        FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(1), FrLocal::from(0), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0),
    ];
    assert_eq!(y_16.len(), 16);
    assert!(x_16.collect_bits::<FrLocal>(&"u16".to_string()) == y_16);

    let x_32: Vec<usize> = vec![30]; 
    let y_32 = vec![
        FrLocal::from(0), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0),
    ];
    assert_eq!(y_32.len(), 32);
    assert!(x_32.collect_bits::<FrLocal>(&"u32".to_string()) == y_32);

    let x_64: Vec<usize> = vec![32];
    let y_64 = vec![
        FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0),
    ];
    assert_eq!(y_64.len(), 64);
    assert!(x_64.collect_bits::<FrLocal>(&"u64".to_string()) == y_64);
}