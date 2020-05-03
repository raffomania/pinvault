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
- [x] youtube-dl integration
- [x] IPFS integration
- [ ] simple web UI for browsing downloads

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
- reddit integration

## Similar Projects
- [ArchiveBox](https://github.com/pirate/ArchiveBox)
- [InterPlanetary Wayback](https://github.com/oduwsdl/ipwb)

## Development Setup

Install dependencies:

- ipfs
- rust
- diesel cli: `cargo install diesel_cli`

`cp .env.example .env` and edit it to your liking.
Create the database using `diesel migration run`.
Start the ipfs daemon. Pinvault assumes the default IPFS ports for communicating with IPFS.

Try running the pinvault CLI: `cargo run`. You should see a summary of pinvault's available commands.
To access the web UI, use `cargo run server`.

## License
AGPL3+. See LICENSE file for details.

Copyright 2020 Rafael Epplée

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
