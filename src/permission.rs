use std::collections::HashMap;

pub fn single_dac_to_octal(dac: String) -> i32 {
    let permission_table: HashMap<&str, i32> =
        HashMap::from([("r", 4), ("w", 2), ("x", 1), ("-", 0)]);

    let dac_permission: Vec<char> = dac.chars().collect::<Vec<char>>();
    let mut octal_permission: i32 = 0;

    for perm in dac_permission.iter() {
        octal_permission += permission_table.get(&perm.to_string() as &str).unwrap();
    }

    return octal_permission;
}

pub fn full_dac_to_octal(dac: String) -> Vec<i32> {
    let dac = dac.replace("d", "");
    let dac = dac.replace(".", "");

    let file_dac_permissions = [
        &dac[0..3].to_string(),
        &dac[3..6].to_string(),
        &dac[6..9].to_string(),
    ];

    let mut file_octal_permissions = vec![];

    for dac_permission in file_dac_permissions {
        file_octal_permissions.push(single_dac_to_octal(dac_permission.to_string()));
    }

    return file_octal_permissions;
}
