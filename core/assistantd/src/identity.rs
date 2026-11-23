use std::io;
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;

/// Identity reported by the Linux kernel for a connected Unix socket peer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientIdentity {
    pub pid: i32,
    pub uid: u32,
    pub gid: u32,
}

impl ClientIdentity {
    pub fn from_stream(stream: &UnixStream) -> io::Result<Self> {
        let mut credentials: libc::ucred = unsafe { std::mem::zeroed() };
        let mut length = std::mem::size_of::<libc::ucred>() as libc::socklen_t;

        let result = unsafe {
            libc::getsockopt(
                stream.as_raw_fd(),
                libc::SOL_SOCKET,
                libc::SO_PEERCRED,
                (&mut credentials as *mut libc::ucred).cast(),
                &mut length,
            )
        };

        if result != 0 {
            return Err(io::Error::last_os_error());
        }

        if length as usize != std::mem::size_of::<libc::ucred>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unexpected peer credential size",
            ));
        }

        if credentials.pid <= 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid peer process ID",
            ));
        }

        Ok(Self {
            pid: credentials.pid,
            uid: credentials.uid,
            gid: credentials.gid,
        })
    }

    pub fn verify_current_user(&self) -> io::Result<()> {
        let expected_uid = unsafe { libc::geteuid() };
        self.verify_uid(expected_uid)
    }

    fn verify_uid(&self, expected_uid: u32) -> io::Result<()> {
        if self.uid != expected_uid {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!(
                    "IPC peer UID {} does not match daemon UID {}",
                    self.uid, expected_uid
                ),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ClientIdentity;
    use std::io::ErrorKind;

    #[test]
    fn accepts_matching_user_id() {
        let identity = ClientIdentity {
            pid: 123,
            uid: 1000,
            gid: 1000,
        };
        assert!(identity.verify_uid(1000).is_ok());
    }

    #[test]
    fn rejects_different_user_id() {
        let identity = ClientIdentity {
            pid: 123,
            uid: 1001,
            gid: 1001,
        };
        let error = identity.verify_uid(1000).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::PermissionDenied);
    }

    #[test]
    fn retrieves_credentials_from_real_unix_socket() {
        use std::os::unix::net::UnixStream;

        let (client, server) = UnixStream::pair().unwrap();

        let identity = ClientIdentity::from_stream(&server).unwrap();

        assert_eq!(identity.pid, std::process::id() as i32);
        assert_eq!(identity.uid, unsafe { libc::geteuid() });
        assert_eq!(identity.gid, unsafe { libc::getegid() });

        drop(client);
    }
}
