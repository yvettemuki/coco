use regex::Regex;
use std::collections::{BTreeMap, HashSet};

lazy_static! {
    static ref JAVA_TEST: Regex = Regex::new(r".*(Tests|Test).java").unwrap();
    static ref JAVA_SOURCE_TEST: Regex = Regex::new(r".*.java").unwrap();
    static ref GROOVY_SOURCE_TEST: Regex = Regex::new(r".*.groovy").unwrap();
    static ref KOTLIN_SOURCE_TEST: Regex = Regex::new(r".*.kt").unwrap();
    static ref SCALA_SOURCE_TEST: Regex = Regex::new(r".*.scala").unwrap();
}

pub fn is_test(path: &str) -> bool {
    return JAVA_TEST.is_match(path);
}
pub fn is_java_source_file(path: &str) -> bool {
    return JAVA_SOURCE_TEST.is_match(path);
}
pub fn is_groovy_source_file(path: &str) -> bool {
    return GROOVY_SOURCE_TEST.is_match(path);
}
pub fn is_kotlin_source_file(path: &str) -> bool {
    return KOTLIN_SOURCE_TEST.is_match(path);
}
pub fn is_scala_source_file(path: &str) -> bool {
    return SCALA_SOURCE_TEST.is_match(path);
}

pub fn light_detect<'a>(file_names: &HashSet<String>) -> BTreeMap<&'a str, bool> {
    let mut tags = BTreeMap::new();
    detect_build_tool(file_names, &mut tags);
    detect_source_file(file_names, &mut tags);
    tags
}

fn detect_build_tool(names: &HashSet<String>, tags: &mut BTreeMap<&str, bool>) {
    tags.insert("workspace.gradle", names.contains("build.gradle"));
    tags.insert(
        "workspace.gradle.composite",
        names.contains("build.gradle") && names.contains("settings.gradle"),
    );
    tags.insert("workspace.pom", names.contains("pom.xml"));
}

fn detect_source_file(file_names: &HashSet<String>, tags: &mut BTreeMap<&str, bool>) {
    for file_name in file_names.iter() {
        if is_test(file_name) {
            tags.insert("workspace.source.test", true);
        }
        if is_java_source_file(file_name) {
            tags.insert("workspace.source.java", true);
        }
        if is_groovy_source_file(file_name) {
            tags.insert("workspace.source.groovy", true);
        }
        if is_kotlin_source_file(file_name) {
            tags.insert("workspace.source.kotlin", true);
        }
        if is_scala_source_file(file_name) {
            tags.insert("workspace.source.scala", true);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lang::jvm::{
        is_groovy_source_file, is_java_source_file, is_kotlin_source_file, is_scala_source_file,
        is_test,
    };

    #[test]
    fn should_ident_test_java_file() {
        assert_eq!(false, is_test("Hello.java"));
        assert_eq!(true, is_test("HelloWorldTest.java"));
        assert_eq!(true, is_test("HelloTests.java"));
    }

    #[test]
    fn should_ident_java_source_file() {
        assert_eq!(true, is_java_source_file("Hello.java"));
        assert_eq!(true, is_java_source_file("HelloWorldTest.java"));
    }

    #[test]
    fn should_ident_groovy_source_file() {
        assert_eq!(true, is_groovy_source_file("Hello.groovy"));
        assert_eq!(true, is_groovy_source_file("HelloTest.groovy"));
    }

    #[test]
    fn should_ident_kotlin_source_file() {
        assert_eq!(true, is_kotlin_source_file("Hello.kt"));
        assert_eq!(true, is_kotlin_source_file("HelloTest.kt"));
    }

    #[test]
    fn should_ident_scala_source_file() {
        assert_eq!(true, is_scala_source_file("Hello.scala"));
        assert_eq!(true, is_scala_source_file("HelloTest.scala"));
    }
}
