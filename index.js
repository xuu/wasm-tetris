const js = import('./wasm/wasm_tetris');

js.then(tetris => {
  tetris.play('canvas', 25, 18, 10);
});
