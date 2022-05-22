# csvtosql-app
This is a small, experimental, WASM based web application for generating sql table creation statements from a csv file. It should be broken out into separate components in the future, as the entire app is one big component at the moment.

All processing is done on device, and no backend server is involved.

### Building
Requires rust and [Trunk](https://trunkrs.dev/).

Run ``trunk serve`` to build and run locally.

Run ``trunk build --release`` to compile a build in release mode.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details