# IronCalc Web bindings

This package contains web bindings for IronCalc. Note that it does not contain the xlsx writer and reader, only the engine.


## Usage

In your project

```
npm install @ironcalc/wasm
```

And then in your TypeScript

```TypeScript
import __wbg_init, { Model } from "@ironcalc/wasm";

async function compute() {
    await __wbg_init();
    const model = new Model('en', 'UTC');
    
    model.setUserInput(0, 1, 1, "23");
    model.setUserInput(0, 1, 2, "=A1*3+1");
    model.evaluate();
    
    const result = model.getCellValueByIndex(0, 1, 2);
    
    console.log("Result: ", result);
}

compute();
```
