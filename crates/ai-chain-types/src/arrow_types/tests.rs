use crate::arrow_types::from_arrow::{map_schema_to_ai_chain};
use crate::arrow_types::to_arrow::AI_CHAIN_SCHEMA_KEY;
use crate::types::{FieldDefinition, FieldType, Schema as AIChainSchema, SourceDefinition};
use arrow::datatypes::{self as arrow_types};

use std::collections::HashMap;

#[test]
fn can_convert_from_arrow_to_AIChain() {
    let field_a = arrow_types::Field::new("a", arrow_types::DataType::Int64, false);
    let field_b = arrow_types::Field::new("b", arrow_types::DataType::Boolean, false);
    let field_c = arrow_types::Field::new("c", arrow_types::DataType::Utf8, false);

    let schema = arrow_types::Schema::new(vec![field_a.clone(), field_b, field_c]);
    let _aichain_schema = AIChainSchema::default()
        .field(
            FieldDefinition::new(
                "a".to_string(),
                FieldType::Int,
                false,
                SourceDefinition::Dynamic,
            ),
            true,
        )
        .field(
            FieldDefinition::new(
                "b".to_string(),
                FieldType::Boolean,
                false,
                SourceDefinition::Dynamic,
            ),
            false,
        )
        .field(
            FieldDefinition::new(
                "c".to_string(),
                FieldType::String,
                false,
                SourceDefinition::Dynamic,
            ),
            false,
        )
        .clone();

    let ai_chain_schema = map_schema_to_ai_chain(&schema).unwrap();
    assert_eq!(
        ai_chain_schema.fields[0].name,
        field_a.name().to_string().as_str()
    );
}

#[test]
fn can_convert_from_arrow_to_AIChain_with_metadata() {
    let field_a = arrow_types::Field::new("a", arrow_types::DataType::Int64, false);
    let field_b = arrow_types::Field::new("b", arrow_types::DataType::Boolean, false);
    let field_c = arrow_types::Field::new("c", arrow_types::DataType::Utf8, false);
    let field_d = arrow_types::Field::new("d", arrow_types::DataType::Utf8, false);

    let schema = arrow_types::Schema::new(vec![field_a.clone(), field_b, field_c, field_d]);
    let ai_chain_schema = AIChainSchema::default()
        .field(
            FieldDefinition::new(
                "a".to_string(),
                FieldType::Int,
                false,
                SourceDefinition::Dynamic,
            ),
            true,
        )
        .field(
            FieldDefinition::new(
                "b".to_string(),
                FieldType::Boolean,
                false,
                SourceDefinition::Dynamic,
            ),
            false,
        )
        .field(
            FieldDefinition::new(
                "c".to_string(),
                FieldType::String,
                false,
                SourceDefinition::Dynamic,
            ),
            false,
        )
        .field(
            FieldDefinition::new(
                "d".to_string(),
                FieldType::Json,
                false,
                SourceDefinition::Dynamic,
            ),
            false,
        )
        .clone();

    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert("row_count".to_string(), "100".to_string());
    metadata.insert(
        AI_CHAIN_SCHEMA_KEY.to_string(),
        serde_json::to_string(&ai_chain_schema).expect("Schema can always be serialized as JSON"),
    );

    let ai_chain_schema = map_schema_to_ai_chain(&schema.with_metadata(metadata)).unwrap();
    assert_eq!(
        ai_chain_schema.fields[0].name,
        field_a.name().to_string().as_str()
    );
}

#[test]
fn roundtrip_record_to_record_batch() {
    use super::super::arrow::record_batch::RecordBatch;
    use super::super::arrow_types::from_arrow::map_record_batch_to_ai_chain_records;
    use super::super::arrow_types::to_arrow::map_record_to_arrow;
    use super::super::types::Record;
    use crate::arrow_types::to_arrow::map_to_arrow_schema;
    use crate::types::field::{arrow_field_test_cases, arrow_field_test_cases_schema};

    let record: Record = Record::new(arrow_field_test_cases().collect());
    let record_batch: RecordBatch =
        map_record_to_arrow(record.clone(), &arrow_field_test_cases_schema()).unwrap();
    let res: Vec<Record> =
        map_record_batch_to_ai_chain_records(record_batch, &arrow_field_test_cases_schema()).unwrap();

    assert_eq!(vec![record], res);

    let arrow_schema = map_to_arrow_schema(&arrow_field_test_cases_schema()).unwrap();
    let original_schema = map_schema_to_ai_chain(&arrow_schema).unwrap();

    assert_eq!(original_schema, arrow_field_test_cases_schema());
}
