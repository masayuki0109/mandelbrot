import { draw } from "./mandelbrot";
import { fib } from "./fib";

import("../pkg/index.js")
  .then((module) => {
    withCalcTime(draw, undefined, "js mandelbrot");
    withCalcTime(module.draw, undefined, "rust mandelbrot");
    withCalcTime(fib, 35, "js fib");
    withCalcTime(module.fib, 35, "rust fib");
  })
  .catch(console.error);

const withCalcTime = (fn, args, log) => {
  const startTime = performance.now(); // 開始時間
  let result = fn(args);
  const endTime = performance.now(); // 終了時間
  console.log(endTime - startTime, log);
  return result;
};
