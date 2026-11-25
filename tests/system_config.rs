use serde_json::json;
use vpsadmin_client::resources::system_config::{index, show, update};

#[test]
fn nullable_setting_choices_and_false_survive_all_output_shapes() {
    let value = json!({"choices": ["first", "second"], "value_nullable": false});
    let index: index::Output = serde_json::from_value(value.clone()).unwrap();
    let show: show::Output = serde_json::from_value(value.clone()).unwrap();
    let update: update::Output = serde_json::from_value(value).unwrap();
    for (choices, nullable) in [
        (index.choices, index.value_nullable),
        (show.choices, show.value_nullable),
        (update.choices, update.value_nullable),
    ] {
        assert_eq!(choices, Some(json!(["first", "second"])));
        assert_eq!(nullable, Some(false));
    }
    for value in [json!({}), json!({"choices": null, "value_nullable": null})] {
        let output: index::Output = serde_json::from_value(value).unwrap();
        assert_eq!(output.choices, None);
        assert_eq!(output.value_nullable, None);
    }
}
