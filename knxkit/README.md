# High-level API for KNX devices and networks
__knxkit__ is a library for interfacing with [KNX](https://www.knx.org) devices and networks. It provides a high-level API for interacting with KNX devices and networks, and is designed to be easy to use and flexible.

- __major data structures__ — individual and group addresses, CEMI, APDU, DataPoint, KNXnet/IP frames

- __KNXnet/IP services__ — search, describe, tunneling are implemented.

- __Project files__ — knxkit can process _.knxproj_ files, which are used by ETS to store KNX project information. Devices and group definitions can be extracted. In addition it DPT definitions, that are used for code generation.

- data structures for all __Data point types (DPT)__  generated from knx_master.xml contents. Serialzation to and from JSON is supported. _Display_ implementations are generated.
- __command line__ utilities for data structures generation and knx network interaction.

### Related crates
  - [knxkit_dpt](knxkit_dpt) - DPT definitions
  - [knxkit_dptgen](knxkit_dptgen) - DPT definitions generator
  - [knxkit_cli](knxkit_cli) - command line utility

### Status
Currently the library is in development. It may be used for testing and experimentation, but is not yet ready for production use. The API is subject to change. The library is being developed in the open, and contributions are welcome.

### Plan
In addition to general improvements and bug fixes, the following features are planned for the library:
  - [ ] KNXnet/IP routing
  - [ ] KNXnet/IP secure
  - [ ] USB tunneling
  - [ ] multiplexing of single tunnel connection over unix domain socket
  
Tools and applications  
  - [x] knx2mqtt
  - [ ] knx2ws (WebSocket)
  - [ ] knx2prometheus
  - [ ] scriptable logic engine

<hr>
<sup>
knxkit is licensed under either of <a href="LICENSE.EPL">Eclipse Public License - v2.0</a> or <a href="LICENSE.GPL">GNU General Public License v3</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you  shall
be dual licensed as above, without any additional terms or conditions.
</sub>
