/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.5
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CountryCodeEnum {
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "AS")]
    As,
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CX")]
    Cx,
    #[serde(rename = "CC")]
    Cc,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "CU")]
    Cu,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HM")]
    Hm,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IR")]
    Ir,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MH")]
    Mh,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "FM")]
    Fm,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NF")]
    Nf,
    #[serde(rename = "KP")]
    Kp,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "MP")]
    Mp,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PW")]
    Pw,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "SD")]
    Sd,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "SY")]
    Sy,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "UM")]
    Um,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VI")]
    Vi,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
}

impl std::fmt::Display for CountryCodeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Af => write!(f, "AF"),
            Self::Ax => write!(f, "AX"),
            Self::Al => write!(f, "AL"),
            Self::Dz => write!(f, "DZ"),
            Self::As => write!(f, "AS"),
            Self::Ad => write!(f, "AD"),
            Self::Ao => write!(f, "AO"),
            Self::Ai => write!(f, "AI"),
            Self::Aq => write!(f, "AQ"),
            Self::Ag => write!(f, "AG"),
            Self::Ar => write!(f, "AR"),
            Self::Am => write!(f, "AM"),
            Self::Aw => write!(f, "AW"),
            Self::Au => write!(f, "AU"),
            Self::At => write!(f, "AT"),
            Self::Az => write!(f, "AZ"),
            Self::Bs => write!(f, "BS"),
            Self::Bh => write!(f, "BH"),
            Self::Bd => write!(f, "BD"),
            Self::Bb => write!(f, "BB"),
            Self::By => write!(f, "BY"),
            Self::Be => write!(f, "BE"),
            Self::Bz => write!(f, "BZ"),
            Self::Bj => write!(f, "BJ"),
            Self::Bm => write!(f, "BM"),
            Self::Bt => write!(f, "BT"),
            Self::Bo => write!(f, "BO"),
            Self::Bq => write!(f, "BQ"),
            Self::Ba => write!(f, "BA"),
            Self::Bw => write!(f, "BW"),
            Self::Bv => write!(f, "BV"),
            Self::Br => write!(f, "BR"),
            Self::Io => write!(f, "IO"),
            Self::Bn => write!(f, "BN"),
            Self::Bg => write!(f, "BG"),
            Self::Bf => write!(f, "BF"),
            Self::Bi => write!(f, "BI"),
            Self::Cv => write!(f, "CV"),
            Self::Kh => write!(f, "KH"),
            Self::Cm => write!(f, "CM"),
            Self::Ca => write!(f, "CA"),
            Self::Ky => write!(f, "KY"),
            Self::Cf => write!(f, "CF"),
            Self::Td => write!(f, "TD"),
            Self::Cl => write!(f, "CL"),
            Self::Cn => write!(f, "CN"),
            Self::Cx => write!(f, "CX"),
            Self::Cc => write!(f, "CC"),
            Self::Co => write!(f, "CO"),
            Self::Km => write!(f, "KM"),
            Self::Cg => write!(f, "CG"),
            Self::Cd => write!(f, "CD"),
            Self::Ck => write!(f, "CK"),
            Self::Cr => write!(f, "CR"),
            Self::Ci => write!(f, "CI"),
            Self::Hr => write!(f, "HR"),
            Self::Cu => write!(f, "CU"),
            Self::Cw => write!(f, "CW"),
            Self::Cy => write!(f, "CY"),
            Self::Cz => write!(f, "CZ"),
            Self::Dk => write!(f, "DK"),
            Self::Dj => write!(f, "DJ"),
            Self::Dm => write!(f, "DM"),
            Self::Do => write!(f, "DO"),
            Self::Ec => write!(f, "EC"),
            Self::Eg => write!(f, "EG"),
            Self::Sv => write!(f, "SV"),
            Self::Gq => write!(f, "GQ"),
            Self::Er => write!(f, "ER"),
            Self::Ee => write!(f, "EE"),
            Self::Sz => write!(f, "SZ"),
            Self::Et => write!(f, "ET"),
            Self::Fk => write!(f, "FK"),
            Self::Fo => write!(f, "FO"),
            Self::Fj => write!(f, "FJ"),
            Self::Fi => write!(f, "FI"),
            Self::Fr => write!(f, "FR"),
            Self::Gf => write!(f, "GF"),
            Self::Pf => write!(f, "PF"),
            Self::Tf => write!(f, "TF"),
            Self::Ga => write!(f, "GA"),
            Self::Gm => write!(f, "GM"),
            Self::Ge => write!(f, "GE"),
            Self::De => write!(f, "DE"),
            Self::Gh => write!(f, "GH"),
            Self::Gi => write!(f, "GI"),
            Self::Gr => write!(f, "GR"),
            Self::Gl => write!(f, "GL"),
            Self::Gd => write!(f, "GD"),
            Self::Gp => write!(f, "GP"),
            Self::Gu => write!(f, "GU"),
            Self::Gt => write!(f, "GT"),
            Self::Gg => write!(f, "GG"),
            Self::Gn => write!(f, "GN"),
            Self::Gw => write!(f, "GW"),
            Self::Gy => write!(f, "GY"),
            Self::Ht => write!(f, "HT"),
            Self::Hm => write!(f, "HM"),
            Self::Va => write!(f, "VA"),
            Self::Hn => write!(f, "HN"),
            Self::Hk => write!(f, "HK"),
            Self::Hu => write!(f, "HU"),
            Self::Is => write!(f, "IS"),
            Self::In => write!(f, "IN"),
            Self::Id => write!(f, "ID"),
            Self::Ir => write!(f, "IR"),
            Self::Iq => write!(f, "IQ"),
            Self::Ie => write!(f, "IE"),
            Self::Im => write!(f, "IM"),
            Self::Il => write!(f, "IL"),
            Self::It => write!(f, "IT"),
            Self::Jm => write!(f, "JM"),
            Self::Jp => write!(f, "JP"),
            Self::Je => write!(f, "JE"),
            Self::Jo => write!(f, "JO"),
            Self::Kz => write!(f, "KZ"),
            Self::Ke => write!(f, "KE"),
            Self::Ki => write!(f, "KI"),
            Self::Kw => write!(f, "KW"),
            Self::Kg => write!(f, "KG"),
            Self::La => write!(f, "LA"),
            Self::Lv => write!(f, "LV"),
            Self::Lb => write!(f, "LB"),
            Self::Ls => write!(f, "LS"),
            Self::Lr => write!(f, "LR"),
            Self::Ly => write!(f, "LY"),
            Self::Li => write!(f, "LI"),
            Self::Lt => write!(f, "LT"),
            Self::Lu => write!(f, "LU"),
            Self::Mo => write!(f, "MO"),
            Self::Mg => write!(f, "MG"),
            Self::Mw => write!(f, "MW"),
            Self::My => write!(f, "MY"),
            Self::Mv => write!(f, "MV"),
            Self::Ml => write!(f, "ML"),
            Self::Mt => write!(f, "MT"),
            Self::Mh => write!(f, "MH"),
            Self::Mq => write!(f, "MQ"),
            Self::Mr => write!(f, "MR"),
            Self::Mu => write!(f, "MU"),
            Self::Yt => write!(f, "YT"),
            Self::Mx => write!(f, "MX"),
            Self::Fm => write!(f, "FM"),
            Self::Md => write!(f, "MD"),
            Self::Mc => write!(f, "MC"),
            Self::Mn => write!(f, "MN"),
            Self::Me => write!(f, "ME"),
            Self::Ms => write!(f, "MS"),
            Self::Ma => write!(f, "MA"),
            Self::Mz => write!(f, "MZ"),
            Self::Mm => write!(f, "MM"),
            Self::Na => write!(f, "NA"),
            Self::Nr => write!(f, "NR"),
            Self::Np => write!(f, "NP"),
            Self::Nl => write!(f, "NL"),
            Self::Nc => write!(f, "NC"),
            Self::Nz => write!(f, "NZ"),
            Self::Ni => write!(f, "NI"),
            Self::Ne => write!(f, "NE"),
            Self::Ng => write!(f, "NG"),
            Self::Nu => write!(f, "NU"),
            Self::Nf => write!(f, "NF"),
            Self::Kp => write!(f, "KP"),
            Self::Mk => write!(f, "MK"),
            Self::Mp => write!(f, "MP"),
            Self::No => write!(f, "NO"),
            Self::Om => write!(f, "OM"),
            Self::Pk => write!(f, "PK"),
            Self::Pw => write!(f, "PW"),
            Self::Ps => write!(f, "PS"),
            Self::Pa => write!(f, "PA"),
            Self::Pg => write!(f, "PG"),
            Self::Py => write!(f, "PY"),
            Self::Pe => write!(f, "PE"),
            Self::Ph => write!(f, "PH"),
            Self::Pn => write!(f, "PN"),
            Self::Pl => write!(f, "PL"),
            Self::Pt => write!(f, "PT"),
            Self::Pr => write!(f, "PR"),
            Self::Qa => write!(f, "QA"),
            Self::Re => write!(f, "RE"),
            Self::Ro => write!(f, "RO"),
            Self::Ru => write!(f, "RU"),
            Self::Rw => write!(f, "RW"),
            Self::Bl => write!(f, "BL"),
            Self::Sh => write!(f, "SH"),
            Self::Kn => write!(f, "KN"),
            Self::Lc => write!(f, "LC"),
            Self::Mf => write!(f, "MF"),
            Self::Pm => write!(f, "PM"),
            Self::Vc => write!(f, "VC"),
            Self::Ws => write!(f, "WS"),
            Self::Sm => write!(f, "SM"),
            Self::St => write!(f, "ST"),
            Self::Sa => write!(f, "SA"),
            Self::Sn => write!(f, "SN"),
            Self::Rs => write!(f, "RS"),
            Self::Sc => write!(f, "SC"),
            Self::Sl => write!(f, "SL"),
            Self::Sg => write!(f, "SG"),
            Self::Sx => write!(f, "SX"),
            Self::Sk => write!(f, "SK"),
            Self::Si => write!(f, "SI"),
            Self::Sb => write!(f, "SB"),
            Self::So => write!(f, "SO"),
            Self::Za => write!(f, "ZA"),
            Self::Gs => write!(f, "GS"),
            Self::Kr => write!(f, "KR"),
            Self::Ss => write!(f, "SS"),
            Self::Es => write!(f, "ES"),
            Self::Lk => write!(f, "LK"),
            Self::Sd => write!(f, "SD"),
            Self::Sr => write!(f, "SR"),
            Self::Sj => write!(f, "SJ"),
            Self::Se => write!(f, "SE"),
            Self::Ch => write!(f, "CH"),
            Self::Sy => write!(f, "SY"),
            Self::Tw => write!(f, "TW"),
            Self::Tj => write!(f, "TJ"),
            Self::Tz => write!(f, "TZ"),
            Self::Th => write!(f, "TH"),
            Self::Tl => write!(f, "TL"),
            Self::Tg => write!(f, "TG"),
            Self::Tk => write!(f, "TK"),
            Self::To => write!(f, "TO"),
            Self::Tt => write!(f, "TT"),
            Self::Tn => write!(f, "TN"),
            Self::Tr => write!(f, "TR"),
            Self::Tm => write!(f, "TM"),
            Self::Tc => write!(f, "TC"),
            Self::Tv => write!(f, "TV"),
            Self::Ug => write!(f, "UG"),
            Self::Ua => write!(f, "UA"),
            Self::Ae => write!(f, "AE"),
            Self::Gb => write!(f, "GB"),
            Self::Um => write!(f, "UM"),
            Self::Us => write!(f, "US"),
            Self::Uy => write!(f, "UY"),
            Self::Uz => write!(f, "UZ"),
            Self::Vu => write!(f, "VU"),
            Self::Ve => write!(f, "VE"),
            Self::Vn => write!(f, "VN"),
            Self::Vg => write!(f, "VG"),
            Self::Vi => write!(f, "VI"),
            Self::Wf => write!(f, "WF"),
            Self::Eh => write!(f, "EH"),
            Self::Ye => write!(f, "YE"),
            Self::Zm => write!(f, "ZM"),
            Self::Zw => write!(f, "ZW"),
        }
    }
}

impl Default for CountryCodeEnum {
    fn default() -> CountryCodeEnum {
        Self::Af
    }
}
