use crate::resource::ResourceData;

//TODO remove it once it's added to serde_json
pub(crate) fn merge(a: &mut ResourceData, b: ResourceData) {
    match (a, b) {
        (ResourceData::Object(a), ResourceData::Object(b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(ResourceData::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_merge() {
        // Test inserting new keys
        let mut a = json!({
            "key1": "value1",
            "key2": "value2"
        });
        let b = json!({
            "key3": "value3"
        });
        merge(&mut a, b);
        assert_eq!(
            a,
            json!({
                "key1": "value1",
                "key2": "value2",
                "key3": "value3"
            })
        );

        // Test overriding existing keys
        let mut a = json!({
            "key1": "value1",
            "key2": "value2"
        });
        let b = json!({
            "key2": "new_value2",
            "key3": "value3"
        });
        merge(&mut a, b);
        assert_eq!(
            a,
            json!({
                "key1": "value1",
                "key2": "new_value2",
                "key3": "value3"
            })
        );

        // Test overriding existing keys in nested json
        let mut a = json!({
            "key1": "value1",
            "key2": "value2",
            "key3":json!({
                "key3-1":"value3-1",
                "key3-2":"value3-2",
            })
        });
        let b = json!({
            "key2": "new_value2",
            "key3":json!({
                "key3-1":"newvalue3-1",
            })
        });
        merge(&mut a, b);
        assert_eq!(
            a,
            json!({
                "key1": "value1",
                "key2": "new_value2",
                "key3":json!({
                    "key3-1":"newvalue3-1",
                    "key3-2":"value3-2",
                })
            })
        );
    }
}
