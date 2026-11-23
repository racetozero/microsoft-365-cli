//! Minimal ZIP archives: writing deflated entries and reading entries back,
//! enough for Teams app packages and `.sppkg` files.

use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;

fn crc32(data: &[u8]) -> u32 {
    static TABLE: std::sync::LazyLock<[u32; 256]> = std::sync::LazyLock::new(|| {
        let mut t = [0u32; 256];
        for (i, e) in t.iter_mut().enumerate() {
            let mut c = i as u32;
            for _ in 0..8 {
                c = if c & 1 != 0 {
                    0xEDB8_8320 ^ (c >> 1)
                } else {
                    c >> 1
                };
            }
            *e = c;
        }
        t
    });
    let mut c = 0xFFFF_FFFFu32;
    for b in data {
        c = TABLE[((c ^ u32::from(*b)) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFF_FFFF
}

/// Builds an archive from `(name, contents)` pairs.
pub fn write(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut central = Vec::new();
    for (name, data) in entries {
        let crc = crc32(data);
        let deflated = compress_to_vec(data, 6);
        let (method, body): (u16, &[u8]) = if deflated.len() < data.len() {
            (8, &deflated)
        } else {
            (0, data)
        };
        let offset = out.len() as u32;
        let n = name.as_bytes();
        let header = |sig: u32, central: bool| {
            let mut h = Vec::new();
            h.extend(sig.to_le_bytes());
            if central {
                h.extend(20u16.to_le_bytes()); // made by
            }
            h.extend(20u16.to_le_bytes()); // needed
            h.extend(0x0800u16.to_le_bytes()); // UTF-8 names
            h.extend(method.to_le_bytes());
            h.extend(0u16.to_le_bytes()); // time
            h.extend(0x21u16.to_le_bytes()); // date 1980-01-01
            h.extend(crc.to_le_bytes());
            h.extend((body.len() as u32).to_le_bytes());
            h.extend((data.len() as u32).to_le_bytes());
            h.extend((n.len() as u16).to_le_bytes());
            h.extend(0u16.to_le_bytes()); // extra
            if central {
                h.extend(0u16.to_le_bytes()); // comment
                h.extend(0u16.to_le_bytes()); // disk
                h.extend(0u16.to_le_bytes()); // internal attrs
                h.extend(0u32.to_le_bytes()); // external attrs
                h.extend(offset.to_le_bytes());
            }
            h.extend(n);
            h
        };
        out.extend(header(0x0403_4b50, false));
        out.extend(body);
        central.extend(header(0x0201_4b50, true));
    }
    let cd_offset = out.len() as u32;
    let cd_len = central.len() as u32;
    out.extend(central);
    out.extend(0x0605_4b50u32.to_le_bytes());
    out.extend([0u8; 4]);
    out.extend((entries.len() as u16).to_le_bytes());
    out.extend((entries.len() as u16).to_le_bytes());
    out.extend(cd_len.to_le_bytes());
    out.extend(cd_offset.to_le_bytes());
    out.extend(0u16.to_le_bytes());
    out
}

fn u16_at(b: &[u8], i: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(i..i + 2)?.try_into().ok()?))
}

fn u32_at(b: &[u8], i: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(i..i + 4)?.try_into().ok()?))
}

/// The entries of an archive as `(name, contents)`, or `None` when it is not
/// a readable ZIP file.
pub fn read(b: &[u8]) -> Option<Vec<(String, Vec<u8>)>> {
    let eocd = (0..b.len().saturating_sub(21))
        .rev()
        .find(|&i| u32_at(b, i) == Some(0x0605_4b50))?;
    let count = u16_at(b, eocd + 10)? as usize;
    let mut p = u32_at(b, eocd + 16)? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        if u32_at(b, p)? != 0x0201_4b50 {
            return None;
        }
        let method = u16_at(b, p + 10)?;
        let csize = u32_at(b, p + 20)? as usize;
        let nlen = u16_at(b, p + 28)? as usize;
        let xlen = u16_at(b, p + 30)? as usize;
        let clen = u16_at(b, p + 32)? as usize;
        let local = u32_at(b, p + 42)? as usize;
        let name = String::from_utf8_lossy(b.get(p + 46..p + 46 + nlen)?).into_owned();
        let lnlen = u16_at(b, local + 26)? as usize;
        let lxlen = u16_at(b, local + 28)? as usize;
        let start = local + 30 + lnlen + lxlen;
        let raw = b.get(start..start + csize)?;
        let data = match method {
            0 => raw.to_vec(),
            8 => decompress_to_vec(raw).ok()?,
            _ => return None,
        };
        out.push((name, data));
        p += 46 + nlen + xlen + clen;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn round_trip() {
        let big = vec![b'a'; 1000];
        let z = super::write(&[("manifest.json", b"{}"), ("big.txt", &big)]);
        let r = super::read(&z).unwrap();
        assert_eq!(r[0], ("manifest.json".to_string(), b"{}".to_vec()));
        assert_eq!(r[1].1, big);
        assert_eq!(super::crc32(b"123456789"), 0xCBF4_3926);
    }
}
