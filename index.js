const js = import('./wasm/wasm_tetris');

js.then(tetris => {
  tetris.play(document.getElementById('canvas'), 25, 18, 10);
});
