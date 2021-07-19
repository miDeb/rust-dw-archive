use std::io::prelude::*;

use flate2::read::GzDecoder;

const FILES: &str = "2019-October.txt
2019-August.txt.gz
2019-April.txt.gz
2018-January.txt.gz
2017-December.txt.gz
2017-October.txt.gz
2016-March.txt.gz
2016-January.txt.gz
2015-November.txt.gz
2015-September.txt.gz
2015-August.txt.gz
2015-May.txt.gz
2015-April.txt.gz
2015-February.txt.gz
2015-January.txt.gz
2014-December.txt.gz
2014-November.txt.gz
2014-October.txt.gz
2014-September.txt.gz
2014-August.txt.gz
2014-July.txt.gz
2014-June.txt.gz
2014-May.txt.gz
2014-April.txt.gz
2014-March.txt.gz
2014-February.txt.gz
2014-January.txt.gz
2013-December.txt.gz
2013-November.txt.gz
2013-October.txt.gz
2013-September.txt.gz
2013-August.txt.gz
2013-July.txt.gz
2013-June.txt.gz
2013-May.txt.gz
2013-April.txt.gz
2013-March.txt.gz
2013-February.txt.gz
2013-January.txt.gz
2012-December.txt.gz
2012-November.txt.gz
2012-October.txt.gz
2012-September.txt.gz
2012-August.txt.gz
2012-July.txt.gz
2012-June.txt.gz
2012-May.txt.gz
2012-April.txt.gz
2012-March.txt.gz
2012-February.txt.gz
2012-January.txt.gz
2011-December.txt.gz
2011-November.txt.gz
2011-October.txt.gz
2011-September.txt.gz
2011-August.txt.gz
2011-July.txt.gz
2011-June.txt.gz
2011-May.txt.gz
2011-April.txt.gz
2011-March.txt.gz
2011-February.txt.gz
2011-January.txt.gz
2010-December.txt.gz
2010-November.txt.gz
2010-October.txt.gz
2010-September.txt.gz
2010-August.txt.gz
2010-July.txt.gz";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for file in FILES.lines() {
        let resp = reqwest::get(format!(
            "https://lists.mozilla.org/pipermail/rust-dev/{}",
            file
        ))
        .await?
        .bytes()
        .await?;
        let resp = if file.ends_with(".gz") {
            let mut d = GzDecoder::new(resp.as_ref());
            let mut s = String::new();
            d.read_to_string(&mut s).unwrap();
            s
        } else {
            String::from(std::str::from_utf8(resp.as_ref())?)
        };
        std::fs::File::create(format!("out/{}", file.strip_suffix(".gz").unwrap_or(file)))
            .unwrap()
            .write_all(resp.as_bytes())?;
    }
    Ok(())
}
