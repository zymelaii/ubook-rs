use super::*;

use async_trait::async_trait;

#[async_trait]
impl crate::api::SearchAPI for BoluobaoHost {
    async fn search<F>(
        &mut self,
        keyword: &str,
        limit: usize,
        work_type: Option<WorkType>,
        mut callback: F,
    ) -> crate::Result<usize>
    where
        F: FnMut(Vec<WorkSearchResult>) -> bool + Send,
    {
        const MAX_SIZE_PER_PAGE: usize = 16;

        let host = self.as_guest();
        let keyword = keyword.to_owned();

        let limit = if limit == 0 { usize::MAX } else { limit };
        let size = limit.min(MAX_SIZE_PER_PAGE);
        let mut total = 0;

        let route = match work_type {
            Some(WorkType::Novel) => Some("novels"),
            Some(WorkType::Comic) => Some("comics"),
            Some(WorkType::Audiobook) => Some("albums"),
            None => Some("any"),
            _ => None,
        };

        if let Some(route) = route {
            for page in 0.. {
                let resp = host
                    .api_get(format!("/search/{route}/result").as_str())
                    .query(&[
                        ("q", keyword.as_str()),
                        ("page", page.to_string().as_str()),
                        ("size", size.to_string().as_str()),
                        ("expand", "intro,systags,tags,typename,authorname"),
                    ])
                    .send()
                    .await?;

                let data =
                    process_response::<types::SearchResult>(resp.status(), &resp.text().await?)?
                        .expect("missing expected field");
                let mut works: Vec<WorkSearchResult> = Default::default();
                works.extend(data.novels.into_iter().map(WorkSearchResult::from));
                works.extend(data.comics.into_iter().map(WorkSearchResult::from));
                works.extend(data.albums.into_iter().map(WorkSearchResult::from));

                let count = works.len().min(limit - total);
                total += count;
                let done = if count > 0 {
                    !callback(works.drain(..count).collect())
                } else {
                    true
                };
                if done {
                    break;
                }
            }
        }

        Ok(total)
    }
}
