import resolve from 'rollup-plugin-node-resolve';
import commonjs from 'rollup-plugin-commonjs';
import pkg from './package.json';

const externals = ['node-emoji', 'joi', 'assert'];

export default [
  // browser-friendly UMD build
   {
     input: 'compiled/core.js',
     external: externals,
     output: {
       name: '@eventific/rest-transport',
       file: pkg.browser,
       format: 'umd'
     },
     plugins: [
       resolve(), // so Rollup can find `ms`
       commonjs() // so Rollup can convert `ms` to an ES module
     ]
   },

  // CommonJS (for Node) and ES module (for bundlers) build.
  // (We could have three entries in the configuration array
  // instead of two, but it's quicker to generate multiple
  // builds from a single configuration where possible, using
  // an array for the `output` option, where we can specify
  // `file` and `format` for each target)
  {
    input: 'compiled/core.js',
    external: externals,
    output: [
      { file: pkg.main, format: 'cjs' },
      { file: pkg.module, format: 'es' }
    ]
  }
];
