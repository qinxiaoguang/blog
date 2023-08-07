// github helper
use log::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Music {
    pub name: String,
    pub artist: String,
    pub url: String,
    pub cover: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Music163 {
    pub result: Option<Music163Result>,
    pub code: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Music163Result {
    pub tracks: Option<Vec<Music163Track>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Music163Track {
    pub name: String,
    pub id: i32,
    pub artists: Option<Vec<Music163Artist>>,
    pub album: Option<Music163Album>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Music163Artist {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Music163Album {
    pub pic_url: String,
}

// 通过github回调的code获取access_token
pub fn get_music_list(id: &str) -> Option<Vec<Music>> {
    let url = format!("http://music.163.com/api/playlist/detail?id={}", id);
    println!("url:{:?}", url);
    let client = reqwest::blocking::Client::new();
    let mut retry_times = 1;
    let res = loop {
        let tmpres = match client.get(&url).send() {
            Ok(res) => {
                let res = res.text().unwrap(); // res.text()会消耗res,后续再调用将为空
                info!("get music list success, url is: {}, res is:{:?}", url, &res);
                res
            }
            Err(e) => {
                error!("get music list failed, url is :{}, error is :{:?}", url, e);
                if retry_times >= 3 {
                    return None;
                } else {
                    retry_times += 1;
                    continue;
                }
            }
        };
        break tmpres;
    };
    let music163: Music163 = match serde_json::from_str(&res) {
        Ok(v) => v,
        Err(e) => {
            error!("json decode failed :{:?}", e);
            return None;
        }
    };

    let mut musics = vec![];
    if music163.code != 200 {
        return None;
    }

    for track in music163.result?.tracks? {
        let artists = track.artists?;
        let artists = artists
            .iter()
            .map(|item| item.name.to_owned())
            .collect::<Vec<String>>()
            .join(",");

        let music = Music {
            name: track.name,
            artist: artists,
            url: format!("https://music.163.com/song/media/outer/url?id={}", track.id),
            cover: track.album?.pic_url,
        };
        musics.push(music);
    }

    Some(musics)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_muisc_list() {
        match super::get_music_list("5047601141") {
            Some(res) => println!("{:?}", res),
            None => println!("none"),
        }
    }
}
