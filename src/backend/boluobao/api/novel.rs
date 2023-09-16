use super::*;

impl crate::api::NovelAPI for BoluobaoHost {
    fn query_novel_info(&mut self, novel_id: Id) -> crate::Result<NovelInfo> {
        #[derive(Deserialize)]
        #[allow(non_snake_case, dead_code)]
        struct ResponseData {
            needPoint: usize,
            point: usize,
            rank: i32,
        }

        let resp = self
            .as_guest()
            .api_get(&format!("/novels/{}", novel_id))
            .query(&[("expand", "typeName,intro,fav,ticket,signLevel")])
            .send()?;

        let data: types::Novel = process_response(resp.status(), &resp.text()?)?.unwrap();
        let mut novel_info = NovelInfo::from(data);

        let resp = self
            .api_get(&format!("/novels/{}/bonus/rank", novel_id))
            .query(&[("numMax", "1"), ("dateRange", "1")])
            .send()?;

        novel_info.browse_info.total_rewards =
            process_response::<ResponseData>(resp.status(), &resp.text()?)?
                .unwrap()
                .point;

        // TODO: API to get the total favorites of the novel
        // novel_info.browse_info.favs = Default::default();

        let resp = self
            .as_guest()
            .api_get(&format!("/novels/{}/volumes", novel_id))
            .send()?;

        let data =
            process_response::<Vec<types::VolumeInfoV2>>(resp.status(), &resp.text()?)?.unwrap();
        novel_info.volumes = data.iter().map(|e| e.volumeId).collect();

        Ok(novel_info)
    }

    fn query_volume_info(&mut self, volume_id: Id) -> crate::Result<VolumeInfo> {
        let resp = self
            .as_guest()
            .api_get(&format!("/volumes/{}", volume_id))
            .send()?;

        let data: types::VolumeInfoV2 = process_response(resp.status(), &resp.text()?)?.unwrap();
        let mut volume_info: VolumeInfo = data.into();

        let resp = self
            .as_guest()
            .api_get(&format!("/volumes/{}/chaps", volume_id))
            .send()?;

        let data: Vec<types::Chapter> = process_response(resp.status(), &resp.text()?)?.unwrap();
        let data = data.iter().map(|e| e.chapId).collect();

        // TODO: API to get the novel id of volume
        // volume_info.novel_id = Default::default();

        volume_info.chapters = data;
        Ok(volume_info)
    }

    fn query_chapter_info(&mut self, chapter_id: Id) -> crate::Result<ChapterInfo> {
        let resp = self
            .as_guest()
            .api_get(&format!("/chaps/{}", chapter_id))
            .query(&[("expand", "needFireMoney,originNeedFireMoney")])
            .send()?;

        let data: types::Chapter = process_response(resp.status(), &resp.text()?)?.unwrap();

        Ok(data.into())
    }

    fn try_get_chapter_content(
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
            .api_get(&format!("/chaps/{}", chapter_id))
            .query(&[("expand", "content")])
            .send()?;

        let data: types::Chapter = process_response(resp.status(), &resp.text()?)?.unwrap();

        if let Some(content) = data.expand.unwrap().content {
            Ok(content)
        } else {
            anyhow::bail!("failed to get chapter content")
        }
    }
}