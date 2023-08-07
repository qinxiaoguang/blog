// github helper

use serde::{Deserialize, Serialize};

use super::Result;

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
    pub pic_url: Option<String>,
}

// 通过github回调的code获取access_token
pub async fn get_music_list(id: &str) -> Result<Vec<Music>> {
    let url = format!("http://music.163.com/api/playlist/detail?id={}", id);
    println!("url:{:?}", url);
    let music163 = reqwest::get(&url).await?.json::<Music163>().await?;
    let mut musics = vec![];
    if music163.code != 200 {
        return Err(super::BizError::CommonError {
            field: format!("get invalid code:{}", music163.code),
        });
    }

    for track in music163
        .result
        .expect("invalid result")
        .tracks
        .expect("invalid tracks")
    {
        let artists = track.artists.expect("invalid artists");
        let artists = artists
            .iter()
            .map(|item| item.name.to_owned())
            .collect::<Vec<String>>()
            .join(",");

        let music = Music {
            name: track.name,
            artist: artists,
            url: format!("https://music.163.com/song/media/outer/url?id={}", track.id),
            cover: track
                .album
                .expect("invalid album")
                .pic_url
                .unwrap_or(String::default()),
        };
        musics.push(music);
    }

    Ok(musics)
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_get_muisc_list() -> std::io::Result<()> {
        match super::get_music_list("5047601141").await {
            Ok(res) => println!("{:?}", res),
            Err(e) => println!("{:?}", e),
        };
        Ok(())
    }
}
