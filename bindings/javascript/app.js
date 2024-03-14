import inference from './dist/composed.js';
const result = inference.compute("What is the Capital of Germany?");
console.log(`Result: ${result}`);
