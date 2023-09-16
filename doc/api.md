# backend - boluobao

请求地址：`https://api.sfacg.com`

关于 expand 展开项可选值的说明：

- 个人信息
  - changeNickNameInfo
  - hasActiveUnlockChapWithAd
  - hasOrderChapWithFireMoney
  - hasOrderedVipChaps
  - hasPaidFirstTime
  - hasUnlockChapWithAd
  - isRealNameAuth
  - redpacketCode
  - usedRedpacketCode
  - useWelfaresys
  - vipInfo
  - welfareCoin
  - welfareMoney
- 用户信息
  - introduction
  - bigAvatar
  - avatar
  - backgroundPic
  - fansNum
  - followNum
  - followyou
  - youfollow
  - verifyType
  - verifyInfo
  - avatarFrame
  - youblock
  - widgets
  - growup
- 小说信息
   - chapterCount
   - bigBgBanner
   - bigNovelCover
   - typeName
   - intro
   - fav
   - ticket
   - pointCount
   - tags
   - sysTags
   - signlevel
   - discount
   - discountExpireDate
   - totalNeedFireMoney
   - rankinglist
   - originTotalNeedFireMoney
   - firstchapter
   - latestchapter
   - latestcommentdate
   - essaytag
   - auditCover
   - preOrderInfo
   - customTag
   - topic
   - unauditedCustomtag
   - homeFlag
   - isbranch

> expand 通常允许递归地生效，这表示其可以作用于 expand 子项的 expand 属性展开。出于一致性考虑，在本文档中均排除多级的 expand 参数。

## 身份验证并获取会话

```plain
POST /sessions
Content-Type: application/json

{
    "username": "<phone-or-emaiil>",
    "password": "<password>"
}
```

- [D] username (字符串, 必须) 用户账户，可为绑定手机号或邮箱
- [D] password (字符串, 必须) 账户密码

## 当前用户个人信息

```plain
GET /user
```

- [P] expand (字符串，可选) 需要附加的个人信息，以逗号分隔，默认为空

> 需要登录

## 当前用户的作者信息

```plain
GET /user/authorInfo
```

> 需要登录

## 当前用户的小说作品

```plain
GET /user/novels
```

- [P] expand (字符串，可选) 需要附加的小说信息，以逗号分隔，默认为空

> 需要登录

## 当前用户的签到记录

```plain
GET /user/signInfo
```

> 需要登录

## 当前用户的收入信息

```plain
GET /user/welfare/income
```

> 需要登录

## 当前用户的小说预定信息

```plain
GET /user/preOrderInfo
```

- [P] expand (字符串, 可选) 需要展开的小说信息，以逗号分隔，默认为空

## 当前用户的小说最后浏览信息

```plain
GET /user/novelviews
```

> 需要登录

## 当前用户的特定小说的最后浏览信息

```plain
GET /user/novelviews/:novel-id
```

- [U] novel-id (整型, 必须) 需要查询的小说 ID

> 需要登录

## 当前用户的消费记录

```plain
GET /user/consumes
```

- [P] type (字符串, 可选) 消费作品类型，以逗号分隔，默认为全部
  - chaps 小说章节
  - comic 漫画
- [P] entityId (整型, 可选) 消费项所属的作品 ID，以逗号分隔，默认为全部
- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 15

> 需要登录

## 当前用户消费的作品信息

```plain
GET /user/consumeitems
```

- [P] type (字符串, 可选) 消费作品类型，以逗号分隔，默认为全部
  - novel 小说
  - comic 漫画
  - album 有声小说
- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 12

> 需要登录

## 当前用户收藏夹信息

```plain
GET /user/pockets
```

- [P] expand (字符串，可选) 需要检索的收藏类型，以逗号分隔，默认为空
  - novels 小说
  - comics 漫画
  - albums 有声小说

> 需要登录

## 当前用户余额信息

```plain
GET /user/money
```

> 需要登录

## 当前用户优惠券

```plain
GET /user/generalcoupons
```

- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量
- [P] couponTypes (整型, 可选) 优惠卷类型，以逗号分隔，默认为全部
  - 4 小说优惠券
  - 5 漫画优惠券
  - 6 有声小说优惠券
- [P] entityId (字符串, 可选) 优惠券种类，以逗号风格，默认为全部
  - -1 全部
- [P] isUsed (字符串, 可选) 是否被使用
  - not 未被使用
- [P] isExpired (字符串, 可选) 是否过期
  - not 未过期
- [P] sort (字符串, 可选) 排序方式
  - amout 折扣面额
  - recordId
- [P] order (字符串, 可选) 排序顺序
  - asc 升序
  - desc 降序

> 需要登录

## 当前用户广告推广

```plain
GET  /user/advertisements
```

- [P] deviceToken (字符串, 必须) 设备 UUID
- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 20

> 需要登录

## 商店商品列表

```plain
GET /user/welfare/storeitems
```

## 最新的商店商品

```plain
GET /user/welfare/storeitems/latest
```

## 当前用户的作者公告

```plain
GET /user/authorAnnouncements
```

- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 2

> 需要登录

## 当前用户的漫画浏览记录

```plain
GET /user/comicvisits
```

> 需要登录

## 当前用户的有声小说浏览记录

```plain
GET /user/albumvisits
```

> 需要登录

## [???]

```plain
GET /user/feeds
```

- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 12
- [P] filter (字符串, 可选)
  - followed
- [P] expand (字符串, 可选)
  - novels 小说
  - comics 漫画
  - albums 有声小说

> 需要登录

## 批量用户信息

```plain
GET /users
```

- [U] nids (整型, 必须) 需要查询的用户 ID 列表，以逗号分隔
- [P] expand (字符串, 可选) 需要展开的用户信息，以逗号分隔，默认为空

## 用户信息

```plain
GET /users/:user-id
```

- [U] user-id (整型, 必须) 需要查询的用户 ID
- [P] expand (字符串, 可选) 需要展开的用户信息，以逗号分隔，默认为空

## 用户的关注信息

```plain
GET /users/:user-id/follows
```

- [U] user-id (整型, 必须) 需要查询的用户 ID
- [P] expand (字符串, 可选) 需要展开的用户信息，以逗号分隔，默认为空
- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 20

> 需要登录

## 用户的粉丝信息

```plain
GET /users/:user-id/fans
```

- [U] user-id (整型, 必须) 需要查询的用户 ID
- [P] expand (字符串, 可选) 需要展开的用户信息，以逗号分隔，默认为空
- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 20

## 用户的收藏

```plain
/users/:user-id/pocketEntities
```

- [U] user-id (整型, 必须) 需要查询的用户 ID
- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量
- [P] expand (字符串, 可选) 需要查询的收藏类型，以逗号分隔，默认为空
    - novels 小说
    - comics 漫画
    - albums 有声小说
    - chatNovel 对话小说

## 用户的动态

```plain
GET /users/:user-id/dynamics
```

- [U] user-id (整型, 必须) 需要查询的用户 ID
- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 20
- [P] expand (字符串, 可选) 动态的所属，以逗号分隔，默认为全部
  - novels 小说
  - comics 漫画
  - albums 有声小说

## 用户的小说作品

```plain
GET /users/:user-id/novels
```

- [U] user-id (整型, 必须) 需要查询的用户 ID
- [P] expand (字符串，可选) 需要附加的小说信息，以逗号分隔，默认为空

## 当前用户的小说书签记录

```plain
GET /bookmark
```

- novelId (整型, 必须) 需要查询的小说 ID

> 需要登录

## 小说列表

```plain
GET /novels
```

- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 6（最大为 50）
- [P] tid (整型, 可选) 小说类型，默认为 -1
  - -1 全部
  - 9 爱情类
  - 21 魔幻
  - 22 玄幻
  - 23 古风
  - 24 科幻
  - 25 校园
  - 26 都市
  - 27 游戏
  - 29 悬疑
- [P] categoryId (整型, 可选) 小说分类，默认为 -1
  - -1 全部
  - 0 非同人文
  - 1 同人文（含类型为“同人文”及系统标签中含“同人”）
- [P] filter (整型, 可选)
  - recom
- [P] expand (字符串, 可选) 需要展开的小说信息，以逗号分隔，默认为空

## 特别推送的小说列表

```plain
GET /novels/specialpushs
```

- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 6
- [P] pushNames (字符串, 可选) 需要获取的特别推送列表，以逗号分隔，默认为空
  - hotpush 热门推送
- [P] expand (字符串, 可选) 需要展开的小说信息，以逗号分隔，默认为空

## 小说简评

```plain
GET /novels/:novel-id/cmts
```

- [U] novel-id (整型, 必须) 需要查询的小说 ID
- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 6
- [P] type (字符串, 可选) 简评类型，默认为空
  - stick 置顶评论、本书作者评论及高额打赏评论
  - clear 非打赏投票简评
  - stickandclear 包括 stick 与 clear
- [P] sort (字符串, 可选) 排序方式
  - timeline 按发布时间排序
  - smart 智能排序
- [P] replyUserId (整型, 可选) 默认为 0

## 小说书评

```plain
GET /novels/:novel-id/lcmts
```

- [U] novel-id (整型, 必须) 需要查询的小说 ID
- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 6
- [P] sort (字符串, 可选) 排序方式
  - addtime 按发布时间排序
- [P] charlen (整型, 可选) 书评限制字数，默认为 -1（获取全文）

> 书评截断部分将以“...”结尾

## 小说打赏排名

```plain
GET /novels/:novel-id/bonus/rank
```

- [U] novel-id (整型, 必须) 需要查询的小说 ID
- [P] numMax (整型, 必须) 参与排名的最大位次
- [P] dateRange (整型, 必须) 统计范围
  - 1 月度

## 小说月票排名

```plain
GET /novels/:novel-id/ticket/rank
```

- [U] novel-id (整型, 必须) 需要查询的小说 ID
- [P] numMax (整型, 必须) 参与排名的最大位次
- [P] dateRange (整型, 必须) 统计范围
  - 0 周度
  - 1 月度
  - 2 总计

## [???]

```plain
GET /novels/:novel-id/actpushes
```

- [U] novel-id (整型, 必须) 需要查询的小说 ID
- [P] filter (字符串, 可选)
  - android
- [P] pageType (整型, 必须)
  - 0
  - 1

## 小说的粉丝榜单

```plain
GET /novels/:novel-id/fans
```

- [U] novel-id (整型, 必须) 需要查询的小说 ID

## 特别推送信息

```plain
GET /specialpush
```

- [P] pushNames (字符串, 可选) 需要获取的推送，以逗号分隔，默认为全部
  - home 首页顶部横幅推送（含新书上架、征文活动与若干（一般为 6 个）小说推送）
  - bigBrainPush 脑洞推荐位
  - contest2Push 征文大赛作品推送
  - boluoBannerPush 菠萝包最新活动
  - bottomButton 重点征文大赛相关活动推送
  - compositionbanner 征文及相关活动推送
  - merchPush 周边推送
  - entityDetailPush 关于一些活动的细节的推送（如悬浮窗推荐）
  - entityLastPagePush 关于一些活动的细节的推送（如尾页推荐）
  - homeTopRreshenPush 有声专辑推荐
  - interactNovel 互动小说推荐
  - myBanner 个人主页横幅活动推荐
  - newSignInPush 新签约小数推荐
  - popup 上架新书力荐
  - vip 精选专区横幅推荐
  - fanNovelHomePush
  - firstChargePush
  - homeBottomPush
  - homeBottomTabPush
  - homeBroadcastPush
  - homeFloatPush
  - homeHolidayPush
  - announcementPush
  - bookMarkPush
  - chatNovelFinishGood
  - chatNovelFinishHighPoint
  - chatNovelHotDaily
  - chatNovelHotEditor
  - chatNovelHotLooking
  - chatNovelHotType
  - chatNovelHotbanners
  - chatNovelNewHot
  - chatNovelNewPotential
  - chatNovelNewWin
  - chatNovelPush
  - chatnovelbannerPush
  - comicCustomizePush

## 举报选项列表

```plain
GET /reportOptions
```

## [???] 优惠券领取

```plain
GET /entity/0/discountCoupons/qualification
```

- [P] entityType (整型, 可选) 默认为 1
- [P] businessscenario (整型, 可选) 默认为 0

> 需要登录

## 章节评论

```plain
GET /chaps/0/:chap-id/tsukkomis
```

- [U] chap-id (整型, 必须) 查询的章节 ID
- [P] expand (字符串, 可选) 附加的评论用户信息，以逗号分隔，默认为空
- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 6
- [P] sort (字符串, 可选) 排序方式（降序），默认为发布日期
  - hot 热度
- [P] row (整型, 可选) 评论的段落序号，默认为 0（标题评论）

## 段落评论

```plain
GET /cmts/:cmt-id
```

- [U] cmt-id (整型, 必须) 需要查询的评论 ID

## 段落评论回复

```plain
GET /cmts/:cmt-id/replys
```

- [U] cmt-id (整型, 必须) 需要查询的评论 ID
- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 20

## 书评

```plain
GET /lcmts/:cmt-id
```

- [U] cmt-id (整型, 必须) 需要查询的评论 ID

## 段落评论回复

```plain
GET /lcmts/:cmt-id/replys
```

- [U] cmt-id (整型, 必须) 需要查询的评论 ID
- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 20

## 小说预定信息

```plain
GET /preOrderInfo
```

- [P] withExpiredPreOrder (布尔值, 可选) 包含过期的预定信息，默认为 false
- [P] expand (字符串, 可选) 需要展开的小说信息，以逗号分隔，默认为空

## [???] 广告推广

```plain
GET /advertisements
```

- [P] page (整型, 可选) 起始页数，默认为 0
- [P] size (整型, 可选) 获取数量，默认为 20

## 小说衍生作品信息

```plain
GET /adpworks/novelId/:novel-id
```

- [U] novel-id (整型, 必须) 需要查询的小说 ID

## 漫画衍生作品信息

```plain
GET /adpworks/comicId/:comic-id
```

- [U] comic-id (整型, 必须) 需要查询的漫画 ID

## 有声小说衍生作品信息

```plain
GET /adpworks/novelId/:album-id
```

- [U] album-id (整型, 必须) 需要查询的有声小说 ID

## [???] 图片资源

```plain
GET /static/images
```

- [P] fields (字符串, 必须) 以逗号分隔
  - topIcons 顶部图标
  - bottomIcons 底部图标

## 当前 IP 属地

```plain
GET /position
```

## 国家代码列表

```plain
/countrycodes
```

## 邀请奖励广告

```plain
GET /welfare/cfg
```
