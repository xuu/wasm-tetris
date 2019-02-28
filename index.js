const rust = import('./pkg/wasm_tetris');

rust.then(tetris => {
  tetris.play(document.getElementById('canvas'), 25, 18, 10);
});
