# rust-ntpc-minimal

Minimal NTP client, written in Rust.

### Limitations

It returns the **transmit** timestamp only, in seconds (_precision is cut off for now_), received from `pool.ntp.org`, and discards any other information in the datagram packet received from the NTP server, which, by the way, cannot be specified from the command line (_yet_).

### _Shameless and prententious performance test:_

```
sntp -d pool.ntp.org  0.00s user 0.01s system 15% cpu 0.055 total
...

./target/release/rust-ntpc-minimal  0.00s user 0.00s system 13% cpu 0.038 total
```

## License: MIT

Copyright (C) 2018 Giuseppe Lobraico

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
