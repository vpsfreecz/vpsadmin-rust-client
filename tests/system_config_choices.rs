use serde_json::json;
use vpsadmin_client::{
    resources::system_config::show,
    runtime::{self, RawResponse},
};

#[test]
fn choices_preserve_scalar_values_and_nullable_membership() {
    let choices = json!([
        {"value": false, "label": "No"},
        {"value": 0, "label": "Zero"},
        {"value": "", "label": "Empty"}
    ]);
    for (output, expected_choices, expected_nullable) in [
        (
            json!({"choices": choices, "value_nullable": false}),
            Some(choices.clone()),
            Some(false),
        ),
        (
            json!({"choices": [], "value_nullable": true}),
            Some(json!([])),
            Some(true),
        ),
        (json!({"choices": null, "value_nullable": null}), None, None),
        (json!({}), None, None),
    ] {
        let decoded = runtime::decode::<show::Output, show::Meta>(
            &show::SPEC,
            RawResponse {
                status: 200,
                body: serde_json::to_vec(&json!({
                    "status": true,
                    "response": {"system_config": output}
                }))
                .unwrap(),
            },
        )
        .unwrap();
        assert_eq!(decoded.output.choices, expected_choices);
        assert_eq!(decoded.output.value_nullable, expected_nullable);
    }
}
