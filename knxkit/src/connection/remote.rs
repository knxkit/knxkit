// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::{
    ffi::OsString,
    fs::metadata,
    io::{Error, ErrorKind},
    net::{IpAddr, Ipv4Addr, SocketAddrV4},
    os::unix::fs::FileTypeExt,
};

#[derive(Debug, Clone)]
pub enum RemoteSpec {
    KnxIpTunnel(SocketAddrV4),
    KnxIpMulticast(SocketAddrV4),
    KnxUSB(OsString),
    KnxkitSocketUnix(OsString),
}

pub fn parse_remote(url: &str) -> Result<RemoteSpec, crate::Error> {
    let source_url = url::Url::parse(url).map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

    let source = match (
        source_url.scheme(),
        source_url.host_str(),
        source_url.port(),
        source_url.path(),
    ) {
        ("udp", Some(host), port, "") => {
            let port = port.unwrap_or(3671);

            if let Some(IpAddr::V4(v4)) = dns_lookup::lookup_host(host)?.first() {
                if v4.is_multicast() {
                    RemoteSpec::KnxIpMulticast(SocketAddrV4::new(*v4, port))
                } else {
                    RemoteSpec::KnxIpTunnel(SocketAddrV4::new(*v4, port))
                }
            } else {
                return Err(
                    Error::new(ErrorKind::NotFound, format!("invalid host: {}", host)).into(),
                );
            }
        }

        ("ipc", None, None, file) => {
            if metadata(file)?.file_type().is_socket() {
                RemoteSpec::KnxkitSocketUnix(OsString::from(file))
            } else {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("file {} does not exist or is not a socket", file),
                )
                .into());
            }
        }

        ("usb", None, None, file) => {
            if metadata(file)?.file_type().is_char_device() {
                RemoteSpec::KnxUSB(OsString::from(file))
            } else {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("file {} does not exist or not a char device", file),
                )
                .into());
            }
        }

        _ => {
            return Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                format!("invalid url: {}", url),
            )
            .into());
        }
    };

    Ok(source)
}

pub async fn connect(
    local: Ipv4Addr,
    remote: &RemoteSpec,
) -> Result<impl super::KnxBusConnection, crate::Error> {
    match remote {
        RemoteSpec::KnxIpTunnel(soa) => {
            Ok(crate::net::tunnel::TunnelConnection::start(local, *soa).await?)
        }
        _ => {
            unimplemented!("only KnxIpTunnel source is currently implemented")
        }
    }
}
