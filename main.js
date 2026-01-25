import init, { convert_to_poweramp } from "./pkg/topamp.js";

// Initialize WASM
await init();

const presetNameInput = document.getElementById("preset-name-input");
const peqInput = document.getElementById("squig-link-peq-input");
const fileInput = document.getElementById("file-input");
const output = document.getElementById("poweramp-peq-output");
const convertButton = document.getElementById("convert-button");
const uploadButton = document.getElementById("upload-button");
const downloadButton = document.getElementById("download-button");

// Clear output on input change
const clearOutput = () => {
  output.value = "";
};
presetNameInput.addEventListener("input", clearOutput);
peqInput.addEventListener("input", clearOutput);

// Handle file selection
fileInput.addEventListener("change", (e) => {
  const file = e.target.files[0];
  if (!file) return;

  const reader = new FileReader();
  reader.onload = (event) => {
    peqInput.value = event.target.result;
  };
  reader.readAsText(file);
});

// Convert Action
convertButton.addEventListener("click", () => {
  const presetNameValue = presetNameInput.value;
  const peqValue = peqInput.value;

  if (!presetNameValue.trim() || !peqValue.trim()) return;

  try {
    output.value = convert_to_poweramp(presetNameValue, peqValue);
  } catch (e) {
    output.value = e;
  }
});

// Trigger hidden file input click
uploadButton.addEventListener("click", () => {
  fileInput.click();
});

// Download Action
downloadButton.addEventListener("click", () => {
  const presetNameValue = presetNameInput.value;
  const outputValue = output.value;

  if (!presetNameValue.trim() || !outputValue.trim()) return;

  const blob = new Blob([outputValue], { type: "application/json" });
  const url = URL.createObjectURL(blob);

  const a = document.createElement("a");
  a.href = url;
  a.download = `${presetNameValue}.json`;
  a.click();

  // Cleanup
  URL.revokeObjectURL(url);
});
