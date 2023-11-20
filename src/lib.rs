mod error;

use wasm_bindgen::{
    prelude::{wasm_bindgen, JsError},
    JsValue,
};

use ironcalc_base::{cell::CellValue, model::Model as BaseModel};

use crate::error::IronCalcError;

#[wasm_bindgen]
pub struct Model {
    model: BaseModel,
}

#[wasm_bindgen]
impl Model {
    #[wasm_bindgen(constructor)]
    pub fn new(locale: &str, timezone: &str) -> Result<Model, JsError> {
        let model =
            BaseModel::new_empty("workbook", locale, timezone).map_err(IronCalcError::from)?;
        Ok(Model { model })
    }

    #[wasm_bindgen(js_name=loadFromJson)]
    pub fn load_from_json(workbook_json: &str) -> Result<Model, JsError> {
        let model = BaseModel::from_json(workbook_json).map_err(IronCalcError::from)?;
        Ok(Model { model })
    }

    #[wasm_bindgen(js_name = "setUserInput")]
    pub fn set_user_input(
        &mut self,
        sheet: u32,
        row: i32,
        column: i32,
        input: String,
    ) -> Result<(), JsError> {
        self.model.set_user_input(sheet, row, column, input);
        Ok(())
    }

    #[wasm_bindgen(js_name = "getCellValueByIndex")]
    pub fn get_cell_value_by_index(
        &self,
        sheet: u32,
        row: i32,
        column: i32,
    ) -> Result<JsValue, JsError> {
        Ok(
            match self
                .model
                .get_cell_value_by_index(sheet, row, column)
                .map_err(IronCalcError::from)?
            {
                CellValue::None => JsValue::NULL,
                CellValue::String(s) => JsValue::from(s),
                CellValue::Number(f) => JsValue::from(f),
                CellValue::Boolean(b) => JsValue::from(b),
            },
        )
    }

    pub fn evaluate(&mut self) {
        self.model.evaluate();
    }
}
