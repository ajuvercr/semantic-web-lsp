(function() {
    var implementors = Object.fromEntries([["base64",[["impl&lt;'e, E: <a class=\"trait\" href=\"base64/engine/trait.Engine.html\" title=\"trait base64::engine::Engine\">Engine</a>, R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"base64/read/struct.DecoderReader.html\" title=\"struct base64::read::DecoderReader\">DecoderReader</a>&lt;'e, E, R&gt;"]]],["bytes",[["impl&lt;B: <a class=\"trait\" href=\"bytes/buf/trait.Buf.html\" title=\"trait bytes::buf::Buf\">Buf</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"bytes/buf/struct.Reader.html\" title=\"struct bytes::buf::Reader\">Reader</a>&lt;B&gt;"]]],["digest",[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"digest/core_api/struct.XofReaderCoreWrapper.html\" title=\"struct digest::core_api::XofReaderCoreWrapper\">XofReaderCoreWrapper</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"digest/core_api/trait.XofReaderCore.html\" title=\"trait digest::core_api::XofReaderCore\">XofReaderCore</a>,\n    T::<a class=\"associatedtype\" href=\"digest/core_api/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type digest::core_api::BlockSizeUser::BlockSize\">BlockSize</a>: <a class=\"trait\" href=\"typenum/type_operators/trait.IsLess.html\" title=\"trait typenum::type_operators::IsLess\">IsLess</a>&lt;<a class=\"type\" href=\"digest/consts/type.U256.html\" title=\"type digest::consts::U256\">U256</a>&gt;,\n    <a class=\"type\" href=\"typenum/operator_aliases/type.Le.html\" title=\"type typenum::operator_aliases::Le\">Le</a>&lt;T::<a class=\"associatedtype\" href=\"digest/core_api/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type digest::core_api::BlockSizeUser::BlockSize\">BlockSize</a>, <a class=\"type\" href=\"digest/consts/type.U256.html\" title=\"type digest::consts::U256\">U256</a>&gt;: <a class=\"trait\" href=\"typenum/marker_traits/trait.NonZero.html\" title=\"trait typenum::marker_traits::NonZero\">NonZero</a>,</div>"]]],["either",[["impl&lt;L, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt;<div class=\"where\">where\n    L: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,</div>"]]],["flate2",[["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.DeflateDecoder.html\" title=\"struct flate2::bufread::DeflateDecoder\">DeflateDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.DeflateEncoder.html\" title=\"struct flate2::bufread::DeflateEncoder\">DeflateEncoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.GzDecoder.html\" title=\"struct flate2::bufread::GzDecoder\">GzDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.GzEncoder.html\" title=\"struct flate2::bufread::GzEncoder\">GzEncoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.MultiGzDecoder.html\" title=\"struct flate2::bufread::MultiGzDecoder\">MultiGzDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.ZlibDecoder.html\" title=\"struct flate2::bufread::ZlibDecoder\">ZlibDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/bufread/struct.ZlibEncoder.html\" title=\"struct flate2::bufread::ZlibEncoder\">ZlibEncoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/write/struct.GzEncoder.html\" title=\"struct flate2::write::GzEncoder\">GzEncoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.DeflateDecoder.html\" title=\"struct flate2::read::DeflateDecoder\">DeflateDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.DeflateEncoder.html\" title=\"struct flate2::read::DeflateEncoder\">DeflateEncoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.GzDecoder.html\" title=\"struct flate2::read::GzDecoder\">GzDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.GzEncoder.html\" title=\"struct flate2::read::GzEncoder\">GzEncoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.MultiGzDecoder.html\" title=\"struct flate2::read::MultiGzDecoder\">MultiGzDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.ZlibDecoder.html\" title=\"struct flate2::read::ZlibDecoder\">ZlibDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/read/struct.ZlibEncoder.html\" title=\"struct flate2::read::ZlibEncoder\">ZlibEncoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/struct.CrcReader.html\" title=\"struct flate2::CrcReader\">CrcReader</a>&lt;R&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/write/struct.DeflateDecoder.html\" title=\"struct flate2::write::DeflateDecoder\">DeflateDecoder</a>&lt;W&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/write/struct.DeflateEncoder.html\" title=\"struct flate2::write::DeflateEncoder\">DeflateEncoder</a>&lt;W&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/write/struct.GzDecoder.html\" title=\"struct flate2::write::GzDecoder\">GzDecoder</a>&lt;W&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/write/struct.ZlibDecoder.html\" title=\"struct flate2::write::ZlibDecoder\">ZlibDecoder</a>&lt;W&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"flate2/write/struct.ZlibEncoder.html\" title=\"struct flate2::write::ZlibEncoder\">ZlibEncoder</a>&lt;W&gt;"]]],["futures_lite",[["impl&lt;T: <a class=\"trait\" href=\"futures_io/if_std/trait.AsyncRead.html\" title=\"trait futures_io::if_std::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"futures_lite/io/struct.AsyncAsSync.html\" title=\"struct futures_lite::io::AsyncAsSync\">AsyncAsSync</a>&lt;'_, '_, T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"futures_io/if_std/trait.AsyncRead.html\" title=\"trait futures_io::if_std::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"futures_lite/io/struct.BlockOn.html\" title=\"struct futures_lite::io::BlockOn\">BlockOn</a>&lt;T&gt;"]]],["futures_util",[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"futures_util/io/struct.AllowStdIo.html\" title=\"struct futures_util::io::AllowStdIo\">AllowStdIo</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,</div>"]]],["mio",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for &amp;<a class=\"struct\" href=\"mio/unix/pipe/struct.Receiver.html\" title=\"struct mio::unix::pipe::Receiver\">Receiver</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"mio/net/struct.TcpStream.html\" title=\"struct mio::net::TcpStream\">TcpStream</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"mio/net/struct.UnixStream.html\" title=\"struct mio::net::UnixStream\">UnixStream</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"mio/unix/pipe/struct.Receiver.html\" title=\"struct mio::unix::pipe::Receiver\">Receiver</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for &amp;'a <a class=\"struct\" href=\"mio/net/struct.TcpStream.html\" title=\"struct mio::net::TcpStream\">TcpStream</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for &amp;'a <a class=\"struct\" href=\"mio/net/struct.UnixStream.html\" title=\"struct mio::net::UnixStream\">UnixStream</a>"]]],["native_tls",[["impl&lt;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"native_tls/struct.TlsStream.html\" title=\"struct native_tls::TlsStream\">TlsStream</a>&lt;S&gt;"]]],["openssl",[["impl&lt;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"openssl/ssl/struct.SslStream.html\" title=\"struct openssl::ssl::SslStream\">SslStream</a>&lt;S&gt;"]]],["quick_xml",[["impl&lt;'r, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"quick_xml/reader/struct.BinaryStream.html\" title=\"struct quick_xml::reader::BinaryStream\">BinaryStream</a>&lt;'r, R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,</div>"]]],["rand_core",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for dyn <a class=\"trait\" href=\"rand_core/trait.RngCore.html\" title=\"trait rand_core::RngCore\">RngCore</a>"]]],["reqwest",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"reqwest/blocking/struct.Response.html\" title=\"struct reqwest::blocking::Response\">Response</a>"]]],["socket2",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"socket2/struct.Socket.html\" title=\"struct socket2::Socket\">Socket</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for &amp;'a <a class=\"struct\" href=\"socket2/struct.Socket.html\" title=\"struct socket2::Socket\">Socket</a>"]]],["tempfile",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for &amp;<a class=\"struct\" href=\"tempfile/struct.NamedTempFile.html\" title=\"struct tempfile::NamedTempFile\">NamedTempFile</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/fs/struct.File.html\" title=\"struct std::fs::File\">File</a>&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"tempfile/struct.SpooledTempFile.html\" title=\"struct tempfile::SpooledTempFile\">SpooledTempFile</a>"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"tempfile/struct.NamedTempFile.html\" title=\"struct tempfile::NamedTempFile\">NamedTempFile</a>&lt;F&gt;"]]],["tokio_native_tls",[["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"tokio_native_tls/struct.AllowStd.html\" title=\"struct tokio_native_tls::AllowStd\">AllowStd</a>&lt;S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"tokio/io/async_read/trait.AsyncRead.html\" title=\"trait tokio::io::async_read::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,</div>"]]],["zip",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"zip/read/struct.ZipFile.html\" title=\"struct zip::read::ZipFile\">ZipFile</a>&lt;'_&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"zip/read/struct.ZipFileSeek.html\" title=\"struct zip::read::ZipFileSeek\">ZipFileSeek</a>&lt;'_, R&gt;"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[545,503,1463,561,9359,1119,466,1553,538,538,479,260,275,504,1078,612,662]}