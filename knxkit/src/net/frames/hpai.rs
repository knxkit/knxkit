// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use crate::core::util::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HPAI {
    UDP(SocketAddrV4),
    TCP(SocketAddrV4),
}

impl HPAI {
    pub const UNSPECIFIED_UDP: Self = HPAI::UDP(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0));

    pub fn new_udp(soa: SocketAddr) -> Self {
        if let SocketAddr::V4(v4) = soa {
            HPAI::UDP(v4)
        } else {
            panic!()
        }
    }

    pub fn new_tcp(soa: SocketAddr) -> Self {
        if let SocketAddr::V4(v4) = soa {
            HPAI::TCP(v4)
        } else {
            panic!()
        }
    }

    pub fn socket_addr(self) -> SocketAddrV4 {
        match self {
            HPAI::UDP(soa) => soa,
            HPAI::TCP(soa) => soa,
        }
    }
}

impl HPAI {
    pub fn parse(input0: Input) -> Result<HPAI> {
        let (input, _length) = parse_u8(input0)?;
        // FIXME check length

        let (input, (protocol_code, ip, port)) =
            parse_tuple((parse_u8, parse_u32, parse_u16))(input)?;

        let ip = std::net::Ipv4Addr::from(ip);

        let hpai = match protocol_code {
            0x01 => HPAI::UDP(SocketAddrV4::new(ip, port)),
            0x02 => HPAI::TCP(SocketAddrV4::new(ip, port)),
            _ => {
                return Err(nom::Err::Failure(Error::general(
                    "invalid protocol",
                    input0,
                )))
            }
        };

        Ok((input, hpai))
    }

    pub fn gen<W: std::io::Write>(&self) -> impl cookie_factory::SerializeFn<W> {
        let soa = self.socket_addr();

        gen_tuple((
            gen_u8(0x08), // length
            match self {
                HPAI::UDP(_) => gen_u8(0x01),
                HPAI::TCP(_) => gen_u8(0x02),
            },
            gen_slice(soa.ip().octets()),
            gen_u16(soa.port()),
        ))
    }
}

impl std::fmt::Display for HPAI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HPAI::UDP(soa) => format!("udp://{}", soa),
            HPAI::TCP(soa) => format!("tcp://{}", soa),
        };

        f.pad(&s)
    }
}

mod test {
    #[test]
    fn test() {
        use std::{net::SocketAddrV4, str::FromStr};

        let original = super::HPAI::UDP(SocketAddrV4::from_str("192.168.10.11:1516").unwrap());

        let mut buffer = vec![];
        cookie_factory::gen(super::HPAI::gen(&original), &mut buffer).unwrap();

        let decoded = super::HPAI::parse(&buffer).unwrap();

        assert_eq!(decoded.0.len(), 0);
        assert_eq!(original, decoded.1);
    }
}
