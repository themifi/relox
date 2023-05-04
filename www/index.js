import init, {run_wasm} from './pkg/relox.js';
await init();

const runButton = document.getElementById("run-button");
const codeInput = document.getElementById("code-input");
const outputArea = document.getElementById("output");

runButton.addEventListener("click", event => {
    const outputText = run_wasm(codeInput.value);
    outputArea.value = outputText;
});
