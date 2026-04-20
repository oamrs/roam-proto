pub mod v1 {
    pub mod agent {
        tonic::include_proto!("v1.agent");
    }

    pub mod query {
        tonic::include_proto!("v1.query");
    }

    pub mod schema {
        tonic::include_proto!("v1.schema");
    }
}

#[cfg(test)]
mod tests {
    use super::v1::agent::Event;
    use super::v1::schema::{
        ColumnDef, FieldMappingDef, GetSchemaResponse, GetTableResponse, IndexDef, TableDef,
        TriggerDef, UserDefinedTypeDef,
    };

    #[test]
    fn schema_response_has_tables_field() {
        let response = GetSchemaResponse {
            schema_id: "test".to_string(),
            database_type: "sqlite".to_string(),
            generated_at: "2024-01-01".to_string(),
            tables: vec![],
            user_defined_types: vec![],
        };
        assert_eq!(response.tables.len(), 0);
    }

    #[test]
    fn table_def_has_columns_and_indexes() {
        let table = TableDef {
            name: "users".to_string(),
            columns: vec![],
            indexes: vec![],
            triggers: vec![],
            field_mappings: vec![],
        };
        assert_eq!(table.name, "users");
    }

    #[test]
    fn column_def_has_expected_fields() {
        let col = ColumnDef {
            name: "id".to_string(),
            data_type: "INTEGER".to_string(),
            is_nullable: false,
            default_value: String::new(),
            is_primary_key: true,
        };
        assert!(col.is_primary_key);
    }

    #[test]
    fn index_def_has_expected_fields() {
        let idx = IndexDef {
            name: "idx_users_email".to_string(),
            columns: vec!["email".to_string()],
            is_unique: true,
        };
        assert!(idx.is_unique);
    }

    #[test]
    fn trigger_def_has_expected_fields() {
        let trig = TriggerDef {
            name: "trg_users_after_insert".to_string(),
            timing: "AFTER".to_string(),
            event: "INSERT".to_string(),
            body: String::new(),
        };
        assert_eq!(trig.timing, "AFTER");
    }

    #[test]
    fn user_defined_type_def_has_expected_fields() {
        let udt = UserDefinedTypeDef {
            name: "Status".to_string(),
            base_type: "TEXT".to_string(),
            variants: vec!["active".to_string(), "inactive".to_string()],
        };
        assert_eq!(udt.variants.len(), 2);
    }

    #[test]
    fn field_mapping_def_has_expected_fields() {
        let fm = FieldMappingDef {
            column_name: "userId".to_string(),
            logical_name: "user_id".to_string(),
            convention: "Hibernate".to_string(),
        };
        assert_eq!(fm.convention, "Hibernate");
    }

    #[test]
    fn get_table_response_has_table_field() {
        let resp = GetTableResponse {
            generated_at: "2024-01-01".to_string(),
            table: Some(TableDef {
                name: "orders".to_string(),
                columns: vec![],
                indexes: vec![],
                triggers: vec![],
                field_mappings: vec![],
            }),
        };
        assert!(resp.table.is_some());
    }

    #[test]
    fn event_has_typed_payload_oneof() {
        // Event must have a typed_payload oneof field, not just opaque bytes.
        // We verify via the schema_changed variant.
        use super::v1::agent::event::TypedPayload;
        use super::v1::agent::SchemaChangedPayload;
        let payload = SchemaChangedPayload {
            schema_id: "s1".to_string(),
            database_type: "sqlite".to_string(),
        };
        let event = Event {
            r#type: "SchemaChanged".to_string(),
            payload: vec![],
            typed_payload: Some(TypedPayload::SchemaChanged(payload)),
        };
        assert!(event.typed_payload.is_some());
    }
}
