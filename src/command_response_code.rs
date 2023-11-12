#[derive(Debug)]
pub enum IrcCommandResponse {
    RplNone = 300,
    RplUserhost = 302,
    RplIson = 303,
    RplAway = 301,
    RplUnaway = 305,
    RplNowaway = 306,
    RplWhoisUser = 311,
    RplWhoisServer = 312,
    RplWhoisOperator = 313,
    RplWhoisIdle = 317,
    RplEndOfWhois = 318,
    RplWhoisChannels = 319,
    RplWhowasUser = 314,
    RplEndOfWhowas = 369,
    RplListStart = 321,
    RplList = 322,
    RplListEnd = 323,
    RplChannelModeIs = 324,
    RplNoTopic = 331,
    RplTopic = 332,
    RplInviting = 341,
    RplSummoning = 342,
    RplVersion = 351,
    RplWhoReply = 352,
    RplEndOfWho = 315,
    RplNamReply = 353,
    RplEndOfNames = 366,
    RplLinks = 364,
    RplEndOfLinks = 365,
    RplBanList = 367,
    RplEndOfBanList = 368,
    RplInfo = 371,
    RplEndOfInfo = 374,
    RplMotdStart = 375,
    RplMotd = 372,
    RplEndOfMotd = 376,
    RplYoureOper = 381,
    RplRehashing = 382,
    RplTime = 391,
    RplUsersStart = 392,
    RplUsers = 393,
    RplEndOfUsers = 394,
    RplNoUsers = 395,
    RplTraceLink = 200,
    RplTraceConnecting = 201,
    RplTraceHandshake = 202,
    RplTraceUnknown = 203,
    RplTraceOperator = 204,
    RplTraceUser = 205,
    RplTraceServer = 206,
    RplTraceNewType = 208,
    RplTraceLog = 261,
    RplStatsLinkInfo = 211,
    RplStatsCommands = 212,
    RplStatsCline = 213,
    RplStatsNline = 214,
    RplStatsIline = 215,
    RplStatsKline = 216,
    RplStatsYline = 218,
    RplEndOfStats = 219,
    RplStatsLline = 241,
    RplStatsUptime = 242,
    RplStatsOline = 243,
    RplStatsHline = 244,
    RplUModeIs = 221,
    RplLuserClient = 251,
    RplLuserOp = 252,
    RplLuserUnknown = 253,
    RplLuserChannels = 254,
    RplLuserMe = 255,
    RplAdminMe = 256,
    RplAdminLoc1 = 257,
    RplAdminLoc2 = 258,
    RplAdminEmail = 259,
    RplWelcome = 1,
}

impl IrcCommandResponse {
    pub fn format_response(&self, args: &[&str]) -> String {
        match self {
            IrcCommandResponse::RplNone => "Dummy reply number. Not used.".to_string(),
            IrcCommandResponse::RplUserhost => format!(":{}", args.join(" ")),
            IrcCommandResponse::RplIson => format!(":{}", args.join(" ")),
            IrcCommandResponse::RplAway => format!("{} :{}", args[0], args[1]),
            IrcCommandResponse::RplUnaway => ":You are no longer marked as being away".to_string(),
            IrcCommandResponse::RplNowaway => ":You have been marked as being away".to_string(),
            IrcCommandResponse::RplWhoisUser => {
                format!("{} {} {} * :{}", args[0], args[1], args[2], args[3])
            }
            IrcCommandResponse::RplWhoisServer => format!("{} {} :{}", args[0], args[1], args[2]),
            IrcCommandResponse::RplWhoisOperator => format!("{} :is an IRC operator", args[0]),
            IrcCommandResponse::RplWhoisIdle => format!("{} {} :seconds idle", args[0], args[1]),
            IrcCommandResponse::RplEndOfWhois => format!("{} :End of /WHOIS list", args[0]),
            IrcCommandResponse::RplWhoisChannels => format!("{} :{}", args[0], args[1]),
            IrcCommandResponse::RplWhowasUser => {
                format!("{} {} {} * :{}", args[0], args[1], args[2], args[3])
            }
            IrcCommandResponse::RplEndOfWhowas => format!("{} :End of WHOWAS", args[0]),
            IrcCommandResponse::RplListStart => "Channel :Users  Name".to_string(),
            IrcCommandResponse::RplList => format!("{} {} :{}", args[0], args[1], args[2]),
            IrcCommandResponse::RplListEnd => ":End of /LIST".to_string(),
            IrcCommandResponse::RplChannelModeIs => format!("{} {}", args[0], args[1]),
            IrcCommandResponse::RplNoTopic => format!("{} :No topic is set", args[0]),
            IrcCommandResponse::RplTopic => format!("{} :{}", args[0], args[1]),
            IrcCommandResponse::RplInviting => format!("{} {}", args[0], args[1]),
            IrcCommandResponse::RplSummoning => format!("{} :Summoning user to IRC", args[0]),
            IrcCommandResponse::RplVersion => {
                format!("{}.{} {} :{}", args[0], args[1], args[2], args[3])
            }
            IrcCommandResponse::RplWhoReply => format!(
                "{} {} {} {} {} <H|G>[*][@|+] :{} {}",
                args[0], args[1], args[2], args[3], args[4], args[5], args[6]
            ),
            IrcCommandResponse::RplEndOfWho => format!("{} :End of /WHO list", args[0]),
            IrcCommandResponse::RplNamReply => format!("{} :{}", args[0], args[1]),
            IrcCommandResponse::RplEndOfNames => format!("{} :End of /NAMES list", args[0]),
            IrcCommandResponse::RplLinks => {
                format!("{} {} :{} {}", args[0], args[1], args[2], args[3])
            }
            IrcCommandResponse::RplEndOfLinks => format!("{} :End of /LINKS list", args[0]),
            IrcCommandResponse::RplBanList => format!("{} {}", args[0], args[1]),
            IrcCommandResponse::RplEndOfBanList => format!("{} :End of channel ban list", args[0]),
            IrcCommandResponse::RplInfo => format!(":{}", args[0]),
            IrcCommandResponse::RplEndOfInfo => ":End of /INFO list".to_string(),
            IrcCommandResponse::RplMotdStart => format!(":- {} Message of the day - ", args[0]),
            IrcCommandResponse::RplMotd => format!(":- {}", args[0]),
            IrcCommandResponse::RplEndOfMotd => ":End of /MOTD command".to_string(),
            IrcCommandResponse::RplYoureOper => ":You are now an IRC operator".to_string(),
            IrcCommandResponse::RplRehashing => format!("{} :Rehashing", args[0]),
            IrcCommandResponse::RplTime => format!("{} :{}", args[0], args[1]),
            IrcCommandResponse::RplUsersStart => ":UserID   Terminal  Host".to_string(),
            IrcCommandResponse::RplUsers => format!(":{} {} {}", args[0], args[1], args[2]), // TODO: Fix this to match ":UserID   Terminal  Host
            IrcCommandResponse::RplEndOfUsers => ":End of users".to_string(),
            IrcCommandResponse::RplNoUsers => ":Nobody logged in".to_string(),
            IrcCommandResponse::RplTraceLink => format!("Link {} {} {}", args[0], args[1], args[2]),
            IrcCommandResponse::RplTraceConnecting => format!("Try. {} {}", args[0], args[1]),
            IrcCommandResponse::RplTraceHandshake => format!("H.S. {} {}", args[0], args[1]),
            IrcCommandResponse::RplTraceUnknown => format!("???? {} {}", args[0], args[1]),
            IrcCommandResponse::RplTraceOperator => format!("Oper {} {}", args[0], args[1]),
            IrcCommandResponse::RplTraceUser => format!("User {} {}", args[0], args[1]),
            IrcCommandResponse::RplTraceServer => format!(
                "Serv {} {}S {}C {} {}@{}",
                args[0], args[1], args[2], args[3], args[4], args[5]
            ),
            IrcCommandResponse::RplTraceNewType => format!("{} 0 {}", args[0], args[1]),
            IrcCommandResponse::RplTraceLog => format!("File {} {}", args[0], args[1]),
            IrcCommandResponse::RplStatsLinkInfo => format!(
                "{} {} {} {} {} {} {}",
                args[0], args[1], args[2], args[3], args[4], args[5], args[6]
            ),
            IrcCommandResponse::RplStatsCommands => format!("{} {}", args[0], args[1]),
            IrcCommandResponse::RplStatsCline => {
                format!("C {} * {} {} {}", args[0], args[1], args[2], args[3])
            }
            IrcCommandResponse::RplStatsNline => {
                format!("N {} * {} {} {}", args[0], args[1], args[2], args[3])
            }
            IrcCommandResponse::RplStatsIline => {
                format!("I {} * {} {} {}", args[0], args[1], args[2], args[3])
            }
            IrcCommandResponse::RplStatsKline => {
                format!("K {} * {} {} {}", args[0], args[1], args[2], args[3])
            }
            IrcCommandResponse::RplStatsYline => {
                format!("Y {} {} {} {}", args[0], args[1], args[2], args[3])
            }
            IrcCommandResponse::RplEndOfStats => format!("{} :End of /STATS report", args[0]),
            IrcCommandResponse::RplStatsLline => format!("L {} * {} {}", args[0], args[1], args[2]),
            IrcCommandResponse::RplStatsUptime => {
                format!(":Server Up {} days {}:{}", args[0], args[1], args[2])
            }
            IrcCommandResponse::RplStatsOline => format!("O {} * {}", args[0], args[1]),
            IrcCommandResponse::RplStatsHline => format!("H {} * {}", args[0], args[1]),
            IrcCommandResponse::RplUModeIs => format!("{}", args[0]),
            IrcCommandResponse::RplLuserClient => format!(
                ":There are {} users and {} invisible on {} servers",
                args[0], args[1], args[2]
            ),
            IrcCommandResponse::RplLuserOp => format!("{} :operator(s) online", args[0]),
            IrcCommandResponse::RplLuserUnknown => format!("{} :unknown connection(s)", args[0]),
            IrcCommandResponse::RplLuserChannels => format!("{} :channels formed", args[0]),
            IrcCommandResponse::RplLuserMe => {
                format!(":I have {} clients and {} servers", args[0], args[1])
            }
            IrcCommandResponse::RplAdminMe => format!("{} :Administrative info", args[0]),
            IrcCommandResponse::RplAdminLoc1 => format!(":{}", args[0]),
            IrcCommandResponse::RplAdminLoc2 => format!(":{}", args[0]),
            IrcCommandResponse::RplAdminEmail => format!(":{}", args[0]),
            IrcCommandResponse::RplWelcome => format!(
                ":Server 001 {} :Welcome to the {} Network, {}",
                args[1], args[0], args[1]
            ), // Server 001 Nickname :Welcome to the Internet Relay Network Nickname
            _ => "Response not implemented".to_string(),
        }
    }
}
