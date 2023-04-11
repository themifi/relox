import init, {run_with_string_output} from './pkg/relox.js';
await init();

const runButton = document.getElementById("run-button");
const codeInput = document.getElementById("code-input");
const outputArea = document.getElementById("output");

runButton.addEventListener("click", event => {
    const outputText = run_with_string_output(codeInput.value);
    outputArea.value = outputText;
});
