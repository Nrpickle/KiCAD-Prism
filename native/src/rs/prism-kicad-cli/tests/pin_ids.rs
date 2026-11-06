//! Pins without saved UUIDs keep their renderer IDs and still form a valid bundle.

mod support;

use prism_bundle::{BundleReader, Limits, Work};
use serde_json::{Value, json};
use support::{run, s, scratch};

const SYMBOL: &str = "00000000-0000-4000-8000-0000000000aa";
const SAVED_PIN: &str = "00000000-0000-4000-8000-0000000000b2";
const SHEET: &str = r#"(kicad_sch (version 20250114) (generator "eeschema")
  (uuid "00000000-0000-4000-8000-000000000001") (paper "A4")
  (lib_symbols (symbol "Demo:Part"
    (symbol "Part_1_1"
      (pin passive line (at 0 0 0) (length 2.54) (name "A") (number "A/B 2"))
      (pin passive line (at 0 -2.54 0) (length 2.54) (name "B") (number "2")))))
  (symbol (lib_id "Demo:Part") (at 10 10 0) (unit 1)
    (uuid "00000000-0000-4000-8000-0000000000aa")
    (property "Reference" "U1" (at 10 10 0))
    (property "Value" "Part" (at 10 10 0))
    (pin "2" (uuid "00000000-0000-4000-8000-0000000000b2"))
    (instances (project "p"
      (path "/00000000-0000-4000-8000-000000000001" (reference "U1") (unit 1)))))
  (sheet_instances (path "/" (page "1"))))
"#;

#[test]
fn missing_pin_uuids_preserve_renderer_identity_and_verify() {
    let tree = scratch("pin-ids");
    std::fs::write(tree.join("p.kicad_pro"), "{}\n").unwrap();
    std::fs::write(tree.join("p.kicad_sch"), SHEET).unwrap();
    let out = tree.join("o.prism");
    let built = run(&[
        "bundle",
        "build",
        s(&tree.join("p.kicad_pro")),
        "--stage",
        "parse",
        "-o",
        s(&out),
        "--quiet",
    ]);
    assert_eq!(built.code, 0, "{}", built.document);
    let verified = run(&["bundle", "verify", s(&out), "--quiet"]);
    assert_eq!(verified.code, 0, "{}", verified.document);
    let reader = BundleReader::open(
        std::fs::read(&out).unwrap(),
        &Limits::defaults(Work::Verify),
    )
    .unwrap();
    let index: Value = serde_json::from_slice(
        &reader
            .member("projections/semantic-index.json")
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(index["schema"], "prism.semantic_index.a3");
    for (pin, expected) in [
        ("A/B 2", format!("{SYMBOL}__pin__A_B_2")),
        ("2", SAVED_PIN.to_owned()),
    ] {
        let terminal = index["terminals"]
            .as_array()
            .unwrap()
            .iter()
            .find(|terminal| terminal["reference"] == "U1" && terminal["pin"] == pin)
            .unwrap();
        assert_eq!(terminal["schematicPinUuid"], expected);
        assert!(
            index["indexes"]["terminalBySchematicPinUuid"]
                .get(&expected)
                .is_some()
        );
        assert!(index["nets"].as_array().unwrap().iter().any(|net| {
            net["schematicRefs"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["pinUuids"].as_array().unwrap().contains(&json!(expected)))
        }));
    }
}

#[test]
fn pin_identity_validation_is_narrow_and_legacy_schemas_still_validate() {
    let original: Value = serde_json::from_str(include_str!(
        "../../../../contracts/prism/examples/SemanticIndexA3.example.json"
    ))
    .unwrap();
    for pin_id in [SAVED_PIN.to_owned(), format!("{SYMBOL}__pin__A_B.2:-")] {
        let mut index = original.clone();
        index["nets"][0]["schematicRefs"][0]["pinUuids"] = json!([pin_id]);
        index["terminals"][0]["schematicPinUuid"] = json!(pin_id);
        prism_bundle::schema::validate("prism.semantic_index.a3", &index).unwrap();
    }
    for pin_id in [
        "".to_owned(),
        "not-a-uuid__pin__1".to_owned(),
        format!("{SYMBOL}__pin__"),
        format!("{SYMBOL}__pin__A/B"),
    ] {
        let mut index = original.clone();
        index["nets"][0]["schematicRefs"][0]["pinUuids"] = json!([pin_id]);
        assert!(prism_bundle::schema::validate("prism.semantic_index.a3", &index).is_err());
        let mut index = original.clone();
        index["terminals"][0]["schematicPinUuid"] = json!(pin_id);
        assert!(prism_bundle::schema::validate("prism.semantic_index.a3", &index).is_err());
    }
    let mut index = original;
    index["nets"][0]["schematicRefs"][0]["wireUuids"] = json!([format!("{SYMBOL}__pin__1")]);
    assert!(prism_bundle::schema::validate("prism.semantic_index.a3", &index).is_err());
    let legacy: Value = serde_json::from_str(include_str!(
        "../../../../contracts/prism/examples/SemanticIndexA2.example.json"
    ))
    .unwrap();
    prism_bundle::schema::validate("prism.semantic_index.a2", &legacy).unwrap();
}
