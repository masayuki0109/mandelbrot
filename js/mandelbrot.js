function belongsToMandelbrotSet(x, y) {
  const MAXIMUM_ITERATION_LIMIT = 100;
  const COMPLEX_NUMBER_THRESHOLD = 10;

  let realComponent = x;
  let imaginaryComponent = y;

  for (let i = 0; i < MAXIMUM_ITERATION_LIMIT; ++i) {
    const _realComponent =
      realComponent * realComponent -
      imaginaryComponent * imaginaryComponent +
      x;
    const _imaginaryComponent = 2 * realComponent * imaginaryComponent + y;

    realComponent = _realComponent;
    imaginaryComponent = _imaginaryComponent;
  }

  return realComponent * imaginaryComponent < COMPLEX_NUMBER_THRESHOLD;
}

export function draw() {
  const canvas = document.createElement("canvas");
  canvas.width = 1000;
  canvas.height = 800;
  document.body.appendChild(canvas);
  const ctx = canvas.getContext("2d");

  const magnificationFactor = 200;
  const panX = 3.0;
  const panY = 2.0;

  for (let x = 0; x < canvas.width; ++x) {
    for (let y = 0; y < canvas.height; ++y) {
      const belongsToSet = belongsToMandelbrotSet(
        x / magnificationFactor - panX,
        y / magnificationFactor - panY
      );

      if (belongsToSet) {
        ctx.fillRect(x, y, 1, 1);
      }
    }
  }
}