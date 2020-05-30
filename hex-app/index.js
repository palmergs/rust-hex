import * as wasm from "rust-hex";

function normalJS() {
  console.log("In a normal JS file");
}

normalJS();
console.log("Did this work?");
console.log(wasm.weasel());
console.log(wasm.smile());
