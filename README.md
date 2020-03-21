# pinvault

**A personal, multi-media, distributed and social web archiving tool**

Easily download, organize and share every video, image, article, audio file and website you care about.
Think pinterest, mastodon, the internet archive and bittorrent mashed up in a single app.

## Vision
- **Content Extraction**: Pinvault makes it as easy as possible to save *only* the stuff you care about. Want to archive a youtube video? Using [youtube-dl](https://ytdl-org.github.io/youtube-dl/index.html), pinvault saves a simple video file. When archiving a page pinvault doesn't know how to process, you can choose the things it should extract with a few clicks.
- **Distribution**: Everything pinvault downloads is saved in [IPFS](https://ipfs.io/). If you choose to publish a download's IPFS address, others can access the content even if the original source website is offline.
- **Follow anything, anywhere**: Using a few clicks, you can show pinvault how to monitor a webpage for new items and automatically download them, creating your own permanent, universal newsfeed.
- **Discover**: You can (optionally) set up pinvault on a server to follow other pinvault users and share your own activity!

## Roadmap
The vision outlined above is, at the moment, just a vision. Listed below are the features already implemented, as well as the things that are to come next.

### 0.1
- [ ] simple webextension / bookmarklet
- [ ] youtube-dl integration
- [ ] IPFS integration

### 0.2
- [ ] Content tagging
- [ ] use a headless browser to connect to pages
- [ ] simple image extraction heuristic
- [ ] element Picker for downloads

### future
- "Readability"-like article normalization
- Discover and monitor RSS feeds
- Element Picker for monitoring changes
- Automatic content backup on other devices
- Multi-user installations
- ActivityPub integration

## Similar Projects
- [ArchiveBox](https://github.com/pirate/ArchiveBox)
- [InterPlanetary Wayback](https://github.com/oduwsdl/ipwb)

## License
TBD
