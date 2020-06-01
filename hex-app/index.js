import * as wasm from "rust-hex";

function resizeCanvas() {
  console.log("resizing canvas...");
  let can = document.getElementById("canvas");
  can.width = document.body.clientWidth;
  // can.height = document.body.clientHeight;
  console.log("...canvas resized: ", can.width, can.height);
};

window.onload = () => {
  resizeCanvas();
  console.log(wasm.weasel());
  console.log(wasm.draw_hexes());
  wasm.start_loop();
}

window.onresize = () => {
  resizeCanvas();
  console.log(wasm.weasel());
  console.log(wasm.draw_hexes());
}
