// prasiolite-bb: a message board
// Copyright (C) 2020 QuietMisdreavus
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::borrow::Cow;

use chrono::{DateTime, Utc, TimeZone};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Sorting {
    Ascending,
    Descending,
}

/// `Sorting` always has a default of `Ascending` when parsing, to allow for malformed query
/// strings.
impl From<&str> for Sorting {
    fn from(input: &str) -> Sorting {
        if input == "desc" {
            Sorting::Descending
        } else {
            Sorting::Ascending
        }
    }
}

impl Default for Sorting {
    fn default() -> Sorting {
        Sorting::Ascending
    }
}

#[derive(Clone, Debug)]
pub struct Forum {
    pub id: u32,
    pub name: Cow<'static, str>,
    pub description: Cow<'static, str>,
}

static TEST_FORUMS: &'static [Forum] = &[
    Forum {
        id: 1,
        name: Cow::Borrowed("General"),
        description: Cow::Borrowed("For whatever you want to post about."),
    },
    Forum {
        id: 2,
        name: Cow::Borrowed("Tech"),
        description: Cow::Borrowed("Get technical!"),
    },
    Forum {
        id: 3,
        name: Cow::Borrowed("Get Sad"),
        description: Cow::Borrowed("Personal venting space. Threads are more heavily moderated here."),
    }
];

pub fn get_forums() -> Vec<Forum> {
    TEST_FORUMS.to_vec()
}

pub fn get_forum(forum: u32) -> Option<Forum> {
    get_forums().into_iter().find(|f| f.id == forum)
}

pub struct Topic {
    pub id: u64,
    pub forum_id: u32,
    pub name: Cow<'static, str>,
    pub author: Cow<'static, str>,
    pub opened: DateTime<Utc>,
    pub last_post: DateTime<Utc>,
    pub post_count: u64,
}

pub fn get_topics(forum: u32, sort: Sorting) -> Vec<Topic> {
    let mut output = vec![
        Topic {
            id: (forum as u64 * 1000) + 1,
            forum_id: forum,
            name: "Test Thread 1".into(),
            author: "Test Author".into(),
            opened: Utc.ymd(2020, 1, 20).and_hms(13, 12, 11),
            last_post: Utc.ymd(2020, 2, 3).and_hms(20, 22, 34),
            post_count: 20,
        },
        Topic {
            id: (forum as u64 * 1000) + 2,
            forum_id: forum,
            name: "Test Thread 2".into(),
            author: "Test Author".into(),
            opened: Utc.ymd(2020, 1, 22).and_hms(13, 12, 11),
            last_post: Utc.ymd(2020, 3, 3).and_hms(14, 7, 42),
            post_count: 45,
        },
        Topic {
            id: (forum as u64 * 1000) + 3,
            forum_id: forum,
            name: "Test Thread 3".into(),
            author: "Test Author".into(),
            opened: Utc.ymd(2020, 4, 20).and_hms(18, 38, 51),
            last_post: Utc.ymd(2020, 4, 22).and_hms(20, 22, 20),
            post_count: 2,
        },
    ];

    output.sort_by_key(|t| t.last_post);

    if sort == Sorting::Descending {
        output.reverse();
    }

    output
}
