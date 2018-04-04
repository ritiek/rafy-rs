
error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        // PythonObjectDowncastError(cpython::PythonObjectDowncastError);
        IO(::std::io::Error);
        Json(::json::Error);
        Parse(::std::num::ParseIntError);
        NativeTls(::hyper_native_tls::native_tls::Error);
        Hyper(::hyper::Error);
        Python(::cpython::PyErr);
    }

    errors {
        /// The requested video was not found.
        VideoNotFound {}
        /// A network request has failed.
        NetworkRequestFailed {}
    }
}
