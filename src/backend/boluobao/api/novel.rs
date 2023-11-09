use super::*;
use crate::RequestBuilder;

use async_trait::async_trait;

#[async_trait]
impl crate::api::NovelAPI for BoluobaoHost {
    async fn query_novel_info(&mut self, novel_id: Id) -> crate::Result<NovelInfo> {
        #[derive(Deserialize)]
        #[allow(dead_code)]
        #[serde(rename_all = "camelCase")]
        struct ResponseData {
            need_point: usize,
            point: usize,
            rank: i32,
        }

        let resp = self
            .as_guest()
            .api_get(&format!("/novels/{novel_id}"))
            .query(&[("expand", "typeName,intro,fav,ticket,signLevel")])
            .send()
            .await?;

        let data: types::Novel =
            process_response(resp.status(), &resp.text().await?)?.expect("missing expected field");
        let mut novel_info = NovelInfo::from(data);

        let resp = self
            .api_get(&format!("/novels/{novel_id}/bonus/rank"))
            .query(&[("numMax", "1"), ("dateRange", "1")])
            .send()
            .await?;

        novel_info.browse_info.total_rewards =
            process_response::<ResponseData>(resp.status(), &resp.text().await?)?
                .expect("missing expected field")
                .point;

        // TODO: API to get the total favorites of the novel
        // novel_info.browse_info.favs = Default::default();

        let resp = self
            .as_guest()
            .api_get(&format!("/novels/{novel_id}/volumes"))
            .send()
            .await?;

        let data =
            process_response::<Vec<types::VolumeInfoV2>>(resp.status(), &resp.text().await?)?
                .expect("missing expected field");
        novel_info.volumes = data.iter().map(|e| e.volume_id).collect();

        Ok(novel_info)
    }

    async fn query_volume_info(&mut self, volume_id: Id) -> crate::Result<VolumeInfo> {
        let resp = self
            .as_guest()
            .api_get(&format!("/volumes/{volume_id}"))
            .send()
            .await?;

        let data: types::VolumeInfoV2 =
            process_response(resp.status(), &resp.text().await?)?.expect("missing expected field");
        let mut volume_info: VolumeInfo = data.into();

        let resp = self
            .as_guest()
            .api_get(&format!("/volumes/{volume_id}/chaps"))
            .send()
            .await?;

        let data: Vec<types::Chapter> =
            process_response(resp.status(), &resp.text().await?)?.expect("missing expected field");
        let data = data.iter().map(|e| e.chap_id).collect();

        // TODO: API to get the novel id of volume
        // volume_info.novel_id = Default::default();

        volume_info.chapters = data;
        Ok(volume_info)
    }

    async fn query_chapter_info(&mut self, chapter_id: Id) -> crate::Result<ChapterInfo> {
        let resp = self
            .as_guest()
            .api_get(&format!("/chaps/{chapter_id}"))
            .query(&[("expand", "needFireMoney,originNeedFireMoney")])
            .send()
            .await?;

        let data: types::Chapter =
            process_response(resp.status(), &resp.text().await?)?.expect("missing expected field");

        Ok(data.into())
    }

    async fn get_chapter_content(
        &mut self,
        chapter_id: Id,
        user_id: Option<Id>,
    ) -> crate::Result<String> {
        let host = if let Some(user_id) = user_id {
            if let Some(host) = self.as_auth(user_id) {
                host
            } else {
                self
            }
        } else {
            self.as_guest()
        };

        let resp = host
            .api_get(&format!("/chaps/{chapter_id}"))
            .query(&[("expand", "content")])
            .send()
            .await?;

        let data: types::Chapter =
            process_response(resp.status(), &resp.text().await?)?.expect("missing expected field");

        if let Some(content) = data.expand.expect("missing `expand` field").content {
            Ok(content)
        } else {
            anyhow::bail!("failed to get chapter content")
        }
    }
}
