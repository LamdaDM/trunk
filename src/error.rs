// use std::{fmt::{self, Display, Formatter}, num::ParseIntError, str::Utf8Error};

// use bunker::exception::BunkerError;


// #[derive(Debug)]
// pub enum TrunkError {
//     Utf8ToStr(Utf8Error),
//     Format(ParseIntError),
//     Mysql(mysql::Error)
// }

// impl Display for TrunkError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             TrunkError::Utf8ToStr(err) => Display::fmt(&err, f),
//             TrunkError::Format(err) => Display::fmt(&err, f),
//             TrunkError::Mysql(err) => Display::fmt(&err, f),
//         }
//     }
// }

// impl From<Utf8Error> for TrunkError { fn from(err: Utf8Error)->TrunkError { TrunkError::Utf8ToStr(err) } }
// impl From<ParseIntError> for TrunkError { fn from(err: ParseIntError)->TrunkError { TrunkError::Format(err) } }
// impl From<mysql::Error> for TrunkError { fn from(err: mysql::Error)->TrunkError { TrunkError::Mysql(err) } }

// impl From<TrunkError> for BunkerError { fn from(err: TrunkError) -> BunkerError { BunkerError::BadRequest(err.to_string()) }
// }
