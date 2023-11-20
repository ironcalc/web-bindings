import __wbg_init, { Model } from "@ironcalc/wasm";

function compute() {
    const model = new Model('en', 'UTC');
    model.setUserInput(0, 1, 1, "23");
    model.setUserInput(0, 1, 2, "=A1*3+1");
    model.evaluate();
    const result = model.getCellValueByIndex(0, 1, 2);
    console.log("Result: ", result);
}

async function init() {
  await __wbg_init();
  compute();
}
export default init;
