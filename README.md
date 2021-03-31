# facial-processing-rs
A rust library enabling simple facial alignment. Note: `facial-processing-rs` contains multiple backends from multiple authors. WIP.
## Features
 - The `vulkan` and `opencl` features are for querying/indexing Vulkan and OpenCL devices respectivly. 
 - Each of the remaining features enable a backend.
 - The `dlib` feature requires that you pack-your-own-models (ship them with the final binrary). 
 - NOTE: the `default` feature contains nothing!
## Contributing
 - Please `rustfmt` all your code.
## License
 - The library itself is licensed under the APL 2.0, but each backend has its own license that you may need to follow and/or give credit to (check src/backends)
 - OpenVTuber: [https://github.com/1996scarlet/OpenVtuber], models under MIT (code itself under GPL v3). 
 - Dlib: [https://github.com/davisking/dlib], (models)[https://github.com/davisking/dlib-models]. Note that the 68 point face prediction is not availible for commercial use.

 