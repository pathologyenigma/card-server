# card server
	a small open source server build for you to simulate the card game card drawing, card and card pool could be customised, could share your settings to community
	一个小型的抽卡模拟器服务端，可自定义卡牌和卡池设置以及分享你的设置到社区
## Under Developing Now
	this server is under developing right now, when it could be use, you can see there is a release version of this project(because I could only code this project when I have time, so the developing may be pretty slow)
	开发中... 可以使用时将会发布release版本（因为是抽空写的项目，所以可能开发速度不理想）
## Project Layout

 - root
	 - migrations
	 - src
		 - entity
		 - gql
		 - some other code
	- tantivy
	- build.rs
	- sqlx-data.json
the basic layout will be like this:
migrations for sqlx-cli to migrate databases([sqlx](https://github.com/launchbadge/sqlx) is the database driver I choose, the orm is the [sea-orm](https://github.com/SeaQL/sea-orm)), by the way I using postgresql for database 
entity is the auto-generated entitys by sea-orm-cli
gql for your graphql types(more details go to [async-graphql](https://github.com/async-graphql/async-graphql))
tantivy is not a folder for code, it is a data path for [tantivy](https://github.com/quickwit-inc/tantivy) text search engine(you could not see this because it is ignore by git, but you need this folder to make this project compiled)
build.rs trigger recompilation when a new migration is added
sqlx-data.json for driver settings
项目基本结构：
migrations 是sqlx命令行工具的检索路径，用于存放迁移文件的sql代码，在下使用的是sea-orm作为持久层框架（这框架使用sqlx作为数据库驱动，故而sqlx的命令行工具也可以使用），顺带一提在下使用的是postgresql
entity文件夹是用sea-orm的命令行工具自动生成（本质上的操作就是你用sqlx-cli跑迁移，然后用sea-orm-cli生成entity）
gql是graphql类型定义相关的代码（去async-graphql的github页面了解更多）
tantivy是用以存放同名的文本搜索引擎的数据的文件夹（你可能看不见这个文件夹，因为不是代码故而加入了.gitignore，但你需要该文件夹才能正常编译此项目）
build.rs是sqlx提供的一个功能，这个不是很重要（因为实际上用的是sea-orm），但还是说一下，这个是当你添加了新的迁移的时候会触发项目重新编译
sqlx-data.json是sqlx的配置文件之一（.env也可以）
## dependencies
as you can see at the project layout part, I use sqlx and sea-orm for postgresql and async-graphql for api,
let's talk about some other dependencies(in nowadays, people works for web backend may all know the api and databases things, this why I skip those).
tantivy, a text search engine like [Apache Lucene](https://lucene.apache.org/), this is used for search our card and card pool settings by keyword which is a text. because I am a chinese person, so I may also need [cang-jie](https://crates.io/crates/cang-jie) for chinese characters, not using es(elastic search) because it takes too much memory, I know there is something like es written in rust, but it just not ready for production right now(it's server and types crate is not on crate.io, wtf!) ps: this situation is at 2022-1-24(if it fix this things in the future, and I still not finish the search feature upon tantivy, I may turn to it).
http-server is warp(I choose it is not because of actix-web's unsafe bullshits, just for the reason that I used to using warp in my work before, and in that time, the actix-web could not be found in github, rocket.rs is not support async runtime), warp is not having docs but the docs.rs provided(it is kind of a little annoying, when you see a struct and it's functions not having any descriptions at all). but becasue I using graphql as the api, that the web server it self is not that matter now.
如各位在上一部分所见，数据库是postgres，使用了sqlx+sea-orm的组合（由于现在的web后端基本都是crud boy，所以这方面应该无需赘述）
直接进入正题，谈谈其他的依赖。
tantivy，文本搜索引擎，和Apache Lucene很像，在下选择该引擎作为卡池配置和卡牌设置的文本搜索功能，当然由于中文的问题需要一个额外的中文词库，这里在下选择的是cang-jie，不选择成品的es是因为java写的太吃内存了，虽然有一个基于tantivy的类似es的库，但这玩意儿作为成品来使用还太早（时间为2022-1-24，之后如果它起飞了再说，而且还得于在下研究明白tantivy之前起飞），需要git下来编译安装，crates.io上完全搜不到它的几个库，也就是说，你要作为客户端连接本地部署的这玩意儿要么就是当一个rest api来用，要么就手动复制它的代码过来通过path来在cargo中依赖（cpp直呼内行）。
服务器选择的是warp（不选actix-web不是在下有unsafe洁癖，而是在下之前在工作中用过warp，相对比较熟悉，而且那时在做选型的时候正好是unsafe节奏风暴刷的最多的时候，github上搜不到actix-web，那时的rocket.rs还不支持异步运行时，实属无奈之举），warp的文档可谓是惜字如金（有些结构体和成员函数甚至是原文放送），但是由于是用的graphql，倒是也不用太多的去关注服务器本身（只需要考虑服务器性能就行了，warp的性能还行）
国内对graphql的使用好像不多，这里稍微说一下，可能理解成一种特殊的rest服务，请求永远是以特定的方式进行的，而请求的类型分为三种query，mutation和subscription，顾名思义，就是请求，突变和关注，第一种获取数据，第二种操作数据，第三种监听事件（知道个大概就行了）
## TODOS
 - [ ] users management(用户管理)
 - [ ] card setting and card pool setting（卡牌和卡池设置）
 - [ ] history for card draw（抽卡记录）
 - [ ] algorithm for card draw（抽卡的算法，这里是指提供可以配置的抽卡算法）
 - [ ] community for sharing（社区功能，主要用于分享配置）
 - [ ] import and export card pool settings（卡池设置的导入导出）
 - [ ] card description and card pool setting sharing text search（可供搜索的卡牌描述和卡池分享）
 - [ ] statistics for card draw historys（抽卡记录统计分析）
