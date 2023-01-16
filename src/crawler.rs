use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use url::Url;

use crate::fetch::{fetch_all_urls, url_status, UrlState};