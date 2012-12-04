/*!
CSS stylesheets, owned types, immutable after creation
*/

use std::net::url::Url;
use util::DataStream;

pub struct Stylesheet {
    inner: netsurfcss::stylesheet::CssStylesheet
}

pub impl Stylesheet {
    static fn new(url: Url, input: DataStream) -> Stylesheet {
        Stylesheet {
            inner: parser::parse_stylesheet(move url, input)
        }
    }
}
