import * as wasm from "rust-hex";

function resizeCanvas() {
  console.log("resizing canvas...");
  let can = document.getElementById("canvas");
  can.width = document.body.clientWidth;
  can.height = document.body.clientHeight;
  console.log("...canvas resized");
};

window.onload = () => {
  resizeCanvas();
  console.log(wasm.weasel());
  console.log(wasm.smile());
}

window.onresize = () => {
  resizeCanvas();
  console.log(wasm.weasel());
  console.log(wasm.smile());
}
