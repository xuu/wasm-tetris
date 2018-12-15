const tetris = import('./wasm/wasm_tetris.js');

tetris.then(t => {
  t.play('canvas', 25, 20, 10);
});
