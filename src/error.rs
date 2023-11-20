use wasm_bindgen::JsError;

#[derive(Debug, Clone)]
pub enum IronCalcError {
    PlainString(String),
}

impl From<String> for IronCalcError {
    fn from(error: String) -> IronCalcError {
        IronCalcError::PlainString(error)
    }
}

impl From<IronCalcError> for JsError {
    fn from(error: IronCalcError) -> JsError {
        let (kind, description) = match error {
            IronCalcError::PlainString(description) => ("PlainString", description),
        };

        JsError::new(
            &serde_json::json!({
                "kind": kind,
                "description": description,
            })
            .to_string(),
        )
    }
}
