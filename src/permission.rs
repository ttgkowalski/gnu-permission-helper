use std::collections::HashMap;



pub fn dac_to_octal(dac: String) {

    let permission_table: HashMap<&str, i32> = HashMap::from([
        ("r", 4),
        ("w", 2),
        ("x", 1),
        ("-", 0),
    ]);

    let dac_permission: Vec<char> = dac.chars().collect::<Vec<char>>();
    let mut octal_permission: i32 = 0;

    for perm in dac_permission.iter() {
        octal_permission += permission_table.get(&perm.to_string() as &str).unwrap();
    }

    println!("{:?}", octal_permission);
}
