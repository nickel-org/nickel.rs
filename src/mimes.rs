use http::headers::content_type::MediaType;

pub fn get_media_type(ext: &str) -> Option<MediaType> {
    match ext {
    "ez" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "andrew-inset".to_string(),
        parameters: vec![]
    }),
    "aw" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "applixware".to_string(),
        parameters: vec![]
    }),
    "atom" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "atom+xml".to_string(),
        parameters: vec![]
    }),
    "atomcat" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "atomcat+xml".to_string(),
        parameters: vec![]
    }),
    "atomsvc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "atomsvc+xml".to_string(),
        parameters: vec![]
    }),
    "ccxml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "ccxml+xml".to_string(),
        parameters: vec![]
    }),
    "cdmia" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "cdmi-capability".to_string(),
        parameters: vec![]
    }),
    "cdmic" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "cdmi-container".to_string(),
        parameters: vec![]
    }),
    "cdmid" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "cdmi-domain".to_string(),
        parameters: vec![]
    }),
    "cdmio" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "cdmi-object".to_string(),
        parameters: vec![]
    }),
    "cdmiq" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "cdmi-queue".to_string(),
        parameters: vec![]
    }),
    "cu" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "cu-seeme".to_string(),
        parameters: vec![]
    }),
    "davmount" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "davmount+xml".to_string(),
        parameters: vec![]
    }),
    "dbk" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "docbook+xml".to_string(),
        parameters: vec![]
    }),
    "dssc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "dssc+der".to_string(),
        parameters: vec![]
    }),
    "xdssc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "dssc+xml".to_string(),
        parameters: vec![]
    }),
    "ecma" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "ecmascript".to_string(),
        parameters: vec![]
    }),
    "emma" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "emma+xml".to_string(),
        parameters: vec![]
    }),
    "epub" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "epub+zip".to_string(),
        parameters: vec![]
    }),
    "exi" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "exi".to_string(),
        parameters: vec![]
    }),
    "pfr" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "font-tdpfr".to_string(),
        parameters: vec![]
    }),
    "gml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "gml+xml".to_string(),
        parameters: vec![]
    }),
    "gpx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "gpx+xml".to_string(),
        parameters: vec![]
    }),
    "gxf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "gxf".to_string(),
        parameters: vec![]
    }),
    "stk" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "hyperstudio".to_string(),
        parameters: vec![]
    }),
    "ink" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "inkml+xml".to_string(),
        parameters: vec![]
    }),
    "ipfix" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "ipfix".to_string(),
        parameters: vec![]
    }),
    "jar" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "java-archive".to_string(),
        parameters: vec![]
    }),
    "ser" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "java-serialized-object".to_string(),
        parameters: vec![]
    }),
    "class" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "java-vm".to_string(),
        parameters: vec![]
    }),
    "js" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "javascript".to_string(),
        parameters: vec![]
    }),
    "json" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "json".to_string(),
        parameters: vec![]
    }),
    "jsonml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "jsonml+json".to_string(),
        parameters: vec![]
    }),
    "lostxml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "lost+xml".to_string(),
        parameters: vec![]
    }),
    "hqx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "mac-binhex40".to_string(),
        parameters: vec![]
    }),
    "cpt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "mac-compactpro".to_string(),
        parameters: vec![]
    }),
    "mads" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "mads+xml".to_string(),
        parameters: vec![]
    }),
    "mrc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "marc".to_string(),
        parameters: vec![]
    }),
    "mrcx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "marcxml+xml".to_string(),
        parameters: vec![]
    }),
    "ma" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "mathematica".to_string(),
        parameters: vec![]
    }),
    "mathml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "mathml+xml".to_string(),
        parameters: vec![]
    }),
    "mbox" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "mbox".to_string(),
        parameters: vec![]
    }),
    "mscml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "mediaservercontrol+xml".to_string(),
        parameters: vec![]
    }),
    "metalink" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "metalink+xml".to_string(),
        parameters: vec![]
    }),
    "meta4" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "metalink4+xml".to_string(),
        parameters: vec![]
    }),
    "mets" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "mets+xml".to_string(),
        parameters: vec![]
    }),
    "mods" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "mods+xml".to_string(),
        parameters: vec![]
    }),
    "m21" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "mp21".to_string(),
        parameters: vec![]
    }),
    "mp4s" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "mp4".to_string(),
        parameters: vec![]
    }),
    "doc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "msword".to_string(),
        parameters: vec![]
    }),
    "mxf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "mxf".to_string(),
        parameters: vec![]
    }),
    "bin" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "octet-stream".to_string(),
        parameters: vec![]
    }),
    "oda" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "oda".to_string(),
        parameters: vec![]
    }),
    "opf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "oebps-package+xml".to_string(),
        parameters: vec![]
    }),
    "ogx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "ogg".to_string(),
        parameters: vec![]
    }),
    "omdoc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "omdoc+xml".to_string(),
        parameters: vec![]
    }),
    "onetoc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "onenote".to_string(),
        parameters: vec![]
    }),
    "oxps" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "oxps".to_string(),
        parameters: vec![]
    }),
    "xer" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "patch-ops-error+xml".to_string(),
        parameters: vec![]
    }),
    "pdf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pdf".to_string(),
        parameters: vec![]
    }),
    "pgp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pgp-encrypted".to_string(),
        parameters: vec![]
    }),
    "asc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pgp-signature".to_string(),
        parameters: vec![]
    }),
    "prf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pics-rules".to_string(),
        parameters: vec![]
    }),
    "p10" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pkcs10".to_string(),
        parameters: vec![]
    }),
    "p7m" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pkcs7-mime".to_string(),
        parameters: vec![]
    }),
    "p7s" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pkcs7-signature".to_string(),
        parameters: vec![]
    }),
    "p8" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pkcs8".to_string(),
        parameters: vec![]
    }),
    "ac" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pkix-attr-cert".to_string(),
        parameters: vec![]
    }),
    "cer" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pkix-cert".to_string(),
        parameters: vec![]
    }),
    "crl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pkix-crl".to_string(),
        parameters: vec![]
    }),
    "pkipath" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pkix-pkipath".to_string(),
        parameters: vec![]
    }),
    "pki" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pkixcmp".to_string(),
        parameters: vec![]
    }),
    "pls" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pls+xml".to_string(),
        parameters: vec![]
    }),
    "ai" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "postscript".to_string(),
        parameters: vec![]
    }),
    "cww" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "prs.cww".to_string(),
        parameters: vec![]
    }),
    "pskcxml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "pskc+xml".to_string(),
        parameters: vec![]
    }),
    "rdf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "rdf+xml".to_string(),
        parameters: vec![]
    }),
    "rif" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "reginfo+xml".to_string(),
        parameters: vec![]
    }),
    "rnc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "relax-ng-compact-syntax".to_string(),
        parameters: vec![]
    }),
    "rl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "resource-lists+xml".to_string(),
        parameters: vec![]
    }),
    "rld" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "resource-lists-diff+xml".to_string(),
        parameters: vec![]
    }),
    "rs" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "rls-services+xml".to_string(),
        parameters: vec![]
    }),
    "gbr" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "rpki-ghostbusters".to_string(),
        parameters: vec![]
    }),
    "mft" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "rpki-manifest".to_string(),
        parameters: vec![]
    }),
    "roa" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "rpki-roa".to_string(),
        parameters: vec![]
    }),
    "rsd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "rsd+xml".to_string(),
        parameters: vec![]
    }),
    "rss" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "rss+xml".to_string(),
        parameters: vec![]
    }),
    "rtf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "rtf".to_string(),
        parameters: vec![]
    }),
    "sbml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "sbml+xml".to_string(),
        parameters: vec![]
    }),
    "scq" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "scvp-cv-request".to_string(),
        parameters: vec![]
    }),
    "scs" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "scvp-cv-response".to_string(),
        parameters: vec![]
    }),
    "spq" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "scvp-vp-request".to_string(),
        parameters: vec![]
    }),
    "spp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "scvp-vp-response".to_string(),
        parameters: vec![]
    }),
    "sdp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "sdp".to_string(),
        parameters: vec![]
    }),
    "setpay" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "set-payment-initiation".to_string(),
        parameters: vec![]
    }),
    "setreg" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "set-registration-initiation".to_string(),
        parameters: vec![]
    }),
    "shf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "shf+xml".to_string(),
        parameters: vec![]
    }),
    "smi" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "smil+xml".to_string(),
        parameters: vec![]
    }),
    "rq" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "sparql-query".to_string(),
        parameters: vec![]
    }),
    "srx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "sparql-results+xml".to_string(),
        parameters: vec![]
    }),
    "gram" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "srgs".to_string(),
        parameters: vec![]
    }),
    "grxml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "srgs+xml".to_string(),
        parameters: vec![]
    }),
    "sru" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "sru+xml".to_string(),
        parameters: vec![]
    }),
    "ssdl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "ssdl+xml".to_string(),
        parameters: vec![]
    }),
    "ssml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "ssml+xml".to_string(),
        parameters: vec![]
    }),
    "tei" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "tei+xml".to_string(),
        parameters: vec![]
    }),
    "tfi" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "thraud+xml".to_string(),
        parameters: vec![]
    }),
    "tsd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "timestamped-data".to_string(),
        parameters: vec![]
    }),
    "plb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.3gpp.pic-bw-large".to_string(),
        parameters: vec![]
    }),
    "psb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.3gpp.pic-bw-small".to_string(),
        parameters: vec![]
    }),
    "pvb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.3gpp.pic-bw-var".to_string(),
        parameters: vec![]
    }),
    "tcap" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.3gpp2.tcap".to_string(),
        parameters: vec![]
    }),
    "pwn" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.3m.post-it-notes".to_string(),
        parameters: vec![]
    }),
    "aso" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.accpac.simply.aso".to_string(),
        parameters: vec![]
    }),
    "imp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.accpac.simply.imp".to_string(),
        parameters: vec![]
    }),
    "acu" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.acucobol".to_string(),
        parameters: vec![]
    }),
    "atc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.acucorp".to_string(),
        parameters: vec![]
    }),
    "air" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.adobe.air-application-installer-package+zip".to_string(),
        parameters: vec![]
    }),
    "fcdt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.adobe.formscentral.fcdt".to_string(),
        parameters: vec![]
    }),
    "fxp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.adobe.fxp".to_string(),
        parameters: vec![]
    }),
    "xdp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.adobe.xdp+xml".to_string(),
        parameters: vec![]
    }),
    "xfdf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.adobe.xfdf".to_string(),
        parameters: vec![]
    }),
    "ahead" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ahead.space".to_string(),
        parameters: vec![]
    }),
    "azf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.airzip.filesecure.azf".to_string(),
        parameters: vec![]
    }),
    "azs" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.airzip.filesecure.azs".to_string(),
        parameters: vec![]
    }),
    "azw" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.amazon.ebook".to_string(),
        parameters: vec![]
    }),
    "acc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.americandynamics.acc".to_string(),
        parameters: vec![]
    }),
    "ami" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.amiga.ami".to_string(),
        parameters: vec![]
    }),
    "apk" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.android.package-archive".to_string(),
        parameters: vec![]
    }),
    "cii" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.anser-web-certificate-issue-initiation".to_string(),
        parameters: vec![]
    }),
    "fti" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.anser-web-funds-transfer-initiation".to_string(),
        parameters: vec![]
    }),
    "atx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.antix.game-component".to_string(),
        parameters: vec![]
    }),
    "mpkg" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.apple.installer+xml".to_string(),
        parameters: vec![]
    }),
    "m3u8" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.apple.mpegurl".to_string(),
        parameters: vec![]
    }),
    "swi" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.aristanetworks.swi".to_string(),
        parameters: vec![]
    }),
    "iota" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.astraea-software.iota".to_string(),
        parameters: vec![]
    }),
    "aep" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.audiograph".to_string(),
        parameters: vec![]
    }),
    "mpm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.blueice.multipass".to_string(),
        parameters: vec![]
    }),
    "bmi" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.bmi".to_string(),
        parameters: vec![]
    }),
    "rep" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.businessobjects".to_string(),
        parameters: vec![]
    }),
    "cdxml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.chemdraw+xml".to_string(),
        parameters: vec![]
    }),
    "mmd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.chipnuts.karaoke-mmd".to_string(),
        parameters: vec![]
    }),
    "cdy" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.cinderella".to_string(),
        parameters: vec![]
    }),
    "cla" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.claymore".to_string(),
        parameters: vec![]
    }),
    "rp9" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.cloanto.rp9".to_string(),
        parameters: vec![]
    }),
    "c4g" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.clonk.c4group".to_string(),
        parameters: vec![]
    }),
    "c11amc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.cluetrust.cartomobile-config".to_string(),
        parameters: vec![]
    }),
    "c11amz" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.cluetrust.cartomobile-config-pkg".to_string(),
        parameters: vec![]
    }),
    "csp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.commonspace".to_string(),
        parameters: vec![]
    }),
    "cdbcmsg" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.contact.cmsg".to_string(),
        parameters: vec![]
    }),
    "cmc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.cosmocaller".to_string(),
        parameters: vec![]
    }),
    "clkx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.crick.clicker".to_string(),
        parameters: vec![]
    }),
    "clkk" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.crick.clicker.keyboard".to_string(),
        parameters: vec![]
    }),
    "clkp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.crick.clicker.palette".to_string(),
        parameters: vec![]
    }),
    "clkt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.crick.clicker.template".to_string(),
        parameters: vec![]
    }),
    "clkw" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.crick.clicker.wordbank".to_string(),
        parameters: vec![]
    }),
    "wbs" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.criticaltools.wbs+xml".to_string(),
        parameters: vec![]
    }),
    "pml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ctc-posml".to_string(),
        parameters: vec![]
    }),
    "ppd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.cups-ppd".to_string(),
        parameters: vec![]
    }),
    "car" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.curl.car".to_string(),
        parameters: vec![]
    }),
    "pcurl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.curl.pcurl".to_string(),
        parameters: vec![]
    }),
    "dart" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.dart".to_string(),
        parameters: vec![]
    }),
    "rdz" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.data-vision.rdz".to_string(),
        parameters: vec![]
    }),
    "uvf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.dece.data".to_string(),
        parameters: vec![]
    }),
    "uvt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.dece.ttml+xml".to_string(),
        parameters: vec![]
    }),
    "uvx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.dece.unspecified".to_string(),
        parameters: vec![]
    }),
    "uvz" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.dece.zip".to_string(),
        parameters: vec![]
    }),
    "fe_launch" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.denovo.fcselayout-link".to_string(),
        parameters: vec![]
    }),
    "dna" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.dna".to_string(),
        parameters: vec![]
    }),
    "mlp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.dolby.mlp".to_string(),
        parameters: vec![]
    }),
    "dpg" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.dpgraph".to_string(),
        parameters: vec![]
    }),
    "dfac" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.dreamfactory".to_string(),
        parameters: vec![]
    }),
    "kpxx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ds-keypoint".to_string(),
        parameters: vec![]
    }),
    "ait" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.dvb.ait".to_string(),
        parameters: vec![]
    }),
    "svc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.dvb.service".to_string(),
        parameters: vec![]
    }),
    "geo" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.dynageo".to_string(),
        parameters: vec![]
    }),
    "mag" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ecowin.chart".to_string(),
        parameters: vec![]
    }),
    "nml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.enliven".to_string(),
        parameters: vec![]
    }),
    "esf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.epson.esf".to_string(),
        parameters: vec![]
    }),
    "msf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.epson.msf".to_string(),
        parameters: vec![]
    }),
    "qam" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.epson.quickanime".to_string(),
        parameters: vec![]
    }),
    "slt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.epson.salt".to_string(),
        parameters: vec![]
    }),
    "ssf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.epson.ssf".to_string(),
        parameters: vec![]
    }),
    "es3" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.eszigno3+xml".to_string(),
        parameters: vec![]
    }),
    "ez2" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ezpix-album".to_string(),
        parameters: vec![]
    }),
    "ez3" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ezpix-package".to_string(),
        parameters: vec![]
    }),
    "fdf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fdf".to_string(),
        parameters: vec![]
    }),
    "mseed" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fdsn.mseed".to_string(),
        parameters: vec![]
    }),
    "seed" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fdsn.seed".to_string(),
        parameters: vec![]
    }),
    "gph" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.flographit".to_string(),
        parameters: vec![]
    }),
    "ftc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fluxtime.clip".to_string(),
        parameters: vec![]
    }),
    "fm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.framemaker".to_string(),
        parameters: vec![]
    }),
    "fnc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.frogans.fnc".to_string(),
        parameters: vec![]
    }),
    "ltf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.frogans.ltf".to_string(),
        parameters: vec![]
    }),
    "fsc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fsc.weblaunch".to_string(),
        parameters: vec![]
    }),
    "oas" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fujitsu.oasys".to_string(),
        parameters: vec![]
    }),
    "oa2" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fujitsu.oasys2".to_string(),
        parameters: vec![]
    }),
    "oa3" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fujitsu.oasys3".to_string(),
        parameters: vec![]
    }),
    "fg5" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fujitsu.oasysgp".to_string(),
        parameters: vec![]
    }),
    "bh2" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fujitsu.oasysprs".to_string(),
        parameters: vec![]
    }),
    "ddd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fujixerox.ddd".to_string(),
        parameters: vec![]
    }),
    "xdw" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fujixerox.docuworks".to_string(),
        parameters: vec![]
    }),
    "xbd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fujixerox.docuworks.binder".to_string(),
        parameters: vec![]
    }),
    "fzs" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.fuzzysheet".to_string(),
        parameters: vec![]
    }),
    "txd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.genomatix.tuxedo".to_string(),
        parameters: vec![]
    }),
    "ggb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.geogebra.file".to_string(),
        parameters: vec![]
    }),
    "ggt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.geogebra.tool".to_string(),
        parameters: vec![]
    }),
    "gex" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.geometry-explorer".to_string(),
        parameters: vec![]
    }),
    "gxt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.geonext".to_string(),
        parameters: vec![]
    }),
    "g2w" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.geoplan".to_string(),
        parameters: vec![]
    }),
    "g3w" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.geospace".to_string(),
        parameters: vec![]
    }),
    "gmx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.gmx".to_string(),
        parameters: vec![]
    }),
    "kml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.google-earth.kml+xml".to_string(),
        parameters: vec![]
    }),
    "kmz" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.google-earth.kmz".to_string(),
        parameters: vec![]
    }),
    "gqf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.grafeq".to_string(),
        parameters: vec![]
    }),
    "gac" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.groove-account".to_string(),
        parameters: vec![]
    }),
    "ghf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.groove-help".to_string(),
        parameters: vec![]
    }),
    "gim" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.groove-identity-message".to_string(),
        parameters: vec![]
    }),
    "grv" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.groove-injector".to_string(),
        parameters: vec![]
    }),
    "gtm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.groove-tool-message".to_string(),
        parameters: vec![]
    }),
    "tpl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.groove-tool-template".to_string(),
        parameters: vec![]
    }),
    "vcg" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.groove-vcard".to_string(),
        parameters: vec![]
    }),
    "hal" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.hal+xml".to_string(),
        parameters: vec![]
    }),
    "zmm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.handheld-entertainment+xml".to_string(),
        parameters: vec![]
    }),
    "hbci" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.hbci".to_string(),
        parameters: vec![]
    }),
    "les" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.hhe.lesson-player".to_string(),
        parameters: vec![]
    }),
    "hpgl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.hp-hpgl".to_string(),
        parameters: vec![]
    }),
    "hpid" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.hp-hpid".to_string(),
        parameters: vec![]
    }),
    "hps" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.hp-hps".to_string(),
        parameters: vec![]
    }),
    "jlt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.hp-jlyt".to_string(),
        parameters: vec![]
    }),
    "pcl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.hp-pcl".to_string(),
        parameters: vec![]
    }),
    "pclxl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.hp-pclxl".to_string(),
        parameters: vec![]
    }),
    "sfd-hdstx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.hydrostatix.sof-data".to_string(),
        parameters: vec![]
    }),
    "mpy" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ibm.minipay".to_string(),
        parameters: vec![]
    }),
    "afp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ibm.modcap".to_string(),
        parameters: vec![]
    }),
    "irm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ibm.rights-management".to_string(),
        parameters: vec![]
    }),
    "sc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ibm.secure-container".to_string(),
        parameters: vec![]
    }),
    "icc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.iccprofile".to_string(),
        parameters: vec![]
    }),
    "igl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.igloader".to_string(),
        parameters: vec![]
    }),
    "ivp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.immervision-ivp".to_string(),
        parameters: vec![]
    }),
    "ivu" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.immervision-ivu".to_string(),
        parameters: vec![]
    }),
    "igm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.insors.igm".to_string(),
        parameters: vec![]
    }),
    "xpw" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.intercon.formnet".to_string(),
        parameters: vec![]
    }),
    "i2g" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.intergeo".to_string(),
        parameters: vec![]
    }),
    "qbo" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.intu.qbo".to_string(),
        parameters: vec![]
    }),
    "qfx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.intu.qfx".to_string(),
        parameters: vec![]
    }),
    "rcprofile" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ipunplugged.rcprofile".to_string(),
        parameters: vec![]
    }),
    "irp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.irepository.package+xml".to_string(),
        parameters: vec![]
    }),
    "xpr" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.is-xpr".to_string(),
        parameters: vec![]
    }),
    "fcs" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.isac.fcs".to_string(),
        parameters: vec![]
    }),
    "jam" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.jam".to_string(),
        parameters: vec![]
    }),
    "rms" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.jcp.javame.midlet-rms".to_string(),
        parameters: vec![]
    }),
    "jisp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.jisp".to_string(),
        parameters: vec![]
    }),
    "joda" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.joost.joda-archive".to_string(),
        parameters: vec![]
    }),
    "ktz" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.kahootz".to_string(),
        parameters: vec![]
    }),
    "karbon" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.kde.karbon".to_string(),
        parameters: vec![]
    }),
    "chrt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.kde.kchart".to_string(),
        parameters: vec![]
    }),
    "kfo" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.kde.kformula".to_string(),
        parameters: vec![]
    }),
    "flw" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.kde.kivio".to_string(),
        parameters: vec![]
    }),
    "kon" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.kde.kontour".to_string(),
        parameters: vec![]
    }),
    "kpr" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.kde.kpresenter".to_string(),
        parameters: vec![]
    }),
    "ksp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.kde.kspread".to_string(),
        parameters: vec![]
    }),
    "kwd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.kde.kword".to_string(),
        parameters: vec![]
    }),
    "htke" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.kenameaapp".to_string(),
        parameters: vec![]
    }),
    "kia" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.kidspiration".to_string(),
        parameters: vec![]
    }),
    "kne" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.kinar".to_string(),
        parameters: vec![]
    }),
    "skp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.koan".to_string(),
        parameters: vec![]
    }),
    "sse" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.kodak-descriptor".to_string(),
        parameters: vec![]
    }),
    "lasxml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.las.las+xml".to_string(),
        parameters: vec![]
    }),
    "lbd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.llamagraphics.life-balance.desktop".to_string(),
        parameters: vec![]
    }),
    "lbe" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.llamagraphics.life-balance.exchange+xml".to_string(),
        parameters: vec![]
    }),
    "123" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.lotus-1-2-3".to_string(),
        parameters: vec![]
    }),
    "apr" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.lotus-approach".to_string(),
        parameters: vec![]
    }),
    "pre" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.lotus-freelance".to_string(),
        parameters: vec![]
    }),
    "nsf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.lotus-notes".to_string(),
        parameters: vec![]
    }),
    "org" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.lotus-organizer".to_string(),
        parameters: vec![]
    }),
    "scm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.lotus-screencam".to_string(),
        parameters: vec![]
    }),
    "lwp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.lotus-wordpro".to_string(),
        parameters: vec![]
    }),
    "portpkg" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.macports.portpkg".to_string(),
        parameters: vec![]
    }),
    "mcd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mcd".to_string(),
        parameters: vec![]
    }),
    "mc1" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.medcalcdata".to_string(),
        parameters: vec![]
    }),
    "cdkey" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mediastation.cdkey".to_string(),
        parameters: vec![]
    }),
    "mwf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mfer".to_string(),
        parameters: vec![]
    }),
    "mfm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mfmp".to_string(),
        parameters: vec![]
    }),
    "flo" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.micrografx.flo".to_string(),
        parameters: vec![]
    }),
    "igx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.micrografx.igx".to_string(),
        parameters: vec![]
    }),
    "mif" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mif".to_string(),
        parameters: vec![]
    }),
    "daf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mobius.daf".to_string(),
        parameters: vec![]
    }),
    "dis" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mobius.dis".to_string(),
        parameters: vec![]
    }),
    "mbk" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mobius.mbk".to_string(),
        parameters: vec![]
    }),
    "mqy" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mobius.mqy".to_string(),
        parameters: vec![]
    }),
    "msl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mobius.msl".to_string(),
        parameters: vec![]
    }),
    "plc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mobius.plc".to_string(),
        parameters: vec![]
    }),
    "txf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mobius.txf".to_string(),
        parameters: vec![]
    }),
    "mpn" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mophun.application".to_string(),
        parameters: vec![]
    }),
    "mpc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mophun.certificate".to_string(),
        parameters: vec![]
    }),
    "xul" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mozilla.xul+xml".to_string(),
        parameters: vec![]
    }),
    "cil" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-artgalry".to_string(),
        parameters: vec![]
    }),
    "cab" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-cab-compressed".to_string(),
        parameters: vec![]
    }),
    "xls" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-excel".to_string(),
        parameters: vec![]
    }),
    "xlam" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-excel.addin.macroenabled.12".to_string(),
        parameters: vec![]
    }),
    "xlsb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-excel.sheet.binary.macroenabled.12".to_string(),
        parameters: vec![]
    }),
    "xlsm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-excel.sheet.macroenabled.12".to_string(),
        parameters: vec![]
    }),
    "xltm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-excel.template.macroenabled.12".to_string(),
        parameters: vec![]
    }),
    "eot" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-fontobject".to_string(),
        parameters: vec![]
    }),
    "chm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-htmlhelp".to_string(),
        parameters: vec![]
    }),
    "ims" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-ims".to_string(),
        parameters: vec![]
    }),
    "lrm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-lrm".to_string(),
        parameters: vec![]
    }),
    "thmx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-officetheme".to_string(),
        parameters: vec![]
    }),
    "cat" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-pki.seccat".to_string(),
        parameters: vec![]
    }),
    "stl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-pki.stl".to_string(),
        parameters: vec![]
    }),
    "ppt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-powerpoint".to_string(),
        parameters: vec![]
    }),
    "ppam" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-powerpoint.addin.macroenabled.12".to_string(),
        parameters: vec![]
    }),
    "pptm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-powerpoint.presentation.macroenabled.12".to_string(),
        parameters: vec![]
    }),
    "sldm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-powerpoint.slide.macroenabled.12".to_string(),
        parameters: vec![]
    }),
    "ppsm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-powerpoint.slideshow.macroenabled.12".to_string(),
        parameters: vec![]
    }),
    "potm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-powerpoint.template.macroenabled.12".to_string(),
        parameters: vec![]
    }),
    "mpp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-project".to_string(),
        parameters: vec![]
    }),
    "docm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-word.document.macroenabled.12".to_string(),
        parameters: vec![]
    }),
    "dotm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-word.template.macroenabled.12".to_string(),
        parameters: vec![]
    }),
    "wps" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-works".to_string(),
        parameters: vec![]
    }),
    "wpl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-wpl".to_string(),
        parameters: vec![]
    }),
    "xps" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ms-xpsdocument".to_string(),
        parameters: vec![]
    }),
    "mseq" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mseq".to_string(),
        parameters: vec![]
    }),
    "mus" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.musician".to_string(),
        parameters: vec![]
    }),
    "msty" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.muvee.style".to_string(),
        parameters: vec![]
    }),
    "taglet" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.mynfc".to_string(),
        parameters: vec![]
    }),
    "nlu" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.neurolanguage.nlu".to_string(),
        parameters: vec![]
    }),
    "ntf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.nitf".to_string(),
        parameters: vec![]
    }),
    "nnd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.noblenet-directory".to_string(),
        parameters: vec![]
    }),
    "nns" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.noblenet-sealer".to_string(),
        parameters: vec![]
    }),
    "nnw" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.noblenet-web".to_string(),
        parameters: vec![]
    }),
    "ngdat" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.nokia.n-gage.data".to_string(),
        parameters: vec![]
    }),
    "n-gage" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.nokia.n-gage.symbian.install".to_string(),
        parameters: vec![]
    }),
    "rpst" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.nokia.radio-preset".to_string(),
        parameters: vec![]
    }),
    "rpss" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.nokia.radio-presets".to_string(),
        parameters: vec![]
    }),
    "edm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.novadigm.edm".to_string(),
        parameters: vec![]
    }),
    "edx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.novadigm.edx".to_string(),
        parameters: vec![]
    }),
    "ext" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.novadigm.ext".to_string(),
        parameters: vec![]
    }),
    "odc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.chart".to_string(),
        parameters: vec![]
    }),
    "otc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.chart-template".to_string(),
        parameters: vec![]
    }),
    "odb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.database".to_string(),
        parameters: vec![]
    }),
    "odf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.formula".to_string(),
        parameters: vec![]
    }),
    "odft" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.formula-template".to_string(),
        parameters: vec![]
    }),
    "odg" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.graphics".to_string(),
        parameters: vec![]
    }),
    "otg" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.graphics-template".to_string(),
        parameters: vec![]
    }),
    "odi" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.image".to_string(),
        parameters: vec![]
    }),
    "oti" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.image-template".to_string(),
        parameters: vec![]
    }),
    "odp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.presentation".to_string(),
        parameters: vec![]
    }),
    "otp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.presentation-template".to_string(),
        parameters: vec![]
    }),
    "ods" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.spreadsheet".to_string(),
        parameters: vec![]
    }),
    "ots" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.spreadsheet-template".to_string(),
        parameters: vec![]
    }),
    "odt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.text".to_string(),
        parameters: vec![]
    }),
    "odm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.text-master".to_string(),
        parameters: vec![]
    }),
    "ott" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.text-template".to_string(),
        parameters: vec![]
    }),
    "oth" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oasis.opendocument.text-web".to_string(),
        parameters: vec![]
    }),
    "xo" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.olpc-sugar".to_string(),
        parameters: vec![]
    }),
    "dd2" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.oma.dd2+xml".to_string(),
        parameters: vec![]
    }),
    "oxt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.openofficeorg.extension".to_string(),
        parameters: vec![]
    }),
    "pptx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.openxmlformats-officedocument.presentationml.presentation".to_string(),
        parameters: vec![]
    }),
    "sldx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.openxmlformats-officedocument.presentationml.slide".to_string(),
        parameters: vec![]
    }),
    "ppsx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.openxmlformats-officedocument.presentationml.slideshow".to_string(),
        parameters: vec![]
    }),
    "potx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.openxmlformats-officedocument.presentationml.template".to_string(),
        parameters: vec![]
    }),
    "xlsx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
        parameters: vec![]
    }),
    "xltx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.template".to_string(),
        parameters: vec![]
    }),
    "docx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
        parameters: vec![]
    }),
    "dotx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.template".to_string(),
        parameters: vec![]
    }),
    "mgp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.osgeo.mapguide.package".to_string(),
        parameters: vec![]
    }),
    "dp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.osgi.dp".to_string(),
        parameters: vec![]
    }),
    "esa" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.osgi.subsystem".to_string(),
        parameters: vec![]
    }),
    "pdb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.palm".to_string(),
        parameters: vec![]
    }),
    "paw" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.pawaafile".to_string(),
        parameters: vec![]
    }),
    "str" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.pg.format".to_string(),
        parameters: vec![]
    }),
    "ei6" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.pg.osasli".to_string(),
        parameters: vec![]
    }),
    "efif" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.picsel".to_string(),
        parameters: vec![]
    }),
    "wg" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.pmi.widget".to_string(),
        parameters: vec![]
    }),
    "plf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.pocketlearn".to_string(),
        parameters: vec![]
    }),
    "pbd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.powerbuilder6".to_string(),
        parameters: vec![]
    }),
    "box" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.previewsystems.box".to_string(),
        parameters: vec![]
    }),
    "mgz" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.proteus.magazine".to_string(),
        parameters: vec![]
    }),
    "qps" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.publishare-delta-tree".to_string(),
        parameters: vec![]
    }),
    "ptid" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.pvi.ptid1".to_string(),
        parameters: vec![]
    }),
    "qxd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.quark.quarkxpress".to_string(),
        parameters: vec![]
    }),
    "bed" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.realvnc.bed".to_string(),
        parameters: vec![]
    }),
    "mxl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.recordare.musicxml".to_string(),
        parameters: vec![]
    }),
    "musicxml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.recordare.musicxml+xml".to_string(),
        parameters: vec![]
    }),
    "cryptonote" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.rig.cryptonote".to_string(),
        parameters: vec![]
    }),
    "cod" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.rim.cod".to_string(),
        parameters: vec![]
    }),
    "rm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.rn-realmedia".to_string(),
        parameters: vec![]
    }),
    "rmvb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.rn-realmedia-vbr".to_string(),
        parameters: vec![]
    }),
    "link66" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.route66.link66+xml".to_string(),
        parameters: vec![]
    }),
    "st" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.sailingtracker.track".to_string(),
        parameters: vec![]
    }),
    "see" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.seemail".to_string(),
        parameters: vec![]
    }),
    "sema" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.sema".to_string(),
        parameters: vec![]
    }),
    "semd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.semd".to_string(),
        parameters: vec![]
    }),
    "semf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.semf".to_string(),
        parameters: vec![]
    }),
    "ifm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.shana.informed.formdata".to_string(),
        parameters: vec![]
    }),
    "itp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.shana.informed.formtemplate".to_string(),
        parameters: vec![]
    }),
    "iif" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.shana.informed.interchange".to_string(),
        parameters: vec![]
    }),
    "ipk" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.shana.informed.package".to_string(),
        parameters: vec![]
    }),
    "twd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.simtech-mindmapper".to_string(),
        parameters: vec![]
    }),
    "mmf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.smaf".to_string(),
        parameters: vec![]
    }),
    "teacher" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.smart.teacher".to_string(),
        parameters: vec![]
    }),
    "sdkm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.solent.sdkm+xml".to_string(),
        parameters: vec![]
    }),
    "dxp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.spotfire.dxp".to_string(),
        parameters: vec![]
    }),
    "sfs" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.spotfire.sfs".to_string(),
        parameters: vec![]
    }),
    "sdc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.stardivision.calc".to_string(),
        parameters: vec![]
    }),
    "sda" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.stardivision.draw".to_string(),
        parameters: vec![]
    }),
    "sdd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.stardivision.impress".to_string(),
        parameters: vec![]
    }),
    "smf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.stardivision.math".to_string(),
        parameters: vec![]
    }),
    "sdw" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.stardivision.writer".to_string(),
        parameters: vec![]
    }),
    "sgl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.stardivision.writer-global".to_string(),
        parameters: vec![]
    }),
    "smzip" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.stepmania.package".to_string(),
        parameters: vec![]
    }),
    "sm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.stepmania.stepchart".to_string(),
        parameters: vec![]
    }),
    "sxc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.sun.xml.calc".to_string(),
        parameters: vec![]
    }),
    "stc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.sun.xml.calc.template".to_string(),
        parameters: vec![]
    }),
    "sxd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.sun.xml.draw".to_string(),
        parameters: vec![]
    }),
    "std" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.sun.xml.draw.template".to_string(),
        parameters: vec![]
    }),
    "sxi" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.sun.xml.impress".to_string(),
        parameters: vec![]
    }),
    "sti" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.sun.xml.impress.template".to_string(),
        parameters: vec![]
    }),
    "sxm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.sun.xml.math".to_string(),
        parameters: vec![]
    }),
    "sxw" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.sun.xml.writer".to_string(),
        parameters: vec![]
    }),
    "sxg" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.sun.xml.writer.global".to_string(),
        parameters: vec![]
    }),
    "stw" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.sun.xml.writer.template".to_string(),
        parameters: vec![]
    }),
    "sus" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.sus-calendar".to_string(),
        parameters: vec![]
    }),
    "svd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.svd".to_string(),
        parameters: vec![]
    }),
    "sis" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.symbian.install".to_string(),
        parameters: vec![]
    }),
    "xsm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.syncml+xml".to_string(),
        parameters: vec![]
    }),
    "bdm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.syncml.dm+wbxml".to_string(),
        parameters: vec![]
    }),
    "xdm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.syncml.dm+xml".to_string(),
        parameters: vec![]
    }),
    "tao" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.tao.intent-module-archive".to_string(),
        parameters: vec![]
    }),
    "pcap" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.tcpdump.pcap".to_string(),
        parameters: vec![]
    }),
    "tmo" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.tmobile-livetv".to_string(),
        parameters: vec![]
    }),
    "tpt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.trid.tpt".to_string(),
        parameters: vec![]
    }),
    "mxs" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.triscape.mxs".to_string(),
        parameters: vec![]
    }),
    "tra" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.trueapp".to_string(),
        parameters: vec![]
    }),
    "ufd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.ufdl".to_string(),
        parameters: vec![]
    }),
    "utz" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.uiq.theme".to_string(),
        parameters: vec![]
    }),
    "umj" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.umajin".to_string(),
        parameters: vec![]
    }),
    "unityweb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.unity".to_string(),
        parameters: vec![]
    }),
    "uoml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.uoml+xml".to_string(),
        parameters: vec![]
    }),
    "vcx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.vcx".to_string(),
        parameters: vec![]
    }),
    "vsd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.visio".to_string(),
        parameters: vec![]
    }),
    "vis" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.visionary".to_string(),
        parameters: vec![]
    }),
    "vsf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.vsf".to_string(),
        parameters: vec![]
    }),
    "wbxml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.wap.wbxml".to_string(),
        parameters: vec![]
    }),
    "wmlc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.wap.wmlc".to_string(),
        parameters: vec![]
    }),
    "wmlsc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.wap.wmlscriptc".to_string(),
        parameters: vec![]
    }),
    "wtb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.webturbo".to_string(),
        parameters: vec![]
    }),
    "nbp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.wolfram.player".to_string(),
        parameters: vec![]
    }),
    "wpd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.wordperfect".to_string(),
        parameters: vec![]
    }),
    "wqd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.wqd".to_string(),
        parameters: vec![]
    }),
    "stf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.wt.stf".to_string(),
        parameters: vec![]
    }),
    "xar" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.xara".to_string(),
        parameters: vec![]
    }),
    "xfdl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.xfdl".to_string(),
        parameters: vec![]
    }),
    "hvd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.yamaha.hv-dic".to_string(),
        parameters: vec![]
    }),
    "hvs" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.yamaha.hv-script".to_string(),
        parameters: vec![]
    }),
    "hvp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.yamaha.hv-voice".to_string(),
        parameters: vec![]
    }),
    "osf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.yamaha.openscoreformat".to_string(),
        parameters: vec![]
    }),
    "osfpvg" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.yamaha.openscoreformat.osfpvg+xml".to_string(),
        parameters: vec![]
    }),
    "saf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.yamaha.smaf-audio".to_string(),
        parameters: vec![]
    }),
    "spf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.yamaha.smaf-phrase".to_string(),
        parameters: vec![]
    }),
    "cmp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.yellowriver-custom-menu".to_string(),
        parameters: vec![]
    }),
    "zir" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.zul".to_string(),
        parameters: vec![]
    }),
    "zaz" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "vnd.zzazz.deck+xml".to_string(),
        parameters: vec![]
    }),
    "vxml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "voicexml+xml".to_string(),
        parameters: vec![]
    }),
    "wgt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "widget".to_string(),
        parameters: vec![]
    }),
    "hlp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "winhlp".to_string(),
        parameters: vec![]
    }),
    "wsdl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "wsdl+xml".to_string(),
        parameters: vec![]
    }),
    "wspolicy" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "wspolicy+xml".to_string(),
        parameters: vec![]
    }),
    "7z" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-7z-compressed".to_string(),
        parameters: vec![]
    }),
    "abw" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-abiword".to_string(),
        parameters: vec![]
    }),
    "ace" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-ace-compressed".to_string(),
        parameters: vec![]
    }),
    "dmg" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-apple-diskimage".to_string(),
        parameters: vec![]
    }),
    "aab" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-authorware-bin".to_string(),
        parameters: vec![]
    }),
    "aam" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-authorware-map".to_string(),
        parameters: vec![]
    }),
    "aas" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-authorware-seg".to_string(),
        parameters: vec![]
    }),
    "bcpio" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-bcpio".to_string(),
        parameters: vec![]
    }),
    "torrent" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-bittorrent".to_string(),
        parameters: vec![]
    }),
    "blb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-blorb".to_string(),
        parameters: vec![]
    }),
    "bz" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-bzip".to_string(),
        parameters: vec![]
    }),
    "bz2" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-bzip2".to_string(),
        parameters: vec![]
    }),
    "cbr" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-cbr".to_string(),
        parameters: vec![]
    }),
    "vcd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-cdlink".to_string(),
        parameters: vec![]
    }),
    "cfs" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-cfs-compressed".to_string(),
        parameters: vec![]
    }),
    "chat" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-chat".to_string(),
        parameters: vec![]
    }),
    "pgn" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-chess-pgn".to_string(),
        parameters: vec![]
    }),
    "nsc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-conference".to_string(),
        parameters: vec![]
    }),
    "cpio" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-cpio".to_string(),
        parameters: vec![]
    }),
    "csh" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-csh".to_string(),
        parameters: vec![]
    }),
    "deb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-debian-package".to_string(),
        parameters: vec![]
    }),
    "dgc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-dgc-compressed".to_string(),
        parameters: vec![]
    }),
    "dir" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-director".to_string(),
        parameters: vec![]
    }),
    "wad" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-doom".to_string(),
        parameters: vec![]
    }),
    "ncx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-dtbncx+xml".to_string(),
        parameters: vec![]
    }),
    "dtb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-dtbook+xml".to_string(),
        parameters: vec![]
    }),
    "res" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-dtbresource+xml".to_string(),
        parameters: vec![]
    }),
    "dvi" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-dvi".to_string(),
        parameters: vec![]
    }),
    "evy" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-envoy".to_string(),
        parameters: vec![]
    }),
    "eva" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-eva".to_string(),
        parameters: vec![]
    }),
    "bdf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-font-bdf".to_string(),
        parameters: vec![]
    }),
    "gsf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-font-ghostscript".to_string(),
        parameters: vec![]
    }),
    "psf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-font-linux-psf".to_string(),
        parameters: vec![]
    }),
    "otf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-font-otf".to_string(),
        parameters: vec![]
    }),
    "pcf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-font-pcf".to_string(),
        parameters: vec![]
    }),
    "snf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-font-snf".to_string(),
        parameters: vec![]
    }),
    "ttf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-font-ttf".to_string(),
        parameters: vec![]
    }),
    "pfa" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-font-type1".to_string(),
        parameters: vec![]
    }),
    "woff" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "font-woff".to_string(),
        parameters: vec![]
    }),
    "arc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-freearc".to_string(),
        parameters: vec![]
    }),
    "spl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-futuresplash".to_string(),
        parameters: vec![]
    }),
    "gca" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-gca-compressed".to_string(),
        parameters: vec![]
    }),
    "ulx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-glulx".to_string(),
        parameters: vec![]
    }),
    "gnumeric" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-gnumeric".to_string(),
        parameters: vec![]
    }),
    "gramps" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-gramps-xml".to_string(),
        parameters: vec![]
    }),
    "gtar" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-gtar".to_string(),
        parameters: vec![]
    }),
    "hdf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-hdf".to_string(),
        parameters: vec![]
    }),
    "install" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-install-instructions".to_string(),
        parameters: vec![]
    }),
    "iso" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-iso9660-image".to_string(),
        parameters: vec![]
    }),
    "jnlp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-java-jnlp-file".to_string(),
        parameters: vec![]
    }),
    "latex" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-latex".to_string(),
        parameters: vec![]
    }),
    "lzh" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-lzh-compressed".to_string(),
        parameters: vec![]
    }),
    "mie" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-mie".to_string(),
        parameters: vec![]
    }),
    "prc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-mobipocket-ebook".to_string(),
        parameters: vec![]
    }),
    "application" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-ms-application".to_string(),
        parameters: vec![]
    }),
    "lnk" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-ms-shortcut".to_string(),
        parameters: vec![]
    }),
    "wmd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-ms-wmd".to_string(),
        parameters: vec![]
    }),
    "wmz" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-ms-wmz".to_string(),
        parameters: vec![]
    }),
    "xbap" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-ms-xbap".to_string(),
        parameters: vec![]
    }),
    "mdb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-msaccess".to_string(),
        parameters: vec![]
    }),
    "obd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-msbinder".to_string(),
        parameters: vec![]
    }),
    "crd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-mscardfile".to_string(),
        parameters: vec![]
    }),
    "clp" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-msclip".to_string(),
        parameters: vec![]
    }),
    "exe" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-msdownload".to_string(),
        parameters: vec![]
    }),
    "mvb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-msmediaview".to_string(),
        parameters: vec![]
    }),
    "wmf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-msmetafile".to_string(),
        parameters: vec![]
    }),
    "mny" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-msmoney".to_string(),
        parameters: vec![]
    }),
    "pub" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-mspublisher".to_string(),
        parameters: vec![]
    }),
    "scd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-msschedule".to_string(),
        parameters: vec![]
    }),
    "trm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-msterminal".to_string(),
        parameters: vec![]
    }),
    "wri" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-mswrite".to_string(),
        parameters: vec![]
    }),
    "nc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-netcdf".to_string(),
        parameters: vec![]
    }),
    "nzb" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-nzb".to_string(),
        parameters: vec![]
    }),
    "p12" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-pkcs12".to_string(),
        parameters: vec![]
    }),
    "p7b" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-pkcs7-certificates".to_string(),
        parameters: vec![]
    }),
    "p7r" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-pkcs7-certreqresp".to_string(),
        parameters: vec![]
    }),
    "rar" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-rar-compressed".to_string(),
        parameters: vec![]
    }),
    "ris" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-research-info-systems".to_string(),
        parameters: vec![]
    }),
    "sh" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-sh".to_string(),
        parameters: vec![]
    }),
    "shar" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-shar".to_string(),
        parameters: vec![]
    }),
    "swf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-shockwave-flash".to_string(),
        parameters: vec![]
    }),
    "xap" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-silverlight-app".to_string(),
        parameters: vec![]
    }),
    "sql" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-sql".to_string(),
        parameters: vec![]
    }),
    "sit" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-stuffit".to_string(),
        parameters: vec![]
    }),
    "sitx" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-stuffitx".to_string(),
        parameters: vec![]
    }),
    "srt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-subrip".to_string(),
        parameters: vec![]
    }),
    "sv4cpio" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-sv4cpio".to_string(),
        parameters: vec![]
    }),
    "sv4crc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-sv4crc".to_string(),
        parameters: vec![]
    }),
    "t3" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-t3vm-image".to_string(),
        parameters: vec![]
    }),
    "gam" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-tads".to_string(),
        parameters: vec![]
    }),
    "tar" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-tar".to_string(),
        parameters: vec![]
    }),
    "tcl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-tcl".to_string(),
        parameters: vec![]
    }),
    "tex" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-tex".to_string(),
        parameters: vec![]
    }),
    "tfm" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-tex-tfm".to_string(),
        parameters: vec![]
    }),
    "texinfo" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-texinfo".to_string(),
        parameters: vec![]
    }),
    "obj" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-tgif".to_string(),
        parameters: vec![]
    }),
    "ustar" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-ustar".to_string(),
        parameters: vec![]
    }),
    "src" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-wais-source".to_string(),
        parameters: vec![]
    }),
    "der" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-x509-ca-cert".to_string(),
        parameters: vec![]
    }),
    "fig" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-xfig".to_string(),
        parameters: vec![]
    }),
    "xlf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-xliff+xml".to_string(),
        parameters: vec![]
    }),
    "xpi" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-xpinstall".to_string(),
        parameters: vec![]
    }),
    "xz" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-xz".to_string(),
        parameters: vec![]
    }),
    "z1" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "x-zmachine".to_string(),
        parameters: vec![]
    }),
    "xaml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "xaml+xml".to_string(),
        parameters: vec![]
    }),
    "xdf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "xcap-diff+xml".to_string(),
        parameters: vec![]
    }),
    "xenc" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "xenc+xml".to_string(),
        parameters: vec![]
    }),
    "xhtml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "xhtml+xml".to_string(),
        parameters: vec![]
    }),
    "xml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "xml".to_string(),
        parameters: vec![]
    }),
    "dtd" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "xml-dtd".to_string(),
        parameters: vec![]
    }),
    "xop" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "xop+xml".to_string(),
        parameters: vec![]
    }),
    "xpl" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "xproc+xml".to_string(),
        parameters: vec![]
    }),
    "xslt" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "xslt+xml".to_string(),
        parameters: vec![]
    }),
    "xspf" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "xspf+xml".to_string(),
        parameters: vec![]
    }),
    "mxml" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "xv+xml".to_string(),
        parameters: vec![]
    }),
    "yang" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "yang".to_string(),
        parameters: vec![]
    }),
    "yin" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "yin+xml".to_string(),
        parameters: vec![]
    }),
    "zip" => Some(MediaType {
        type_: "application".to_string(),
        subtype: "zip".to_string(),
        parameters: vec![]
    }),
    "adp" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "adpcm".to_string(),
        parameters: vec![]
    }),
    "au" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "basic".to_string(),
        parameters: vec![]
    }),
    "mid" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "midi".to_string(),
        parameters: vec![]
    }),
    "mp4a" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "mp4".to_string(),
        parameters: vec![]
    }),
    "mpga" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "mpeg".to_string(),
        parameters: vec![]
    }),
    "oga" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "ogg".to_string(),
        parameters: vec![]
    }),
    "s3m" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "s3m".to_string(),
        parameters: vec![]
    }),
    "sil" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "silk".to_string(),
        parameters: vec![]
    }),
    "uva" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "vnd.dece.audio".to_string(),
        parameters: vec![]
    }),
    "eol" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "vnd.digital-winds".to_string(),
        parameters: vec![]
    }),
    "dra" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "vnd.dra".to_string(),
        parameters: vec![]
    }),
    "dts" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "vnd.dts".to_string(),
        parameters: vec![]
    }),
    "dtshd" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "vnd.dts.hd".to_string(),
        parameters: vec![]
    }),
    "lvp" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "vnd.lucent.voice".to_string(),
        parameters: vec![]
    }),
    "pya" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "vnd.ms-playready.media.pya".to_string(),
        parameters: vec![]
    }),
    "ecelp4800" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "vnd.nuera.ecelp4800".to_string(),
        parameters: vec![]
    }),
    "ecelp7470" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "vnd.nuera.ecelp7470".to_string(),
        parameters: vec![]
    }),
    "ecelp9600" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "vnd.nuera.ecelp9600".to_string(),
        parameters: vec![]
    }),
    "rip" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "vnd.rip".to_string(),
        parameters: vec![]
    }),
    "weba" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "webm".to_string(),
        parameters: vec![]
    }),
    "aac" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "x-aac".to_string(),
        parameters: vec![]
    }),
    "aif" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "x-aiff".to_string(),
        parameters: vec![]
    }),
    "caf" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "x-caf".to_string(),
        parameters: vec![]
    }),
    "flac" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "x-flac".to_string(),
        parameters: vec![]
    }),
    "mka" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "x-matroska".to_string(),
        parameters: vec![]
    }),
    "m3u" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "x-mpegurl".to_string(),
        parameters: vec![]
    }),
    "wax" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "x-ms-wax".to_string(),
        parameters: vec![]
    }),
    "wma" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "x-ms-wma".to_string(),
        parameters: vec![]
    }),
    "ram" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "x-pn-realaudio".to_string(),
        parameters: vec![]
    }),
    "rmp" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "x-pn-realaudio-plugin".to_string(),
        parameters: vec![]
    }),
    "wav" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "x-wav".to_string(),
        parameters: vec![]
    }),
    "xm" => Some(MediaType {
        type_: "audio".to_string(),
        subtype: "xm".to_string(),
        parameters: vec![]
    }),
    "cdx" => Some(MediaType {
        type_: "chemical".to_string(),
        subtype: "x-cdx".to_string(),
        parameters: vec![]
    }),
    "cif" => Some(MediaType {
        type_: "chemical".to_string(),
        subtype: "x-cif".to_string(),
        parameters: vec![]
    }),
    "cmdf" => Some(MediaType {
        type_: "chemical".to_string(),
        subtype: "x-cmdf".to_string(),
        parameters: vec![]
    }),
    "cml" => Some(MediaType {
        type_: "chemical".to_string(),
        subtype: "x-cml".to_string(),
        parameters: vec![]
    }),
    "csml" => Some(MediaType {
        type_: "chemical".to_string(),
        subtype: "x-csml".to_string(),
        parameters: vec![]
    }),
    "xyz" => Some(MediaType {
        type_: "chemical".to_string(),
        subtype: "x-xyz".to_string(),
        parameters: vec![]
    }),
    "bmp" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "bmp".to_string(),
        parameters: vec![]
    }),
    "cgm" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "cgm".to_string(),
        parameters: vec![]
    }),
    "g3" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "g3fax".to_string(),
        parameters: vec![]
    }),
    "gif" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "gif".to_string(),
        parameters: vec![]
    }),
    "ief" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "ief".to_string(),
        parameters: vec![]
    }),
    "jpeg" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "jpeg".to_string(),
        parameters: vec![]
    }),
    "ktx" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "ktx".to_string(),
        parameters: vec![]
    }),
    "png" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "png".to_string(),
        parameters: vec![]
    }),
    "btif" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "prs.btif".to_string(),
        parameters: vec![]
    }),
    "sgi" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "sgi".to_string(),
        parameters: vec![]
    }),
    "svg" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "svg+xml".to_string(),
        parameters: vec![]
    }),
    "tiff" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "tiff".to_string(),
        parameters: vec![]
    }),
    "psd" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.adobe.photoshop".to_string(),
        parameters: vec![]
    }),
    "uvi" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.dece.graphic".to_string(),
        parameters: vec![]
    }),
    "sub" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.dvb.subtitle".to_string(),
        parameters: vec![]
    }),
    "djvu" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.djvu".to_string(),
        parameters: vec![]
    }),
    "dwg" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.dwg".to_string(),
        parameters: vec![]
    }),
    "dxf" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.dxf".to_string(),
        parameters: vec![]
    }),
    "fbs" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.fastbidsheet".to_string(),
        parameters: vec![]
    }),
    "fpx" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.fpx".to_string(),
        parameters: vec![]
    }),
    "fst" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.fst".to_string(),
        parameters: vec![]
    }),
    "mmr" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.fujixerox.edmics-mmr".to_string(),
        parameters: vec![]
    }),
    "rlc" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.fujixerox.edmics-rlc".to_string(),
        parameters: vec![]
    }),
    "mdi" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.ms-modi".to_string(),
        parameters: vec![]
    }),
    "wdp" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.ms-photo".to_string(),
        parameters: vec![]
    }),
    "npx" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.net-fpx".to_string(),
        parameters: vec![]
    }),
    "wbmp" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.wap.wbmp".to_string(),
        parameters: vec![]
    }),
    "xif" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "vnd.xiff".to_string(),
        parameters: vec![]
    }),
    "webp" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "webp".to_string(),
        parameters: vec![]
    }),
    "3ds" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-3ds".to_string(),
        parameters: vec![]
    }),
    "ras" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-cmu-raster".to_string(),
        parameters: vec![]
    }),
    "cmx" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-cmx".to_string(),
        parameters: vec![]
    }),
    "fh" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-freehand".to_string(),
        parameters: vec![]
    }),
    "ico" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-icon".to_string(),
        parameters: vec![]
    }),
    "sid" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-mrsid-image".to_string(),
        parameters: vec![]
    }),
    "pcx" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-pcx".to_string(),
        parameters: vec![]
    }),
    "pic" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-pict".to_string(),
        parameters: vec![]
    }),
    "pnm" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-portable-anymap".to_string(),
        parameters: vec![]
    }),
    "pbm" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-portable-bitmap".to_string(),
        parameters: vec![]
    }),
    "pgm" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-portable-graymap".to_string(),
        parameters: vec![]
    }),
    "ppm" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-portable-pixmap".to_string(),
        parameters: vec![]
    }),
    "rgb" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-rgb".to_string(),
        parameters: vec![]
    }),
    "tga" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-tga".to_string(),
        parameters: vec![]
    }),
    "xbm" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-xbitmap".to_string(),
        parameters: vec![]
    }),
    "xpm" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-xpixmap".to_string(),
        parameters: vec![]
    }),
    "xwd" => Some(MediaType {
        type_: "image".to_string(),
        subtype: "x-xwindowdump".to_string(),
        parameters: vec![]
    }),
    "eml" => Some(MediaType {
        type_: "message".to_string(),
        subtype: "rfc822".to_string(),
        parameters: vec![]
    }),
    "igs" => Some(MediaType {
        type_: "model".to_string(),
        subtype: "iges".to_string(),
        parameters: vec![]
    }),
    "msh" => Some(MediaType {
        type_: "model".to_string(),
        subtype: "mesh".to_string(),
        parameters: vec![]
    }),
    "dae" => Some(MediaType {
        type_: "model".to_string(),
        subtype: "vnd.collada+xml".to_string(),
        parameters: vec![]
    }),
    "dwf" => Some(MediaType {
        type_: "model".to_string(),
        subtype: "vnd.dwf".to_string(),
        parameters: vec![]
    }),
    "gdl" => Some(MediaType {
        type_: "model".to_string(),
        subtype: "vnd.gdl".to_string(),
        parameters: vec![]
    }),
    "gtw" => Some(MediaType {
        type_: "model".to_string(),
        subtype: "vnd.gtw".to_string(),
        parameters: vec![]
    }),
    "mts" => Some(MediaType {
        type_: "model".to_string(),
        subtype: "vnd.mts".to_string(),
        parameters: vec![]
    }),
    "vtu" => Some(MediaType {
        type_: "model".to_string(),
        subtype: "vnd.vtu".to_string(),
        parameters: vec![]
    }),
    "wrl" => Some(MediaType {
        type_: "model".to_string(),
        subtype: "vrml".to_string(),
        parameters: vec![]
    }),
    "x3db" => Some(MediaType {
        type_: "model".to_string(),
        subtype: "x3d+binary".to_string(),
        parameters: vec![]
    }),
    "x3dv" => Some(MediaType {
        type_: "model".to_string(),
        subtype: "x3d+vrml".to_string(),
        parameters: vec![]
    }),
    "x3d" => Some(MediaType {
        type_: "model".to_string(),
        subtype: "x3d+xml".to_string(),
        parameters: vec![]
    }),
    "appcache" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "cache-manifest".to_string(),
        parameters: vec![]
    }),
    "ics" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "calendar".to_string(),
        parameters: vec![]
    }),
    "css" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "css".to_string(),
        parameters: vec![]
    }),
    "csv" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "csv".to_string(),
        parameters: vec![]
    }),
    "html" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "html".to_string(),
        parameters: vec![]
    }),
    "n3" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "n3".to_string(),
        parameters: vec![]
    }),
    "txt" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "plain".to_string(),
        parameters: vec![]
    }),
    "dsc" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "prs.lines.tag".to_string(),
        parameters: vec![]
    }),
    "rtx" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "richtext".to_string(),
        parameters: vec![]
    }),
    "sgml" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "sgml".to_string(),
        parameters: vec![]
    }),
    "tsv" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "tab-separated-values".to_string(),
        parameters: vec![]
    }),
    "t" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "troff".to_string(),
        parameters: vec![]
    }),
    "ttl" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "turtle".to_string(),
        parameters: vec![]
    }),
    "uri" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "uri-list".to_string(),
        parameters: vec![]
    }),
    "vcard" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "vcard".to_string(),
        parameters: vec![]
    }),
    "curl" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "vnd.curl".to_string(),
        parameters: vec![]
    }),
    "dcurl" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "vnd.curl.dcurl".to_string(),
        parameters: vec![]
    }),
    "scurl" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "vnd.curl.scurl".to_string(),
        parameters: vec![]
    }),
    "mcurl" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "vnd.curl.mcurl".to_string(),
        parameters: vec![]
    }),
    "fly" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "vnd.fly".to_string(),
        parameters: vec![]
    }),
    "flx" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "vnd.fmi.flexstor".to_string(),
        parameters: vec![]
    }),
    "gv" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "vnd.graphviz".to_string(),
        parameters: vec![]
    }),
    "3dml" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "vnd.in3d.3dml".to_string(),
        parameters: vec![]
    }),
    "spot" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "vnd.in3d.spot".to_string(),
        parameters: vec![]
    }),
    "jad" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "vnd.sun.j2me.app-descriptor".to_string(),
        parameters: vec![]
    }),
    "wml" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "vnd.wap.wml".to_string(),
        parameters: vec![]
    }),
    "wmls" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "vnd.wap.wmlscript".to_string(),
        parameters: vec![]
    }),
    "s" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "x-asm".to_string(),
        parameters: vec![]
    }),
    "c" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "x-c".to_string(),
        parameters: vec![]
    }),
    "f" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "x-fortran".to_string(),
        parameters: vec![]
    }),
    "java" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "x-java-source".to_string(),
        parameters: vec![]
    }),
    "opml" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "x-opml".to_string(),
        parameters: vec![]
    }),
    "p" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "x-pascal".to_string(),
        parameters: vec![]
    }),
    "nfo" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "x-nfo".to_string(),
        parameters: vec![]
    }),
    "etx" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "x-setext".to_string(),
        parameters: vec![]
    }),
    "sfv" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "x-sfv".to_string(),
        parameters: vec![]
    }),
    "uu" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "x-uuencode".to_string(),
        parameters: vec![]
    }),
    "vcs" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "x-vcalendar".to_string(),
        parameters: vec![]
    }),
    "vcf" => Some(MediaType {
        type_: "text".to_string(),
        subtype: "x-vcard".to_string(),
        parameters: vec![]
    }),
    "3gp" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "3gpp".to_string(),
        parameters: vec![]
    }),
    "3g2" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "3gpp2".to_string(),
        parameters: vec![]
    }),
    "h261" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "h261".to_string(),
        parameters: vec![]
    }),
    "h263" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "h263".to_string(),
        parameters: vec![]
    }),
    "h264" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "h264".to_string(),
        parameters: vec![]
    }),
    "jpgv" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "jpeg".to_string(),
        parameters: vec![]
    }),
    "jpm" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "jpm".to_string(),
        parameters: vec![]
    }),
    "mj2" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "mj2".to_string(),
        parameters: vec![]
    }),
    "mp4" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "mp4".to_string(),
        parameters: vec![]
    }),
    "mpeg" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "mpeg".to_string(),
        parameters: vec![]
    }),
    "ogv" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "ogg".to_string(),
        parameters: vec![]
    }),
    "qt" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "quicktime".to_string(),
        parameters: vec![]
    }),
    "uvh" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "vnd.dece.hd".to_string(),
        parameters: vec![]
    }),
    "uvm" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "vnd.dece.mobile".to_string(),
        parameters: vec![]
    }),
    "uvp" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "vnd.dece.pd".to_string(),
        parameters: vec![]
    }),
    "uvs" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "vnd.dece.sd".to_string(),
        parameters: vec![]
    }),
    "uvv" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "vnd.dece.video".to_string(),
        parameters: vec![]
    }),
    "dvb" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "vnd.dvb.file".to_string(),
        parameters: vec![]
    }),
    "fvt" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "vnd.fvt".to_string(),
        parameters: vec![]
    }),
    "mxu" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "vnd.mpegurl".to_string(),
        parameters: vec![]
    }),
    "pyv" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "vnd.ms-playready.media.pyv".to_string(),
        parameters: vec![]
    }),
    "uvu" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "vnd.uvvu.mp4".to_string(),
        parameters: vec![]
    }),
    "viv" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "vnd.vivo".to_string(),
        parameters: vec![]
    }),
    "webm" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "webm".to_string(),
        parameters: vec![]
    }),
    "f4v" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-f4v".to_string(),
        parameters: vec![]
    }),
    "fli" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-fli".to_string(),
        parameters: vec![]
    }),
    "flv" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-flv".to_string(),
        parameters: vec![]
    }),
    "m4v" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-m4v".to_string(),
        parameters: vec![]
    }),
    "mkv" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-matroska".to_string(),
        parameters: vec![]
    }),
    "mng" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-mng".to_string(),
        parameters: vec![]
    }),
    "asf" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-ms-asf".to_string(),
        parameters: vec![]
    }),
    "vob" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-ms-vob".to_string(),
        parameters: vec![]
    }),
    "wm" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-ms-wm".to_string(),
        parameters: vec![]
    }),
    "wmv" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-ms-wmv".to_string(),
        parameters: vec![]
    }),
    "wmx" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-ms-wmx".to_string(),
        parameters: vec![]
    }),
    "wvx" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-ms-wvx".to_string(),
        parameters: vec![]
    }),
    "avi" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-msvideo".to_string(),
        parameters: vec![]
    }),
    "movie" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-sgi-movie".to_string(),
        parameters: vec![]
    }),
    "smv" => Some(MediaType {
        type_: "video".to_string(),
        subtype: "x-smv".to_string(),
        parameters: vec![]
    }),
    "ice" => Some(MediaType {
        type_: "x-conference".to_string(),
        subtype: "x-cooltalk".to_string(),
        parameters: vec![]
    }),
        _ => None
    }
}
