use http::headers::content_type::MediaType;

pub fn get_media_type(ext: &str) -> Option<MediaType> {
    match ext {
    "ez" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "andrew-inset".to_str(),
        parameters: vec![]
    }),
    "aw" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "applixware".to_str(),
        parameters: vec![]
    }),
    "atom" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "atom+xml".to_str(),
        parameters: vec![]
    }),
    "atomcat" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "atomcat+xml".to_str(),
        parameters: vec![]
    }),
    "atomsvc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "atomsvc+xml".to_str(),
        parameters: vec![]
    }),
    "ccxml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "ccxml+xml".to_str(),
        parameters: vec![]
    }),
    "cdmia" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "cdmi-capability".to_str(),
        parameters: vec![]
    }),
    "cdmic" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "cdmi-container".to_str(),
        parameters: vec![]
    }),
    "cdmid" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "cdmi-domain".to_str(),
        parameters: vec![]
    }),
    "cdmio" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "cdmi-object".to_str(),
        parameters: vec![]
    }),
    "cdmiq" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "cdmi-queue".to_str(),
        parameters: vec![]
    }),
    "cu" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "cu-seeme".to_str(),
        parameters: vec![]
    }),
    "davmount" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "davmount+xml".to_str(),
        parameters: vec![]
    }),
    "dbk" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "docbook+xml".to_str(),
        parameters: vec![]
    }),
    "dssc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "dssc+der".to_str(),
        parameters: vec![]
    }),
    "xdssc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "dssc+xml".to_str(),
        parameters: vec![]
    }),
    "ecma" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "ecmascript".to_str(),
        parameters: vec![]
    }),
    "emma" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "emma+xml".to_str(),
        parameters: vec![]
    }),
    "epub" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "epub+zip".to_str(),
        parameters: vec![]
    }),
    "exi" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "exi".to_str(),
        parameters: vec![]
    }),
    "pfr" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "font-tdpfr".to_str(),
        parameters: vec![]
    }),
    "gml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "gml+xml".to_str(),
        parameters: vec![]
    }),
    "gpx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "gpx+xml".to_str(),
        parameters: vec![]
    }),
    "gxf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "gxf".to_str(),
        parameters: vec![]
    }),
    "stk" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "hyperstudio".to_str(),
        parameters: vec![]
    }),
    "ink" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "inkml+xml".to_str(),
        parameters: vec![]
    }),
    "ipfix" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "ipfix".to_str(),
        parameters: vec![]
    }),
    "jar" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "java-archive".to_str(),
        parameters: vec![]
    }),
    "ser" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "java-serialized-object".to_str(),
        parameters: vec![]
    }),
    "class" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "java-vm".to_str(),
        parameters: vec![]
    }),
    "js" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "javascript".to_str(),
        parameters: vec![]
    }),
    "json" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "json".to_str(),
        parameters: vec![]
    }),
    "jsonml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "jsonml+json".to_str(),
        parameters: vec![]
    }),
    "lostxml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "lost+xml".to_str(),
        parameters: vec![]
    }),
    "hqx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "mac-binhex40".to_str(),
        parameters: vec![]
    }),
    "cpt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "mac-compactpro".to_str(),
        parameters: vec![]
    }),
    "mads" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "mads+xml".to_str(),
        parameters: vec![]
    }),
    "mrc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "marc".to_str(),
        parameters: vec![]
    }),
    "mrcx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "marcxml+xml".to_str(),
        parameters: vec![]
    }),
    "ma" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "mathematica".to_str(),
        parameters: vec![]
    }),
    "mathml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "mathml+xml".to_str(),
        parameters: vec![]
    }),
    "mbox" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "mbox".to_str(),
        parameters: vec![]
    }),
    "mscml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "mediaservercontrol+xml".to_str(),
        parameters: vec![]
    }),
    "metalink" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "metalink+xml".to_str(),
        parameters: vec![]
    }),
    "meta4" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "metalink4+xml".to_str(),
        parameters: vec![]
    }),
    "mets" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "mets+xml".to_str(),
        parameters: vec![]
    }),
    "mods" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "mods+xml".to_str(),
        parameters: vec![]
    }),
    "m21" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "mp21".to_str(),
        parameters: vec![]
    }),
    "mp4s" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "mp4".to_str(),
        parameters: vec![]
    }),
    "doc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "msword".to_str(),
        parameters: vec![]
    }),
    "mxf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "mxf".to_str(),
        parameters: vec![]
    }),
    "bin" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "octet-stream".to_str(),
        parameters: vec![]
    }),
    "oda" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "oda".to_str(),
        parameters: vec![]
    }),
    "opf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "oebps-package+xml".to_str(),
        parameters: vec![]
    }),
    "ogx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "ogg".to_str(),
        parameters: vec![]
    }),
    "omdoc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "omdoc+xml".to_str(),
        parameters: vec![]
    }),
    "onetoc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "onenote".to_str(),
        parameters: vec![]
    }),
    "oxps" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "oxps".to_str(),
        parameters: vec![]
    }),
    "xer" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "patch-ops-error+xml".to_str(),
        parameters: vec![]
    }),
    "pdf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pdf".to_str(),
        parameters: vec![]
    }),
    "pgp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pgp-encrypted".to_str(),
        parameters: vec![]
    }),
    "asc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pgp-signature".to_str(),
        parameters: vec![]
    }),
    "prf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pics-rules".to_str(),
        parameters: vec![]
    }),
    "p10" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pkcs10".to_str(),
        parameters: vec![]
    }),
    "p7m" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pkcs7-mime".to_str(),
        parameters: vec![]
    }),
    "p7s" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pkcs7-signature".to_str(),
        parameters: vec![]
    }),
    "p8" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pkcs8".to_str(),
        parameters: vec![]
    }),
    "ac" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pkix-attr-cert".to_str(),
        parameters: vec![]
    }),
    "cer" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pkix-cert".to_str(),
        parameters: vec![]
    }),
    "crl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pkix-crl".to_str(),
        parameters: vec![]
    }),
    "pkipath" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pkix-pkipath".to_str(),
        parameters: vec![]
    }),
    "pki" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pkixcmp".to_str(),
        parameters: vec![]
    }),
    "pls" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pls+xml".to_str(),
        parameters: vec![]
    }),
    "ai" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "postscript".to_str(),
        parameters: vec![]
    }),
    "cww" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "prs.cww".to_str(),
        parameters: vec![]
    }),
    "pskcxml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "pskc+xml".to_str(),
        parameters: vec![]
    }),
    "rdf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "rdf+xml".to_str(),
        parameters: vec![]
    }),
    "rif" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "reginfo+xml".to_str(),
        parameters: vec![]
    }),
    "rnc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "relax-ng-compact-syntax".to_str(),
        parameters: vec![]
    }),
    "rl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "resource-lists+xml".to_str(),
        parameters: vec![]
    }),
    "rld" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "resource-lists-diff+xml".to_str(),
        parameters: vec![]
    }),
    "rs" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "rls-services+xml".to_str(),
        parameters: vec![]
    }),
    "gbr" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "rpki-ghostbusters".to_str(),
        parameters: vec![]
    }),
    "mft" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "rpki-manifest".to_str(),
        parameters: vec![]
    }),
    "roa" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "rpki-roa".to_str(),
        parameters: vec![]
    }),
    "rsd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "rsd+xml".to_str(),
        parameters: vec![]
    }),
    "rss" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "rss+xml".to_str(),
        parameters: vec![]
    }),
    "rtf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "rtf".to_str(),
        parameters: vec![]
    }),
    "sbml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "sbml+xml".to_str(),
        parameters: vec![]
    }),
    "scq" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "scvp-cv-request".to_str(),
        parameters: vec![]
    }),
    "scs" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "scvp-cv-response".to_str(),
        parameters: vec![]
    }),
    "spq" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "scvp-vp-request".to_str(),
        parameters: vec![]
    }),
    "spp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "scvp-vp-response".to_str(),
        parameters: vec![]
    }),
    "sdp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "sdp".to_str(),
        parameters: vec![]
    }),
    "setpay" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "set-payment-initiation".to_str(),
        parameters: vec![]
    }),
    "setreg" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "set-registration-initiation".to_str(),
        parameters: vec![]
    }),
    "shf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "shf+xml".to_str(),
        parameters: vec![]
    }),
    "smi" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "smil+xml".to_str(),
        parameters: vec![]
    }),
    "rq" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "sparql-query".to_str(),
        parameters: vec![]
    }),
    "srx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "sparql-results+xml".to_str(),
        parameters: vec![]
    }),
    "gram" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "srgs".to_str(),
        parameters: vec![]
    }),
    "grxml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "srgs+xml".to_str(),
        parameters: vec![]
    }),
    "sru" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "sru+xml".to_str(),
        parameters: vec![]
    }),
    "ssdl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "ssdl+xml".to_str(),
        parameters: vec![]
    }),
    "ssml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "ssml+xml".to_str(),
        parameters: vec![]
    }),
    "tei" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "tei+xml".to_str(),
        parameters: vec![]
    }),
    "tfi" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "thraud+xml".to_str(),
        parameters: vec![]
    }),
    "tsd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "timestamped-data".to_str(),
        parameters: vec![]
    }),
    "plb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.3gpp.pic-bw-large".to_str(),
        parameters: vec![]
    }),
    "psb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.3gpp.pic-bw-small".to_str(),
        parameters: vec![]
    }),
    "pvb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.3gpp.pic-bw-var".to_str(),
        parameters: vec![]
    }),
    "tcap" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.3gpp2.tcap".to_str(),
        parameters: vec![]
    }),
    "pwn" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.3m.post-it-notes".to_str(),
        parameters: vec![]
    }),
    "aso" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.accpac.simply.aso".to_str(),
        parameters: vec![]
    }),
    "imp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.accpac.simply.imp".to_str(),
        parameters: vec![]
    }),
    "acu" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.acucobol".to_str(),
        parameters: vec![]
    }),
    "atc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.acucorp".to_str(),
        parameters: vec![]
    }),
    "air" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.adobe.air-application-installer-package+zip".to_str(),
        parameters: vec![]
    }),
    "fcdt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.adobe.formscentral.fcdt".to_str(),
        parameters: vec![]
    }),
    "fxp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.adobe.fxp".to_str(),
        parameters: vec![]
    }),
    "xdp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.adobe.xdp+xml".to_str(),
        parameters: vec![]
    }),
    "xfdf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.adobe.xfdf".to_str(),
        parameters: vec![]
    }),
    "ahead" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ahead.space".to_str(),
        parameters: vec![]
    }),
    "azf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.airzip.filesecure.azf".to_str(),
        parameters: vec![]
    }),
    "azs" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.airzip.filesecure.azs".to_str(),
        parameters: vec![]
    }),
    "azw" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.amazon.ebook".to_str(),
        parameters: vec![]
    }),
    "acc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.americandynamics.acc".to_str(),
        parameters: vec![]
    }),
    "ami" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.amiga.ami".to_str(),
        parameters: vec![]
    }),
    "apk" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.android.package-archive".to_str(),
        parameters: vec![]
    }),
    "cii" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.anser-web-certificate-issue-initiation".to_str(),
        parameters: vec![]
    }),
    "fti" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.anser-web-funds-transfer-initiation".to_str(),
        parameters: vec![]
    }),
    "atx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.antix.game-component".to_str(),
        parameters: vec![]
    }),
    "mpkg" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.apple.installer+xml".to_str(),
        parameters: vec![]
    }),
    "m3u8" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.apple.mpegurl".to_str(),
        parameters: vec![]
    }),
    "swi" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.aristanetworks.swi".to_str(),
        parameters: vec![]
    }),
    "iota" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.astraea-software.iota".to_str(),
        parameters: vec![]
    }),
    "aep" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.audiograph".to_str(),
        parameters: vec![]
    }),
    "mpm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.blueice.multipass".to_str(),
        parameters: vec![]
    }),
    "bmi" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.bmi".to_str(),
        parameters: vec![]
    }),
    "rep" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.businessobjects".to_str(),
        parameters: vec![]
    }),
    "cdxml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.chemdraw+xml".to_str(),
        parameters: vec![]
    }),
    "mmd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.chipnuts.karaoke-mmd".to_str(),
        parameters: vec![]
    }),
    "cdy" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.cinderella".to_str(),
        parameters: vec![]
    }),
    "cla" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.claymore".to_str(),
        parameters: vec![]
    }),
    "rp9" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.cloanto.rp9".to_str(),
        parameters: vec![]
    }),
    "c4g" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.clonk.c4group".to_str(),
        parameters: vec![]
    }),
    "c11amc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.cluetrust.cartomobile-config".to_str(),
        parameters: vec![]
    }),
    "c11amz" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.cluetrust.cartomobile-config-pkg".to_str(),
        parameters: vec![]
    }),
    "csp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.commonspace".to_str(),
        parameters: vec![]
    }),
    "cdbcmsg" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.contact.cmsg".to_str(),
        parameters: vec![]
    }),
    "cmc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.cosmocaller".to_str(),
        parameters: vec![]
    }),
    "clkx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.crick.clicker".to_str(),
        parameters: vec![]
    }),
    "clkk" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.crick.clicker.keyboard".to_str(),
        parameters: vec![]
    }),
    "clkp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.crick.clicker.palette".to_str(),
        parameters: vec![]
    }),
    "clkt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.crick.clicker.template".to_str(),
        parameters: vec![]
    }),
    "clkw" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.crick.clicker.wordbank".to_str(),
        parameters: vec![]
    }),
    "wbs" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.criticaltools.wbs+xml".to_str(),
        parameters: vec![]
    }),
    "pml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ctc-posml".to_str(),
        parameters: vec![]
    }),
    "ppd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.cups-ppd".to_str(),
        parameters: vec![]
    }),
    "car" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.curl.car".to_str(),
        parameters: vec![]
    }),
    "pcurl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.curl.pcurl".to_str(),
        parameters: vec![]
    }),
    "dart" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.dart".to_str(),
        parameters: vec![]
    }),
    "rdz" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.data-vision.rdz".to_str(),
        parameters: vec![]
    }),
    "uvf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.dece.data".to_str(),
        parameters: vec![]
    }),
    "uvt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.dece.ttml+xml".to_str(),
        parameters: vec![]
    }),
    "uvx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.dece.unspecified".to_str(),
        parameters: vec![]
    }),
    "uvz" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.dece.zip".to_str(),
        parameters: vec![]
    }),
    "fe_launch" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.denovo.fcselayout-link".to_str(),
        parameters: vec![]
    }),
    "dna" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.dna".to_str(),
        parameters: vec![]
    }),
    "mlp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.dolby.mlp".to_str(),
        parameters: vec![]
    }),
    "dpg" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.dpgraph".to_str(),
        parameters: vec![]
    }),
    "dfac" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.dreamfactory".to_str(),
        parameters: vec![]
    }),
    "kpxx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ds-keypoint".to_str(),
        parameters: vec![]
    }),
    "ait" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.dvb.ait".to_str(),
        parameters: vec![]
    }),
    "svc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.dvb.service".to_str(),
        parameters: vec![]
    }),
    "geo" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.dynageo".to_str(),
        parameters: vec![]
    }),
    "mag" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ecowin.chart".to_str(),
        parameters: vec![]
    }),
    "nml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.enliven".to_str(),
        parameters: vec![]
    }),
    "esf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.epson.esf".to_str(),
        parameters: vec![]
    }),
    "msf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.epson.msf".to_str(),
        parameters: vec![]
    }),
    "qam" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.epson.quickanime".to_str(),
        parameters: vec![]
    }),
    "slt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.epson.salt".to_str(),
        parameters: vec![]
    }),
    "ssf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.epson.ssf".to_str(),
        parameters: vec![]
    }),
    "es3" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.eszigno3+xml".to_str(),
        parameters: vec![]
    }),
    "ez2" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ezpix-album".to_str(),
        parameters: vec![]
    }),
    "ez3" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ezpix-package".to_str(),
        parameters: vec![]
    }),
    "fdf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fdf".to_str(),
        parameters: vec![]
    }),
    "mseed" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fdsn.mseed".to_str(),
        parameters: vec![]
    }),
    "seed" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fdsn.seed".to_str(),
        parameters: vec![]
    }),
    "gph" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.flographit".to_str(),
        parameters: vec![]
    }),
    "ftc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fluxtime.clip".to_str(),
        parameters: vec![]
    }),
    "fm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.framemaker".to_str(),
        parameters: vec![]
    }),
    "fnc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.frogans.fnc".to_str(),
        parameters: vec![]
    }),
    "ltf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.frogans.ltf".to_str(),
        parameters: vec![]
    }),
    "fsc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fsc.weblaunch".to_str(),
        parameters: vec![]
    }),
    "oas" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fujitsu.oasys".to_str(),
        parameters: vec![]
    }),
    "oa2" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fujitsu.oasys2".to_str(),
        parameters: vec![]
    }),
    "oa3" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fujitsu.oasys3".to_str(),
        parameters: vec![]
    }),
    "fg5" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fujitsu.oasysgp".to_str(),
        parameters: vec![]
    }),
    "bh2" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fujitsu.oasysprs".to_str(),
        parameters: vec![]
    }),
    "ddd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fujixerox.ddd".to_str(),
        parameters: vec![]
    }),
    "xdw" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fujixerox.docuworks".to_str(),
        parameters: vec![]
    }),
    "xbd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fujixerox.docuworks.binder".to_str(),
        parameters: vec![]
    }),
    "fzs" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.fuzzysheet".to_str(),
        parameters: vec![]
    }),
    "txd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.genomatix.tuxedo".to_str(),
        parameters: vec![]
    }),
    "ggb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.geogebra.file".to_str(),
        parameters: vec![]
    }),
    "ggt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.geogebra.tool".to_str(),
        parameters: vec![]
    }),
    "gex" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.geometry-explorer".to_str(),
        parameters: vec![]
    }),
    "gxt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.geonext".to_str(),
        parameters: vec![]
    }),
    "g2w" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.geoplan".to_str(),
        parameters: vec![]
    }),
    "g3w" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.geospace".to_str(),
        parameters: vec![]
    }),
    "gmx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.gmx".to_str(),
        parameters: vec![]
    }),
    "kml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.google-earth.kml+xml".to_str(),
        parameters: vec![]
    }),
    "kmz" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.google-earth.kmz".to_str(),
        parameters: vec![]
    }),
    "gqf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.grafeq".to_str(),
        parameters: vec![]
    }),
    "gac" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.groove-account".to_str(),
        parameters: vec![]
    }),
    "ghf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.groove-help".to_str(),
        parameters: vec![]
    }),
    "gim" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.groove-identity-message".to_str(),
        parameters: vec![]
    }),
    "grv" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.groove-injector".to_str(),
        parameters: vec![]
    }),
    "gtm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.groove-tool-message".to_str(),
        parameters: vec![]
    }),
    "tpl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.groove-tool-template".to_str(),
        parameters: vec![]
    }),
    "vcg" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.groove-vcard".to_str(),
        parameters: vec![]
    }),
    "hal" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.hal+xml".to_str(),
        parameters: vec![]
    }),
    "zmm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.handheld-entertainment+xml".to_str(),
        parameters: vec![]
    }),
    "hbci" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.hbci".to_str(),
        parameters: vec![]
    }),
    "les" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.hhe.lesson-player".to_str(),
        parameters: vec![]
    }),
    "hpgl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.hp-hpgl".to_str(),
        parameters: vec![]
    }),
    "hpid" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.hp-hpid".to_str(),
        parameters: vec![]
    }),
    "hps" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.hp-hps".to_str(),
        parameters: vec![]
    }),
    "jlt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.hp-jlyt".to_str(),
        parameters: vec![]
    }),
    "pcl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.hp-pcl".to_str(),
        parameters: vec![]
    }),
    "pclxl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.hp-pclxl".to_str(),
        parameters: vec![]
    }),
    "sfd-hdstx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.hydrostatix.sof-data".to_str(),
        parameters: vec![]
    }),
    "mpy" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ibm.minipay".to_str(),
        parameters: vec![]
    }),
    "afp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ibm.modcap".to_str(),
        parameters: vec![]
    }),
    "irm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ibm.rights-management".to_str(),
        parameters: vec![]
    }),
    "sc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ibm.secure-container".to_str(),
        parameters: vec![]
    }),
    "icc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.iccprofile".to_str(),
        parameters: vec![]
    }),
    "igl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.igloader".to_str(),
        parameters: vec![]
    }),
    "ivp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.immervision-ivp".to_str(),
        parameters: vec![]
    }),
    "ivu" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.immervision-ivu".to_str(),
        parameters: vec![]
    }),
    "igm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.insors.igm".to_str(),
        parameters: vec![]
    }),
    "xpw" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.intercon.formnet".to_str(),
        parameters: vec![]
    }),
    "i2g" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.intergeo".to_str(),
        parameters: vec![]
    }),
    "qbo" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.intu.qbo".to_str(),
        parameters: vec![]
    }),
    "qfx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.intu.qfx".to_str(),
        parameters: vec![]
    }),
    "rcprofile" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ipunplugged.rcprofile".to_str(),
        parameters: vec![]
    }),
    "irp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.irepository.package+xml".to_str(),
        parameters: vec![]
    }),
    "xpr" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.is-xpr".to_str(),
        parameters: vec![]
    }),
    "fcs" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.isac.fcs".to_str(),
        parameters: vec![]
    }),
    "jam" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.jam".to_str(),
        parameters: vec![]
    }),
    "rms" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.jcp.javame.midlet-rms".to_str(),
        parameters: vec![]
    }),
    "jisp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.jisp".to_str(),
        parameters: vec![]
    }),
    "joda" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.joost.joda-archive".to_str(),
        parameters: vec![]
    }),
    "ktz" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.kahootz".to_str(),
        parameters: vec![]
    }),
    "karbon" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.kde.karbon".to_str(),
        parameters: vec![]
    }),
    "chrt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.kde.kchart".to_str(),
        parameters: vec![]
    }),
    "kfo" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.kde.kformula".to_str(),
        parameters: vec![]
    }),
    "flw" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.kde.kivio".to_str(),
        parameters: vec![]
    }),
    "kon" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.kde.kontour".to_str(),
        parameters: vec![]
    }),
    "kpr" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.kde.kpresenter".to_str(),
        parameters: vec![]
    }),
    "ksp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.kde.kspread".to_str(),
        parameters: vec![]
    }),
    "kwd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.kde.kword".to_str(),
        parameters: vec![]
    }),
    "htke" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.kenameaapp".to_str(),
        parameters: vec![]
    }),
    "kia" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.kidspiration".to_str(),
        parameters: vec![]
    }),
    "kne" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.kinar".to_str(),
        parameters: vec![]
    }),
    "skp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.koan".to_str(),
        parameters: vec![]
    }),
    "sse" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.kodak-descriptor".to_str(),
        parameters: vec![]
    }),
    "lasxml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.las.las+xml".to_str(),
        parameters: vec![]
    }),
    "lbd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.llamagraphics.life-balance.desktop".to_str(),
        parameters: vec![]
    }),
    "lbe" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.llamagraphics.life-balance.exchange+xml".to_str(),
        parameters: vec![]
    }),
    "123" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.lotus-1-2-3".to_str(),
        parameters: vec![]
    }),
    "apr" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.lotus-approach".to_str(),
        parameters: vec![]
    }),
    "pre" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.lotus-freelance".to_str(),
        parameters: vec![]
    }),
    "nsf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.lotus-notes".to_str(),
        parameters: vec![]
    }),
    "org" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.lotus-organizer".to_str(),
        parameters: vec![]
    }),
    "scm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.lotus-screencam".to_str(),
        parameters: vec![]
    }),
    "lwp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.lotus-wordpro".to_str(),
        parameters: vec![]
    }),
    "portpkg" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.macports.portpkg".to_str(),
        parameters: vec![]
    }),
    "mcd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mcd".to_str(),
        parameters: vec![]
    }),
    "mc1" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.medcalcdata".to_str(),
        parameters: vec![]
    }),
    "cdkey" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mediastation.cdkey".to_str(),
        parameters: vec![]
    }),
    "mwf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mfer".to_str(),
        parameters: vec![]
    }),
    "mfm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mfmp".to_str(),
        parameters: vec![]
    }),
    "flo" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.micrografx.flo".to_str(),
        parameters: vec![]
    }),
    "igx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.micrografx.igx".to_str(),
        parameters: vec![]
    }),
    "mif" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mif".to_str(),
        parameters: vec![]
    }),
    "daf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mobius.daf".to_str(),
        parameters: vec![]
    }),
    "dis" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mobius.dis".to_str(),
        parameters: vec![]
    }),
    "mbk" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mobius.mbk".to_str(),
        parameters: vec![]
    }),
    "mqy" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mobius.mqy".to_str(),
        parameters: vec![]
    }),
    "msl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mobius.msl".to_str(),
        parameters: vec![]
    }),
    "plc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mobius.plc".to_str(),
        parameters: vec![]
    }),
    "txf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mobius.txf".to_str(),
        parameters: vec![]
    }),
    "mpn" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mophun.application".to_str(),
        parameters: vec![]
    }),
    "mpc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mophun.certificate".to_str(),
        parameters: vec![]
    }),
    "xul" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mozilla.xul+xml".to_str(),
        parameters: vec![]
    }),
    "cil" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-artgalry".to_str(),
        parameters: vec![]
    }),
    "cab" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-cab-compressed".to_str(),
        parameters: vec![]
    }),
    "xls" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-excel".to_str(),
        parameters: vec![]
    }),
    "xlam" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-excel.addin.macroenabled.12".to_str(),
        parameters: vec![]
    }),
    "xlsb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-excel.sheet.binary.macroenabled.12".to_str(),
        parameters: vec![]
    }),
    "xlsm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-excel.sheet.macroenabled.12".to_str(),
        parameters: vec![]
    }),
    "xltm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-excel.template.macroenabled.12".to_str(),
        parameters: vec![]
    }),
    "eot" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-fontobject".to_str(),
        parameters: vec![]
    }),
    "chm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-htmlhelp".to_str(),
        parameters: vec![]
    }),
    "ims" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-ims".to_str(),
        parameters: vec![]
    }),
    "lrm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-lrm".to_str(),
        parameters: vec![]
    }),
    "thmx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-officetheme".to_str(),
        parameters: vec![]
    }),
    "cat" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-pki.seccat".to_str(),
        parameters: vec![]
    }),
    "stl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-pki.stl".to_str(),
        parameters: vec![]
    }),
    "ppt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-powerpoint".to_str(),
        parameters: vec![]
    }),
    "ppam" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-powerpoint.addin.macroenabled.12".to_str(),
        parameters: vec![]
    }),
    "pptm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-powerpoint.presentation.macroenabled.12".to_str(),
        parameters: vec![]
    }),
    "sldm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-powerpoint.slide.macroenabled.12".to_str(),
        parameters: vec![]
    }),
    "ppsm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-powerpoint.slideshow.macroenabled.12".to_str(),
        parameters: vec![]
    }),
    "potm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-powerpoint.template.macroenabled.12".to_str(),
        parameters: vec![]
    }),
    "mpp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-project".to_str(),
        parameters: vec![]
    }),
    "docm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-word.document.macroenabled.12".to_str(),
        parameters: vec![]
    }),
    "dotm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-word.template.macroenabled.12".to_str(),
        parameters: vec![]
    }),
    "wps" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-works".to_str(),
        parameters: vec![]
    }),
    "wpl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-wpl".to_str(),
        parameters: vec![]
    }),
    "xps" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ms-xpsdocument".to_str(),
        parameters: vec![]
    }),
    "mseq" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mseq".to_str(),
        parameters: vec![]
    }),
    "mus" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.musician".to_str(),
        parameters: vec![]
    }),
    "msty" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.muvee.style".to_str(),
        parameters: vec![]
    }),
    "taglet" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.mynfc".to_str(),
        parameters: vec![]
    }),
    "nlu" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.neurolanguage.nlu".to_str(),
        parameters: vec![]
    }),
    "ntf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.nitf".to_str(),
        parameters: vec![]
    }),
    "nnd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.noblenet-directory".to_str(),
        parameters: vec![]
    }),
    "nns" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.noblenet-sealer".to_str(),
        parameters: vec![]
    }),
    "nnw" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.noblenet-web".to_str(),
        parameters: vec![]
    }),
    "ngdat" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.nokia.n-gage.data".to_str(),
        parameters: vec![]
    }),
    "n-gage" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.nokia.n-gage.symbian.install".to_str(),
        parameters: vec![]
    }),
    "rpst" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.nokia.radio-preset".to_str(),
        parameters: vec![]
    }),
    "rpss" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.nokia.radio-presets".to_str(),
        parameters: vec![]
    }),
    "edm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.novadigm.edm".to_str(),
        parameters: vec![]
    }),
    "edx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.novadigm.edx".to_str(),
        parameters: vec![]
    }),
    "ext" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.novadigm.ext".to_str(),
        parameters: vec![]
    }),
    "odc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.chart".to_str(),
        parameters: vec![]
    }),
    "otc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.chart-template".to_str(),
        parameters: vec![]
    }),
    "odb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.database".to_str(),
        parameters: vec![]
    }),
    "odf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.formula".to_str(),
        parameters: vec![]
    }),
    "odft" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.formula-template".to_str(),
        parameters: vec![]
    }),
    "odg" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.graphics".to_str(),
        parameters: vec![]
    }),
    "otg" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.graphics-template".to_str(),
        parameters: vec![]
    }),
    "odi" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.image".to_str(),
        parameters: vec![]
    }),
    "oti" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.image-template".to_str(),
        parameters: vec![]
    }),
    "odp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.presentation".to_str(),
        parameters: vec![]
    }),
    "otp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.presentation-template".to_str(),
        parameters: vec![]
    }),
    "ods" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.spreadsheet".to_str(),
        parameters: vec![]
    }),
    "ots" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.spreadsheet-template".to_str(),
        parameters: vec![]
    }),
    "odt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.text".to_str(),
        parameters: vec![]
    }),
    "odm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.text-master".to_str(),
        parameters: vec![]
    }),
    "ott" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.text-template".to_str(),
        parameters: vec![]
    }),
    "oth" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oasis.opendocument.text-web".to_str(),
        parameters: vec![]
    }),
    "xo" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.olpc-sugar".to_str(),
        parameters: vec![]
    }),
    "dd2" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.oma.dd2+xml".to_str(),
        parameters: vec![]
    }),
    "oxt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.openofficeorg.extension".to_str(),
        parameters: vec![]
    }),
    "pptx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.openxmlformats-officedocument.presentationml.presentation".to_str(),
        parameters: vec![]
    }),
    "sldx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.openxmlformats-officedocument.presentationml.slide".to_str(),
        parameters: vec![]
    }),
    "ppsx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.openxmlformats-officedocument.presentationml.slideshow".to_str(),
        parameters: vec![]
    }),
    "potx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.openxmlformats-officedocument.presentationml.template".to_str(),
        parameters: vec![]
    }),
    "xlsx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_str(),
        parameters: vec![]
    }),
    "xltx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.template".to_str(),
        parameters: vec![]
    }),
    "docx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.document".to_str(),
        parameters: vec![]
    }),
    "dotx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.template".to_str(),
        parameters: vec![]
    }),
    "mgp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.osgeo.mapguide.package".to_str(),
        parameters: vec![]
    }),
    "dp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.osgi.dp".to_str(),
        parameters: vec![]
    }),
    "esa" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.osgi.subsystem".to_str(),
        parameters: vec![]
    }),
    "pdb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.palm".to_str(),
        parameters: vec![]
    }),
    "paw" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.pawaafile".to_str(),
        parameters: vec![]
    }),
    "str" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.pg.format".to_str(),
        parameters: vec![]
    }),
    "ei6" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.pg.osasli".to_str(),
        parameters: vec![]
    }),
    "efif" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.picsel".to_str(),
        parameters: vec![]
    }),
    "wg" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.pmi.widget".to_str(),
        parameters: vec![]
    }),
    "plf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.pocketlearn".to_str(),
        parameters: vec![]
    }),
    "pbd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.powerbuilder6".to_str(),
        parameters: vec![]
    }),
    "box" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.previewsystems.box".to_str(),
        parameters: vec![]
    }),
    "mgz" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.proteus.magazine".to_str(),
        parameters: vec![]
    }),
    "qps" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.publishare-delta-tree".to_str(),
        parameters: vec![]
    }),
    "ptid" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.pvi.ptid1".to_str(),
        parameters: vec![]
    }),
    "qxd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.quark.quarkxpress".to_str(),
        parameters: vec![]
    }),
    "bed" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.realvnc.bed".to_str(),
        parameters: vec![]
    }),
    "mxl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.recordare.musicxml".to_str(),
        parameters: vec![]
    }),
    "musicxml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.recordare.musicxml+xml".to_str(),
        parameters: vec![]
    }),
    "cryptonote" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.rig.cryptonote".to_str(),
        parameters: vec![]
    }),
    "cod" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.rim.cod".to_str(),
        parameters: vec![]
    }),
    "rm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.rn-realmedia".to_str(),
        parameters: vec![]
    }),
    "rmvb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.rn-realmedia-vbr".to_str(),
        parameters: vec![]
    }),
    "link66" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.route66.link66+xml".to_str(),
        parameters: vec![]
    }),
    "st" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.sailingtracker.track".to_str(),
        parameters: vec![]
    }),
    "see" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.seemail".to_str(),
        parameters: vec![]
    }),
    "sema" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.sema".to_str(),
        parameters: vec![]
    }),
    "semd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.semd".to_str(),
        parameters: vec![]
    }),
    "semf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.semf".to_str(),
        parameters: vec![]
    }),
    "ifm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.shana.informed.formdata".to_str(),
        parameters: vec![]
    }),
    "itp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.shana.informed.formtemplate".to_str(),
        parameters: vec![]
    }),
    "iif" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.shana.informed.interchange".to_str(),
        parameters: vec![]
    }),
    "ipk" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.shana.informed.package".to_str(),
        parameters: vec![]
    }),
    "twd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.simtech-mindmapper".to_str(),
        parameters: vec![]
    }),
    "mmf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.smaf".to_str(),
        parameters: vec![]
    }),
    "teacher" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.smart.teacher".to_str(),
        parameters: vec![]
    }),
    "sdkm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.solent.sdkm+xml".to_str(),
        parameters: vec![]
    }),
    "dxp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.spotfire.dxp".to_str(),
        parameters: vec![]
    }),
    "sfs" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.spotfire.sfs".to_str(),
        parameters: vec![]
    }),
    "sdc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.stardivision.calc".to_str(),
        parameters: vec![]
    }),
    "sda" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.stardivision.draw".to_str(),
        parameters: vec![]
    }),
    "sdd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.stardivision.impress".to_str(),
        parameters: vec![]
    }),
    "smf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.stardivision.math".to_str(),
        parameters: vec![]
    }),
    "sdw" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.stardivision.writer".to_str(),
        parameters: vec![]
    }),
    "sgl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.stardivision.writer-global".to_str(),
        parameters: vec![]
    }),
    "smzip" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.stepmania.package".to_str(),
        parameters: vec![]
    }),
    "sm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.stepmania.stepchart".to_str(),
        parameters: vec![]
    }),
    "sxc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.sun.xml.calc".to_str(),
        parameters: vec![]
    }),
    "stc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.sun.xml.calc.template".to_str(),
        parameters: vec![]
    }),
    "sxd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.sun.xml.draw".to_str(),
        parameters: vec![]
    }),
    "std" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.sun.xml.draw.template".to_str(),
        parameters: vec![]
    }),
    "sxi" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.sun.xml.impress".to_str(),
        parameters: vec![]
    }),
    "sti" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.sun.xml.impress.template".to_str(),
        parameters: vec![]
    }),
    "sxm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.sun.xml.math".to_str(),
        parameters: vec![]
    }),
    "sxw" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.sun.xml.writer".to_str(),
        parameters: vec![]
    }),
    "sxg" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.sun.xml.writer.global".to_str(),
        parameters: vec![]
    }),
    "stw" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.sun.xml.writer.template".to_str(),
        parameters: vec![]
    }),
    "sus" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.sus-calendar".to_str(),
        parameters: vec![]
    }),
    "svd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.svd".to_str(),
        parameters: vec![]
    }),
    "sis" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.symbian.install".to_str(),
        parameters: vec![]
    }),
    "xsm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.syncml+xml".to_str(),
        parameters: vec![]
    }),
    "bdm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.syncml.dm+wbxml".to_str(),
        parameters: vec![]
    }),
    "xdm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.syncml.dm+xml".to_str(),
        parameters: vec![]
    }),
    "tao" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.tao.intent-module-archive".to_str(),
        parameters: vec![]
    }),
    "pcap" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.tcpdump.pcap".to_str(),
        parameters: vec![]
    }),
    "tmo" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.tmobile-livetv".to_str(),
        parameters: vec![]
    }),
    "tpt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.trid.tpt".to_str(),
        parameters: vec![]
    }),
    "mxs" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.triscape.mxs".to_str(),
        parameters: vec![]
    }),
    "tra" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.trueapp".to_str(),
        parameters: vec![]
    }),
    "ufd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.ufdl".to_str(),
        parameters: vec![]
    }),
    "utz" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.uiq.theme".to_str(),
        parameters: vec![]
    }),
    "umj" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.umajin".to_str(),
        parameters: vec![]
    }),
    "unityweb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.unity".to_str(),
        parameters: vec![]
    }),
    "uoml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.uoml+xml".to_str(),
        parameters: vec![]
    }),
    "vcx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.vcx".to_str(),
        parameters: vec![]
    }),
    "vsd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.visio".to_str(),
        parameters: vec![]
    }),
    "vis" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.visionary".to_str(),
        parameters: vec![]
    }),
    "vsf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.vsf".to_str(),
        parameters: vec![]
    }),
    "wbxml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.wap.wbxml".to_str(),
        parameters: vec![]
    }),
    "wmlc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.wap.wmlc".to_str(),
        parameters: vec![]
    }),
    "wmlsc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.wap.wmlscriptc".to_str(),
        parameters: vec![]
    }),
    "wtb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.webturbo".to_str(),
        parameters: vec![]
    }),
    "nbp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.wolfram.player".to_str(),
        parameters: vec![]
    }),
    "wpd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.wordperfect".to_str(),
        parameters: vec![]
    }),
    "wqd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.wqd".to_str(),
        parameters: vec![]
    }),
    "stf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.wt.stf".to_str(),
        parameters: vec![]
    }),
    "xar" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.xara".to_str(),
        parameters: vec![]
    }),
    "xfdl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.xfdl".to_str(),
        parameters: vec![]
    }),
    "hvd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.yamaha.hv-dic".to_str(),
        parameters: vec![]
    }),
    "hvs" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.yamaha.hv-script".to_str(),
        parameters: vec![]
    }),
    "hvp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.yamaha.hv-voice".to_str(),
        parameters: vec![]
    }),
    "osf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.yamaha.openscoreformat".to_str(),
        parameters: vec![]
    }),
    "osfpvg" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.yamaha.openscoreformat.osfpvg+xml".to_str(),
        parameters: vec![]
    }),
    "saf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.yamaha.smaf-audio".to_str(),
        parameters: vec![]
    }),
    "spf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.yamaha.smaf-phrase".to_str(),
        parameters: vec![]
    }),
    "cmp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.yellowriver-custom-menu".to_str(),
        parameters: vec![]
    }),
    "zir" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.zul".to_str(),
        parameters: vec![]
    }),
    "zaz" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "vnd.zzazz.deck+xml".to_str(),
        parameters: vec![]
    }),
    "vxml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "voicexml+xml".to_str(),
        parameters: vec![]
    }),
    "wgt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "widget".to_str(),
        parameters: vec![]
    }),
    "hlp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "winhlp".to_str(),
        parameters: vec![]
    }),
    "wsdl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "wsdl+xml".to_str(),
        parameters: vec![]
    }),
    "wspolicy" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "wspolicy+xml".to_str(),
        parameters: vec![]
    }),
    "7z" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-7z-compressed".to_str(),
        parameters: vec![]
    }),
    "abw" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-abiword".to_str(),
        parameters: vec![]
    }),
    "ace" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-ace-compressed".to_str(),
        parameters: vec![]
    }),
    "dmg" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-apple-diskimage".to_str(),
        parameters: vec![]
    }),
    "aab" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-authorware-bin".to_str(),
        parameters: vec![]
    }),
    "aam" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-authorware-map".to_str(),
        parameters: vec![]
    }),
    "aas" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-authorware-seg".to_str(),
        parameters: vec![]
    }),
    "bcpio" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-bcpio".to_str(),
        parameters: vec![]
    }),
    "torrent" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-bittorrent".to_str(),
        parameters: vec![]
    }),
    "blb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-blorb".to_str(),
        parameters: vec![]
    }),
    "bz" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-bzip".to_str(),
        parameters: vec![]
    }),
    "bz2" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-bzip2".to_str(),
        parameters: vec![]
    }),
    "cbr" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-cbr".to_str(),
        parameters: vec![]
    }),
    "vcd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-cdlink".to_str(),
        parameters: vec![]
    }),
    "cfs" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-cfs-compressed".to_str(),
        parameters: vec![]
    }),
    "chat" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-chat".to_str(),
        parameters: vec![]
    }),
    "pgn" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-chess-pgn".to_str(),
        parameters: vec![]
    }),
    "nsc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-conference".to_str(),
        parameters: vec![]
    }),
    "cpio" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-cpio".to_str(),
        parameters: vec![]
    }),
    "csh" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-csh".to_str(),
        parameters: vec![]
    }),
    "deb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-debian-package".to_str(),
        parameters: vec![]
    }),
    "dgc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-dgc-compressed".to_str(),
        parameters: vec![]
    }),
    "dir" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-director".to_str(),
        parameters: vec![]
    }),
    "wad" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-doom".to_str(),
        parameters: vec![]
    }),
    "ncx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-dtbncx+xml".to_str(),
        parameters: vec![]
    }),
    "dtb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-dtbook+xml".to_str(),
        parameters: vec![]
    }),
    "res" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-dtbresource+xml".to_str(),
        parameters: vec![]
    }),
    "dvi" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-dvi".to_str(),
        parameters: vec![]
    }),
    "evy" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-envoy".to_str(),
        parameters: vec![]
    }),
    "eva" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-eva".to_str(),
        parameters: vec![]
    }),
    "bdf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-font-bdf".to_str(),
        parameters: vec![]
    }),
    "gsf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-font-ghostscript".to_str(),
        parameters: vec![]
    }),
    "psf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-font-linux-psf".to_str(),
        parameters: vec![]
    }),
    "otf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-font-otf".to_str(),
        parameters: vec![]
    }),
    "pcf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-font-pcf".to_str(),
        parameters: vec![]
    }),
    "snf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-font-snf".to_str(),
        parameters: vec![]
    }),
    "ttf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-font-ttf".to_str(),
        parameters: vec![]
    }),
    "pfa" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-font-type1".to_str(),
        parameters: vec![]
    }),
    "woff" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "font-woff".to_str(),
        parameters: vec![]
    }),
    "arc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-freearc".to_str(),
        parameters: vec![]
    }),
    "spl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-futuresplash".to_str(),
        parameters: vec![]
    }),
    "gca" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-gca-compressed".to_str(),
        parameters: vec![]
    }),
    "ulx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-glulx".to_str(),
        parameters: vec![]
    }),
    "gnumeric" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-gnumeric".to_str(),
        parameters: vec![]
    }),
    "gramps" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-gramps-xml".to_str(),
        parameters: vec![]
    }),
    "gtar" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-gtar".to_str(),
        parameters: vec![]
    }),
    "hdf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-hdf".to_str(),
        parameters: vec![]
    }),
    "install" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-install-instructions".to_str(),
        parameters: vec![]
    }),
    "iso" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-iso9660-image".to_str(),
        parameters: vec![]
    }),
    "jnlp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-java-jnlp-file".to_str(),
        parameters: vec![]
    }),
    "latex" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-latex".to_str(),
        parameters: vec![]
    }),
    "lzh" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-lzh-compressed".to_str(),
        parameters: vec![]
    }),
    "mie" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-mie".to_str(),
        parameters: vec![]
    }),
    "prc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-mobipocket-ebook".to_str(),
        parameters: vec![]
    }),
    "application" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-ms-application".to_str(),
        parameters: vec![]
    }),
    "lnk" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-ms-shortcut".to_str(),
        parameters: vec![]
    }),
    "wmd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-ms-wmd".to_str(),
        parameters: vec![]
    }),
    "wmz" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-ms-wmz".to_str(),
        parameters: vec![]
    }),
    "xbap" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-ms-xbap".to_str(),
        parameters: vec![]
    }),
    "mdb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-msaccess".to_str(),
        parameters: vec![]
    }),
    "obd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-msbinder".to_str(),
        parameters: vec![]
    }),
    "crd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-mscardfile".to_str(),
        parameters: vec![]
    }),
    "clp" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-msclip".to_str(),
        parameters: vec![]
    }),
    "exe" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-msdownload".to_str(),
        parameters: vec![]
    }),
    "mvb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-msmediaview".to_str(),
        parameters: vec![]
    }),
    "wmf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-msmetafile".to_str(),
        parameters: vec![]
    }),
    "mny" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-msmoney".to_str(),
        parameters: vec![]
    }),
    "pub" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-mspublisher".to_str(),
        parameters: vec![]
    }),
    "scd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-msschedule".to_str(),
        parameters: vec![]
    }),
    "trm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-msterminal".to_str(),
        parameters: vec![]
    }),
    "wri" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-mswrite".to_str(),
        parameters: vec![]
    }),
    "nc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-netcdf".to_str(),
        parameters: vec![]
    }),
    "nzb" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-nzb".to_str(),
        parameters: vec![]
    }),
    "p12" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-pkcs12".to_str(),
        parameters: vec![]
    }),
    "p7b" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-pkcs7-certificates".to_str(),
        parameters: vec![]
    }),
    "p7r" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-pkcs7-certreqresp".to_str(),
        parameters: vec![]
    }),
    "rar" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-rar-compressed".to_str(),
        parameters: vec![]
    }),
    "ris" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-research-info-systems".to_str(),
        parameters: vec![]
    }),
    "sh" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-sh".to_str(),
        parameters: vec![]
    }),
    "shar" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-shar".to_str(),
        parameters: vec![]
    }),
    "swf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-shockwave-flash".to_str(),
        parameters: vec![]
    }),
    "xap" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-silverlight-app".to_str(),
        parameters: vec![]
    }),
    "sql" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-sql".to_str(),
        parameters: vec![]
    }),
    "sit" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-stuffit".to_str(),
        parameters: vec![]
    }),
    "sitx" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-stuffitx".to_str(),
        parameters: vec![]
    }),
    "srt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-subrip".to_str(),
        parameters: vec![]
    }),
    "sv4cpio" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-sv4cpio".to_str(),
        parameters: vec![]
    }),
    "sv4crc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-sv4crc".to_str(),
        parameters: vec![]
    }),
    "t3" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-t3vm-image".to_str(),
        parameters: vec![]
    }),
    "gam" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-tads".to_str(),
        parameters: vec![]
    }),
    "tar" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-tar".to_str(),
        parameters: vec![]
    }),
    "tcl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-tcl".to_str(),
        parameters: vec![]
    }),
    "tex" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-tex".to_str(),
        parameters: vec![]
    }),
    "tfm" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-tex-tfm".to_str(),
        parameters: vec![]
    }),
    "texinfo" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-texinfo".to_str(),
        parameters: vec![]
    }),
    "obj" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-tgif".to_str(),
        parameters: vec![]
    }),
    "ustar" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-ustar".to_str(),
        parameters: vec![]
    }),
    "src" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-wais-source".to_str(),
        parameters: vec![]
    }),
    "der" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-x509-ca-cert".to_str(),
        parameters: vec![]
    }),
    "fig" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-xfig".to_str(),
        parameters: vec![]
    }),
    "xlf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-xliff+xml".to_str(),
        parameters: vec![]
    }),
    "xpi" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-xpinstall".to_str(),
        parameters: vec![]
    }),
    "xz" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-xz".to_str(),
        parameters: vec![]
    }),
    "z1" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "x-zmachine".to_str(),
        parameters: vec![]
    }),
    "xaml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "xaml+xml".to_str(),
        parameters: vec![]
    }),
    "xdf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "xcap-diff+xml".to_str(),
        parameters: vec![]
    }),
    "xenc" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "xenc+xml".to_str(),
        parameters: vec![]
    }),
    "xhtml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "xhtml+xml".to_str(),
        parameters: vec![]
    }),
    "xml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "xml".to_str(),
        parameters: vec![]
    }),
    "dtd" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "xml-dtd".to_str(),
        parameters: vec![]
    }),
    "xop" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "xop+xml".to_str(),
        parameters: vec![]
    }),
    "xpl" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "xproc+xml".to_str(),
        parameters: vec![]
    }),
    "xslt" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "xslt+xml".to_str(),
        parameters: vec![]
    }),
    "xspf" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "xspf+xml".to_str(),
        parameters: vec![]
    }),
    "mxml" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "xv+xml".to_str(),
        parameters: vec![]
    }),
    "yang" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "yang".to_str(),
        parameters: vec![]
    }),
    "yin" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "yin+xml".to_str(),
        parameters: vec![]
    }),
    "zip" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "zip".to_str(),
        parameters: vec![]
    }),
    "adp" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "adpcm".to_str(),
        parameters: vec![]
    }),
    "au" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "basic".to_str(),
        parameters: vec![]
    }),
    "mid" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "midi".to_str(),
        parameters: vec![]
    }),
    "mp4a" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "mp4".to_str(),
        parameters: vec![]
    }),
    "mpga" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "mpeg".to_str(),
        parameters: vec![]
    }),
    "oga" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "ogg".to_str(),
        parameters: vec![]
    }),
    "s3m" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "s3m".to_str(),
        parameters: vec![]
    }),
    "sil" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "silk".to_str(),
        parameters: vec![]
    }),
    "uva" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "vnd.dece.audio".to_str(),
        parameters: vec![]
    }),
    "eol" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "vnd.digital-winds".to_str(),
        parameters: vec![]
    }),
    "dra" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "vnd.dra".to_str(),
        parameters: vec![]
    }),
    "dts" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "vnd.dts".to_str(),
        parameters: vec![]
    }),
    "dtshd" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "vnd.dts.hd".to_str(),
        parameters: vec![]
    }),
    "lvp" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "vnd.lucent.voice".to_str(),
        parameters: vec![]
    }),
    "pya" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "vnd.ms-playready.media.pya".to_str(),
        parameters: vec![]
    }),
    "ecelp4800" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "vnd.nuera.ecelp4800".to_str(),
        parameters: vec![]
    }),
    "ecelp7470" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "vnd.nuera.ecelp7470".to_str(),
        parameters: vec![]
    }),
    "ecelp9600" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "vnd.nuera.ecelp9600".to_str(),
        parameters: vec![]
    }),
    "rip" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "vnd.rip".to_str(),
        parameters: vec![]
    }),
    "weba" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "webm".to_str(),
        parameters: vec![]
    }),
    "aac" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "x-aac".to_str(),
        parameters: vec![]
    }),
    "aif" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "x-aiff".to_str(),
        parameters: vec![]
    }),
    "caf" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "x-caf".to_str(),
        parameters: vec![]
    }),
    "flac" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "x-flac".to_str(),
        parameters: vec![]
    }),
    "mka" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "x-matroska".to_str(),
        parameters: vec![]
    }),
    "m3u" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "x-mpegurl".to_str(),
        parameters: vec![]
    }),
    "wax" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "x-ms-wax".to_str(),
        parameters: vec![]
    }),
    "wma" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "x-ms-wma".to_str(),
        parameters: vec![]
    }),
    "ram" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "x-pn-realaudio".to_str(),
        parameters: vec![]
    }),
    "rmp" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "x-pn-realaudio-plugin".to_str(),
        parameters: vec![]
    }),
    "wav" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "x-wav".to_str(),
        parameters: vec![]
    }),
    "xm" => Some(MediaType {
        type_: "audio".to_str(),
        subtype: "xm".to_str(),
        parameters: vec![]
    }),
    "cdx" => Some(MediaType {
        type_: "chemical".to_str(),
        subtype: "x-cdx".to_str(),
        parameters: vec![]
    }),
    "cif" => Some(MediaType {
        type_: "chemical".to_str(),
        subtype: "x-cif".to_str(),
        parameters: vec![]
    }),
    "cmdf" => Some(MediaType {
        type_: "chemical".to_str(),
        subtype: "x-cmdf".to_str(),
        parameters: vec![]
    }),
    "cml" => Some(MediaType {
        type_: "chemical".to_str(),
        subtype: "x-cml".to_str(),
        parameters: vec![]
    }),
    "csml" => Some(MediaType {
        type_: "chemical".to_str(),
        subtype: "x-csml".to_str(),
        parameters: vec![]
    }),
    "xyz" => Some(MediaType {
        type_: "chemical".to_str(),
        subtype: "x-xyz".to_str(),
        parameters: vec![]
    }),
    "bmp" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "bmp".to_str(),
        parameters: vec![]
    }),
    "cgm" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "cgm".to_str(),
        parameters: vec![]
    }),
    "g3" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "g3fax".to_str(),
        parameters: vec![]
    }),
    "gif" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "gif".to_str(),
        parameters: vec![]
    }),
    "ief" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "ief".to_str(),
        parameters: vec![]
    }),
    "jpeg" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "jpeg".to_str(),
        parameters: vec![]
    }),
    "ktx" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "ktx".to_str(),
        parameters: vec![]
    }),
    "png" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "png".to_str(),
        parameters: vec![]
    }),
    "btif" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "prs.btif".to_str(),
        parameters: vec![]
    }),
    "sgi" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "sgi".to_str(),
        parameters: vec![]
    }),
    "svg" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "svg+xml".to_str(),
        parameters: vec![]
    }),
    "tiff" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "tiff".to_str(),
        parameters: vec![]
    }),
    "psd" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.adobe.photoshop".to_str(),
        parameters: vec![]
    }),
    "uvi" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.dece.graphic".to_str(),
        parameters: vec![]
    }),
    "sub" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.dvb.subtitle".to_str(),
        parameters: vec![]
    }),
    "djvu" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.djvu".to_str(),
        parameters: vec![]
    }),
    "dwg" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.dwg".to_str(),
        parameters: vec![]
    }),
    "dxf" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.dxf".to_str(),
        parameters: vec![]
    }),
    "fbs" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.fastbidsheet".to_str(),
        parameters: vec![]
    }),
    "fpx" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.fpx".to_str(),
        parameters: vec![]
    }),
    "fst" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.fst".to_str(),
        parameters: vec![]
    }),
    "mmr" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.fujixerox.edmics-mmr".to_str(),
        parameters: vec![]
    }),
    "rlc" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.fujixerox.edmics-rlc".to_str(),
        parameters: vec![]
    }),
    "mdi" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.ms-modi".to_str(),
        parameters: vec![]
    }),
    "wdp" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.ms-photo".to_str(),
        parameters: vec![]
    }),
    "npx" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.net-fpx".to_str(),
        parameters: vec![]
    }),
    "wbmp" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.wap.wbmp".to_str(),
        parameters: vec![]
    }),
    "xif" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "vnd.xiff".to_str(),
        parameters: vec![]
    }),
    "webp" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "webp".to_str(),
        parameters: vec![]
    }),
    "3ds" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-3ds".to_str(),
        parameters: vec![]
    }),
    "ras" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-cmu-raster".to_str(),
        parameters: vec![]
    }),
    "cmx" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-cmx".to_str(),
        parameters: vec![]
    }),
    "fh" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-freehand".to_str(),
        parameters: vec![]
    }),
    "ico" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-icon".to_str(),
        parameters: vec![]
    }),
    "sid" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-mrsid-image".to_str(),
        parameters: vec![]
    }),
    "pcx" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-pcx".to_str(),
        parameters: vec![]
    }),
    "pic" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-pict".to_str(),
        parameters: vec![]
    }),
    "pnm" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-portable-anymap".to_str(),
        parameters: vec![]
    }),
    "pbm" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-portable-bitmap".to_str(),
        parameters: vec![]
    }),
    "pgm" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-portable-graymap".to_str(),
        parameters: vec![]
    }),
    "ppm" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-portable-pixmap".to_str(),
        parameters: vec![]
    }),
    "rgb" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-rgb".to_str(),
        parameters: vec![]
    }),
    "tga" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-tga".to_str(),
        parameters: vec![]
    }),
    "xbm" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-xbitmap".to_str(),
        parameters: vec![]
    }),
    "xpm" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-xpixmap".to_str(),
        parameters: vec![]
    }),
    "xwd" => Some(MediaType {
        type_: "image".to_str(),
        subtype: "x-xwindowdump".to_str(),
        parameters: vec![]
    }),
    "eml" => Some(MediaType {
        type_: "message".to_str(),
        subtype: "rfc822".to_str(),
        parameters: vec![]
    }),
    "igs" => Some(MediaType {
        type_: "model".to_str(),
        subtype: "iges".to_str(),
        parameters: vec![]
    }),
    "msh" => Some(MediaType {
        type_: "model".to_str(),
        subtype: "mesh".to_str(),
        parameters: vec![]
    }),
    "dae" => Some(MediaType {
        type_: "model".to_str(),
        subtype: "vnd.collada+xml".to_str(),
        parameters: vec![]
    }),
    "dwf" => Some(MediaType {
        type_: "model".to_str(),
        subtype: "vnd.dwf".to_str(),
        parameters: vec![]
    }),
    "gdl" => Some(MediaType {
        type_: "model".to_str(),
        subtype: "vnd.gdl".to_str(),
        parameters: vec![]
    }),
    "gtw" => Some(MediaType {
        type_: "model".to_str(),
        subtype: "vnd.gtw".to_str(),
        parameters: vec![]
    }),
    "mts" => Some(MediaType {
        type_: "model".to_str(),
        subtype: "vnd.mts".to_str(),
        parameters: vec![]
    }),
    "vtu" => Some(MediaType {
        type_: "model".to_str(),
        subtype: "vnd.vtu".to_str(),
        parameters: vec![]
    }),
    "wrl" => Some(MediaType {
        type_: "model".to_str(),
        subtype: "vrml".to_str(),
        parameters: vec![]
    }),
    "x3db" => Some(MediaType {
        type_: "model".to_str(),
        subtype: "x3d+binary".to_str(),
        parameters: vec![]
    }),
    "x3dv" => Some(MediaType {
        type_: "model".to_str(),
        subtype: "x3d+vrml".to_str(),
        parameters: vec![]
    }),
    "x3d" => Some(MediaType {
        type_: "model".to_str(),
        subtype: "x3d+xml".to_str(),
        parameters: vec![]
    }),
    "appcache" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "cache-manifest".to_str(),
        parameters: vec![]
    }),
    "ics" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "calendar".to_str(),
        parameters: vec![]
    }),
    "css" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "css".to_str(),
        parameters: vec![]
    }),
    "csv" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "csv".to_str(),
        parameters: vec![]
    }),
    "html" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "html".to_str(),
        parameters: vec![]
    }),
    "n3" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "n3".to_str(),
        parameters: vec![]
    }),
    "txt" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "plain".to_str(),
        parameters: vec![]
    }),
    "dsc" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "prs.lines.tag".to_str(),
        parameters: vec![]
    }),
    "rtx" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "richtext".to_str(),
        parameters: vec![]
    }),
    "sgml" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "sgml".to_str(),
        parameters: vec![]
    }),
    "tsv" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "tab-separated-values".to_str(),
        parameters: vec![]
    }),
    "t" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "troff".to_str(),
        parameters: vec![]
    }),
    "ttl" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "turtle".to_str(),
        parameters: vec![]
    }),
    "uri" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "uri-list".to_str(),
        parameters: vec![]
    }),
    "vcard" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "vcard".to_str(),
        parameters: vec![]
    }),
    "curl" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "vnd.curl".to_str(),
        parameters: vec![]
    }),
    "dcurl" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "vnd.curl.dcurl".to_str(),
        parameters: vec![]
    }),
    "scurl" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "vnd.curl.scurl".to_str(),
        parameters: vec![]
    }),
    "mcurl" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "vnd.curl.mcurl".to_str(),
        parameters: vec![]
    }),
    "fly" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "vnd.fly".to_str(),
        parameters: vec![]
    }),
    "flx" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "vnd.fmi.flexstor".to_str(),
        parameters: vec![]
    }),
    "gv" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "vnd.graphviz".to_str(),
        parameters: vec![]
    }),
    "3dml" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "vnd.in3d.3dml".to_str(),
        parameters: vec![]
    }),
    "spot" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "vnd.in3d.spot".to_str(),
        parameters: vec![]
    }),
    "jad" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "vnd.sun.j2me.app-descriptor".to_str(),
        parameters: vec![]
    }),
    "wml" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "vnd.wap.wml".to_str(),
        parameters: vec![]
    }),
    "wmls" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "vnd.wap.wmlscript".to_str(),
        parameters: vec![]
    }),
    "s" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "x-asm".to_str(),
        parameters: vec![]
    }),
    "c" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "x-c".to_str(),
        parameters: vec![]
    }),
    "f" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "x-fortran".to_str(),
        parameters: vec![]
    }),
    "java" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "x-java-source".to_str(),
        parameters: vec![]
    }),
    "opml" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "x-opml".to_str(),
        parameters: vec![]
    }),
    "p" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "x-pascal".to_str(),
        parameters: vec![]
    }),
    "nfo" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "x-nfo".to_str(),
        parameters: vec![]
    }),
    "etx" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "x-setext".to_str(),
        parameters: vec![]
    }),
    "sfv" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "x-sfv".to_str(),
        parameters: vec![]
    }),
    "uu" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "x-uuencode".to_str(),
        parameters: vec![]
    }),
    "vcs" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "x-vcalendar".to_str(),
        parameters: vec![]
    }),
    "vcf" => Some(MediaType {
        type_: "text".to_str(),
        subtype: "x-vcard".to_str(),
        parameters: vec![]
    }),
    "3gp" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "3gpp".to_str(),
        parameters: vec![]
    }),
    "3g2" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "3gpp2".to_str(),
        parameters: vec![]
    }),
    "h261" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "h261".to_str(),
        parameters: vec![]
    }),
    "h263" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "h263".to_str(),
        parameters: vec![]
    }),
    "h264" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "h264".to_str(),
        parameters: vec![]
    }),
    "jpgv" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "jpeg".to_str(),
        parameters: vec![]
    }),
    "jpm" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "jpm".to_str(),
        parameters: vec![]
    }),
    "mj2" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "mj2".to_str(),
        parameters: vec![]
    }),
    "mp4" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "mp4".to_str(),
        parameters: vec![]
    }),
    "mpeg" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "mpeg".to_str(),
        parameters: vec![]
    }),
    "ogv" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "ogg".to_str(),
        parameters: vec![]
    }),
    "qt" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "quicktime".to_str(),
        parameters: vec![]
    }),
    "uvh" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "vnd.dece.hd".to_str(),
        parameters: vec![]
    }),
    "uvm" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "vnd.dece.mobile".to_str(),
        parameters: vec![]
    }),
    "uvp" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "vnd.dece.pd".to_str(),
        parameters: vec![]
    }),
    "uvs" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "vnd.dece.sd".to_str(),
        parameters: vec![]
    }),
    "uvv" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "vnd.dece.video".to_str(),
        parameters: vec![]
    }),
    "dvb" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "vnd.dvb.file".to_str(),
        parameters: vec![]
    }),
    "fvt" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "vnd.fvt".to_str(),
        parameters: vec![]
    }),
    "mxu" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "vnd.mpegurl".to_str(),
        parameters: vec![]
    }),
    "pyv" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "vnd.ms-playready.media.pyv".to_str(),
        parameters: vec![]
    }),
    "uvu" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "vnd.uvvu.mp4".to_str(),
        parameters: vec![]
    }),
    "viv" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "vnd.vivo".to_str(),
        parameters: vec![]
    }),
    "webm" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "webm".to_str(),
        parameters: vec![]
    }),
    "f4v" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-f4v".to_str(),
        parameters: vec![]
    }),
    "fli" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-fli".to_str(),
        parameters: vec![]
    }),
    "flv" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-flv".to_str(),
        parameters: vec![]
    }),
    "m4v" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-m4v".to_str(),
        parameters: vec![]
    }),
    "mkv" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-matroska".to_str(),
        parameters: vec![]
    }),
    "mng" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-mng".to_str(),
        parameters: vec![]
    }),
    "asf" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-ms-asf".to_str(),
        parameters: vec![]
    }),
    "vob" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-ms-vob".to_str(),
        parameters: vec![]
    }),
    "wm" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-ms-wm".to_str(),
        parameters: vec![]
    }),
    "wmv" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-ms-wmv".to_str(),
        parameters: vec![]
    }),
    "wmx" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-ms-wmx".to_str(),
        parameters: vec![]
    }),
    "wvx" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-ms-wvx".to_str(),
        parameters: vec![]
    }),
    "avi" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-msvideo".to_str(),
        parameters: vec![]
    }),
    "movie" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-sgi-movie".to_str(),
        parameters: vec![]
    }),
    "smv" => Some(MediaType {
        type_: "video".to_str(),
        subtype: "x-smv".to_str(),
        parameters: vec![]
    }),
    "ice" => Some(MediaType {
        type_: "x-conference".to_str(),
        subtype: "x-cooltalk".to_str(),
        parameters: vec![]
    }),
        _ => None
    }
}
