pub struct MemberInfo {
    pub name: String,
    pub access: String,
    pub data_type: String,
}

pub struct MethodInfo {
    pub name: String,
    pub access: String,
    pub return_type: String,
}

pub struct ClassInfo {
    pub name: String,
    pub id: i32,
    pub parents: Vec<String>,
    pub members: Vec<MemberInfo>,
    pub method: Vec<MethodInfo>,
}

impl ClassInfo {
    pub fn new(class_name: &str) -> Self {
        ClassInfo {
            name: class_name.to_string(),
            id: 0,
            parents: vec![],
            members: vec![],
            method: vec![],
        }
    }
}
