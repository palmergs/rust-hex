import * as wasm from "rust-hex";

const resize = function() {
  console.log("resizing canvas...");
  let can = document.getElementById("canvas");
  can.width = window.innerWidth - 18;
  can.height = window.innerHeight - 100;
  console.log("...canvas resized: ", can.width, can.height);
};

window.addEventListener('load', (evt) => {
  console.log("in load..."+ evt);
  resize();
  console.log(wasm.draw_hexes());
  console.log(wasm.weasel());
  wasm.start_loop();
}, false);

window.addEventListener('resize', (evt) => {
  console.log("in resize...");
  resize();
  console.log(wasm.draw_hexes());
  console.log(wasm.weasel());
}, false);

